# Thumbsdown

Generate thumbnail grids from video files.

![example](https://i.imgur.com/n0d5tNZ.png)

## Requirements

- [ffmpeg](https://ffmpeg.org/) (includes `ffprobe`)

## Installation

### From source

```bash
cargo install --path .
```

### From GitHub releases

Download the binary for your platform from [Releases](https://github.com/aladac/thumbsdown/releases).

## Usage

```
thumbsdown [OPTIONS] <VIDEO>

Arguments:
  <VIDEO>  Path to the video file

Options:
  -s, --start <START>      Start time in seconds [default: 1]
  -t, --thumbs <THUMBS>    Number of thumbnails to generate [default: 20]
  -c, --columns <COLUMNS>  Number of columns in the grid [default: 5]
  -o, --output <OUTPUT>    Output file path [default: thumbs.png]
  -T, --temp <TEMP>        Temporary directory (default: system temp)
  -w, --width <WIDTH>      Thumbnail width in pixels [default: 320]
  -v, --verbose            Enable verbose output
  -f, --force              Overwrite existing output file
  -h, --help               Print help
  -V, --version            Print version
```

### Examples

```bash
# Basic usage - 20 thumbnails in a 5-column grid
thumbsdown video.mp4

# 12 thumbnails in a 4-column grid, custom output
thumbsdown -t 12 -c 4 -o preview.png video.mp4

# Verbose mode, overwrite existing output
thumbsdown -v -f -t 8 -c 4 video.mp4
```

## License

Available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
