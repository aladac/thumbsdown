use std::path::PathBuf;

use clap::Parser;

use crate::error::{Result, ThumbsdownError};

/// Generate thumbnail grids from video files
#[derive(Parser, Debug)]
#[command(name = "thumbsdown", version, about)]
pub struct Args {
    /// Path to the video file
    pub video: PathBuf,

    /// Start time in seconds
    #[arg(short = 's', long, default_value_t = 1)]
    pub start: u64,

    /// Number of thumbnails to generate
    #[arg(short = 't', long, default_value_t = 20)]
    pub thumbs: u32,

    /// Number of columns in the grid
    #[arg(short = 'c', long, default_value_t = 5)]
    pub columns: u32,

    /// Output file path
    #[arg(short = 'o', long, default_value = "thumbs.png")]
    pub output: PathBuf,

    /// Temporary directory (default: system temp)
    #[arg(short = 'T', long)]
    pub temp: Option<PathBuf>,

    /// Thumbnail width in pixels
    #[arg(short = 'w', long, default_value_t = 320)]
    pub width: u32,

    /// Enable verbose output
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Overwrite existing output file
    #[arg(short = 'f', long)]
    pub force: bool,
}

pub fn validate(args: &Args) -> Result<()> {
    if !args.video.exists() {
        return Err(ThumbsdownError::InputNotFound(args.video.clone()));
    }

    if args.output.exists() && !args.force {
        return Err(ThumbsdownError::OutputExists(args.output.clone()));
    }

    if let Some(ref temp) = args.temp {
        if !temp.is_dir() {
            return Err(ThumbsdownError::TempDirNotFound(temp.clone()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rejects_missing_video() {
        let args = Args {
            video: PathBuf::from("nonexistent_video.mp4"),
            start: 1,
            thumbs: 20,
            columns: 5,
            output: PathBuf::from("out.png"),
            temp: None,
            width: 320,
            verbose: false,
            force: false,
        };
        let err = validate(&args).unwrap_err();
        assert!(err.to_string().contains("does not exist"));
    }

    #[test]
    fn validate_rejects_existing_output_without_force() {
        let dir = tempfile::tempdir().expect("tempdir");
        let video = dir.path().join("video.mp4");
        std::fs::write(&video, b"fake").expect("write");
        let output = dir.path().join("existing.png");
        std::fs::write(&output, b"fake").expect("write");

        let args = Args {
            video,
            start: 1,
            thumbs: 20,
            columns: 5,
            output,
            temp: None,
            width: 320,
            verbose: false,
            force: false,
        };
        let err = validate(&args).unwrap_err();
        assert!(err.to_string().contains("already exists"));
    }

    #[test]
    fn validate_allows_force_overwrite() {
        let dir = tempfile::tempdir().expect("tempdir");
        let video = dir.path().join("video.mp4");
        std::fs::write(&video, b"fake").expect("write");
        let output = dir.path().join("existing.png");
        std::fs::write(&output, b"fake").expect("write");

        let args = Args {
            video,
            start: 1,
            thumbs: 20,
            columns: 5,
            output,
            temp: None,
            width: 320,
            verbose: false,
            force: true,
        };
        assert!(validate(&args).is_ok());
    }

    #[test]
    fn validate_rejects_nonexistent_temp_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let video = dir.path().join("video.mp4");
        std::fs::write(&video, b"fake").expect("write");

        let args = Args {
            video,
            start: 1,
            thumbs: 20,
            columns: 5,
            output: PathBuf::from("out.png"),
            temp: Some(PathBuf::from("/nonexistent_dir_xyz")),
            width: 320,
            verbose: false,
            force: false,
        };
        let err = validate(&args).unwrap_err();
        assert!(err.to_string().contains("does not exist"));
    }
}
