# Thumbsdown: Rust Rewrite

Rewrite the Ruby gem (single 152-line script) as a Rust CLI. Replace mplayer with ffmpeg/ffprobe. Replace ImageMagick with pure-Rust image processing. Result: single binary with only ffmpeg as runtime dependency.

## Dependencies

| Crate | Purpose | Replaces |
|---|---|---|
| `clap` (derive) | CLI args | Ruby `optparse` |
| `thiserror` | Error types | `puts`+`exit` |
| `serde` + `serde_json` | Parse ffprobe JSON | Regex on mplayer `ID_*` lines |
| `image` | Resize, buffers, save PNG | ImageMagick `mogrify`/`convert` |
| `imageproc` + `ab_glyph` | Header text rendering | ImageMagick `convert text:-` |
| `indicatif` | Progress bar | Ruby `tqdm` |
| `tempfile` | Auto-cleanup temp dir | Manual `/tmp` management |
| `assert_cmd` + `predicates` | Integration tests | (new) |

## File Structure

```
src/
  main.rs     ~60 lines  - entry point, orchestration pipeline
  cli.rs      ~80 lines  - clap Args struct + validate()
  error.rs    ~50 lines  - ThumbsdownError enum (thiserror)
  video.rs    ~120 lines - ffprobe metadata, ffmpeg frame capture
  grid.rs     ~150 lines - resize, border, grid composition
  header.rs   ~80 lines  - text rendering with embedded DejaVu Sans font
fonts/
  DejaVuSans.ttf          - embedded at compile time via include_bytes!
tests/
  integration.rs          - CLI integration tests (assert_cmd)
```

## Implementation Tasks

### Phase 1: Scaffold
- [x] `cargo init`, Cargo.toml with all dependencies
- [x] Download DejaVu Sans font to `fonts/`
- [x] Update `.gitignore` for Rust (`/target`)

### Phase 2: Core Modules
- [x] `error.rs` - `ThumbsdownError` enum with variants for all failure modes
- [x] `cli.rs` - clap `Args` struct preserving all original flags, `validate()` fn
- [x] `video.rs` - `check_dependencies()`, `probe()` (ffprobe JSON), `capture_frame()` (ffmpeg)
- [x] `grid.rs` - `process_thumbnail()`, `add_border()`, `compose_grid()`, `assemble_final()`
- [x] `header.rs` - `render_header()` with embedded font, 2-line text (filename + metadata)

### Phase 3: Wire Up
- [x] `main.rs` - Pipeline: parse -> validate -> probe -> capture -> grid -> header -> save

### Phase 4: Tests
- [x] Unit tests in each module (`#[cfg(test)]`) - 25 tests
- [x] Integration tests with `assert_cmd` (--help, --version, missing file, etc.) - 5 tests

### Phase 5: Cleanup
- [x] Remove Ruby files: `exe/`, `Gemfile`, `Gemfile.lock`, `Rakefile`, `thumbsdown.gemspec`
- [x] Remove `.github/workflows/publish-ghpkg.yml`
- [x] Update README.md for Rust CLI
- [x] Add CI workflow (fmt/clippy/test)
- [x] Add release workflow (cross-platform binaries)

## CLI (preserved from Ruby)

```
thumbsdown [OPTIONS] <VIDEO>

Options:
  -s, --start <N>      Start time in seconds [default: 1]
  -t, --thumbs <N>     Number of thumbnails [default: 20]
  -c, --columns <N>    Grid columns [default: 5]
  -o, --output <PATH>  Output file [default: thumbs.png]
  -T, --temp <DIR>     Temp directory [default: OS temp]
  -w, --width <N>      Thumbnail width in pixels [default: 320]
  -v, --verbose        Verbose output
  -f, --force          Overwrite existing output
  -V, --version        Version
  -h, --help           Help
```

## Pipeline

```
parse args -> validate -> check ffmpeg/ffprobe on PATH
  -> ffprobe video -> VideoInfo { duration, width, height, codec, fps }
  -> for each thumbnail:
       ffmpeg -ss T -i video -frames:v 1 frame.png
       load frame, add 10px white border, resize to --width
  -> compose_grid: chunks by columns, concat_horizontal per row, concat_vertical
  -> render_header: 2-line text (filename + codec/fps/resolution) with embedded font
  -> assemble_final: header on top of grid, white background
  -> save output PNG
  -> TempDir auto-drops (cleanup)
```

## Key Improvements Over Ruby

- **Single binary** - no Ruby, no ImageMagick, only ffmpeg needed
- **Auto temp cleanup** - `tempfile::TempDir` drops on exit/panic
- **Fast seeking** - ffmpeg `-ss` before `-i` (input seeking)
- **f64 time math** - accurate frame spacing (Ruby used integer division)
- **In-memory grid** - no intermediate disk files for composition
