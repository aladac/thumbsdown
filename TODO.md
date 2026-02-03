# Thumbsdown - Update for 2025

Video thumbnail grid generator - needs modernization from mplayer to ffmpeg/mpv.

## Current State

- **Language**: Ruby gem
- **Dependencies**: mplayer (deprecated), ImageMagick
- **Last updated**: 2018

## Problem

mplayer is legacy/unmaintained. Modern alternatives:
1. **ffmpeg** - Universal, widely available, best choice
2. **mpv** - Modern mplayer fork, similar syntax
3. **libav/avconv** - Less common now

## Current mplayer Usage

### Metadata extraction
```bash
mplayer -nolirc -frames 1 -nosound -vo null -identify "video.mp4"
# Outputs: ID_LENGTH, ID_VIDEO_FORMAT, ID_VIDEO_FPS, ID_VIDEO_WIDTH, ID_VIDEO_HEIGHT
```

### Frame capture
```bash
mplayer -nolirc -osdlevel 2 -vo png -nosound -frames 1 -vf expand -ss <seconds> "video.mp4"
```

## Replacement Commands

### ffmpeg (recommended)
```bash
# Metadata
ffprobe -v quiet -print_format json -show_format -show_streams "video.mp4"

# Frame capture at specific time
ffmpeg -ss <seconds> -i "video.mp4" -frames:v 1 -q:v 2 output.png
```

### mpv
```bash
# Metadata
mpv --vo=null --ao=null --frames=1 --term-playing-msg='LENGTH=${duration}
WIDTH=${width}
HEIGHT=${height}' "video.mp4"

# Frame capture
mpv --vo=image --start=<seconds> --frames=1 "video.mp4"
```

## Tasks

### Phase 1: Backend Switch
- [ ] Replace mplayer metadata extraction with ffprobe
- [ ] Replace mplayer frame capture with ffmpeg
- [ ] Update dependency check (`which ffmpeg ffprobe convert mogrify`)
- [ ] Parse ffprobe JSON output instead of mplayer ID_ lines

### Phase 2: Modernize Ruby
- [ ] Update to modern Ruby (3.x compatible)
- [ ] Replace `tqdm` with native progress or `ruby-progressbar`
- [ ] Update bundler/rake dependencies
- [ ] Add proper error handling
- [ ] Use shellwords for safe command building

### Phase 3: Features
- [ ] Add `--backend` flag to choose ffmpeg/mpv
- [ ] Add timestamp overlay on thumbnails
- [ ] Add video info header (duration, codec, resolution)
- [ ] Support output formats: PNG, JPEG, WebP
- [ ] Add `--quality` option for output compression

### Phase 4: Distribution
- [ ] Update gemspec for modern RubyGems
- [ ] Add GitHub Actions CI
- [ ] Publish to RubyGems.org
- [ ] Add Homebrew formula

## Code Changes Required

### exe/thumbsdown

```ruby
# OLD
used_binaries = %w[convert mogrify mplayer]
identify_cmd = 'mplayer -nolirc -frames 1 -nosound -vo null -identify'
thumb_cmd = 'mplayer -nolirc -osdlevel 2 -vo png -nosound -frames 1'

# NEW
used_binaries = %w[convert mogrify ffmpeg ffprobe]

def get_video_info(file)
  json = `ffprobe -v quiet -print_format json -show_format -show_streams "#{file}"`
  data = JSON.parse(json)
  video = data['streams'].find { |s| s['codec_type'] == 'video' }
  {
    duration: data['format']['duration'].to_f,
    width: video['width'],
    height: video['height'],
    codec: video['codec_name'],
    fps: eval(video['r_frame_rate']).to_f.round(2)
  }
end

def capture_frame(file, time, output)
  `ffmpeg -y -ss #{time} -i "#{file}" -frames:v 1 -q:v 2 "#{output}" 2>/dev/null`
end
```

## Alternative: Rewrite in Rust

Consider rewriting as a Rust CLI for:
- Single binary distribution (no Ruby/gem needed)
- Better performance for large batches
- Use `ffmpeg` crate or shell out to ffmpeg
