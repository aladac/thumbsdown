# Thumbsdown

Thumbnail generator for video files

![example](https://i.imgur.com/n0d5tNZ.png)

## Installation

    $ gem install thumbsdown

## Usage

```
Usage: thumbsdown [OPTIONS]

Create a multi-thumbnail image of a video file

    -h, --help                       show this message
    -v, --[no-]verbose=[FLAG]        run verbosly
    -o, --output=STRING              Output filename (default out.(png|jpeg))
    -c, --columns=INT                Number of thumbnail columns (default: 5)
    -s, --start=INT                  Start point for making thumbnails in seconds (default: 1)
    -T, --temp=STRING                Specify a temporary directory (default /tmp)
    -t, --thumbs=INT                 Number of thumbs to generate - the length of the video is divided equally by this (default: 20)
    -w, --width=INT                  Width of a single thumbnail - Height calculated automaticly (default: 320px)
    -f, --[no-]force=[FLAG]          force mode - override output

```

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
