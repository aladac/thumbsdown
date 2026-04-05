# Thumbsdown

Generate thumbnail grids from video files.

![example](https://i.imgur.com/n0d5tNZ.png)

## Requirements

- [ffmpeg](https://ffmpeg.org/) (includes `ffprobe`)

## Installation

### Homebrew (macOS / Linux)

```bash
brew install saiden-dev/tap/thumbsdown
```

### Quick install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/aladac/thumbsdown/master/install.sh | bash
```

### From crates.io

```bash
cargo install thumbsdown
```

### From source

```bash
cargo install --path .
```

### From GitHub releases

Download the binary for your platform from [Releases](https://github.com/aladac/thumbsdown/releases).

| Platform | Binary |
|----------|--------|
| Linux x86_64 | `thumbsdown-linux-amd64` |
| Linux ARM64 | `thumbsdown-linux-arm64` |
| macOS ARM64 | `thumbsdown-macos-arm64` |

## Usage

```
thumbsdown [OPTIONS] <VIDEO>

Arguments:
  <VIDEO>  Path to the video file

Options:
  -s, --start <START>              Start time in seconds [default: 1]
  -t, --thumbs <THUMBS>            Number of thumbnails to generate [default: 20]
  -c, --columns <COLUMNS>          Number of columns in the grid [default: 5]
  -o, --output <OUTPUT>            Output file path [default: thumbs.png]
  -T, --temp <TEMP>                Temporary directory (default: system temp)
  -w, --width <WIDTH>              Thumbnail width in pixels [default: 320]
  -v, --verbose                    Enable verbose output
  -f, --force                      Overwrite existing output file
  -k, --keep-frames <KEEP_FRAMES>  Keep extracted frames in specified directory
      --no-grid                    Skip grid generation (only extract frames)
  -h, --help                       Print help
  -V, --version                    Print version
```

### Examples

```bash
# Basic usage - 20 thumbnails in a 5-column grid
thumbsdown video.mp4

# 12 thumbnails in a 4-column grid, custom output
thumbsdown -t 12 -c 4 -o preview.png video.mp4

# Verbose mode, overwrite existing output
thumbsdown -v -f -t 8 -c 4 video.mp4

# Extract frames only (no grid)
thumbsdown --no-grid -k ./frames video.mp4

# Generate grid and keep individual frames
thumbsdown -k ./frames video.mp4
```

## License

Available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
