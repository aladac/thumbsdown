use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThumbsdownError {
    #[error("input video file does not exist: {0}")]
    InputNotFound(PathBuf),

    #[error("output file already exists: {0} (use -f to overwrite)")]
    OutputExists(PathBuf),

    #[error("temp directory does not exist: {0}")]
    TempDirNotFound(PathBuf),

    #[error("ffprobe not found on PATH (install ffmpeg)")]
    FfprobeNotFound,

    #[error("ffmpeg not found on PATH (install ffmpeg)")]
    FfmpegNotFound,

    #[error("ffprobe failed for {path}: {reason}")]
    FfprobeFailed { path: PathBuf, reason: String },

    #[error("no video stream found in {0}")]
    NoVideoStream(PathBuf),

    #[error("ffmpeg frame capture failed at {time}s: {reason}")]
    FrameCaptureFailed { time: f64, reason: String },

    #[error("image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("font loading error: {0}")]
    FontError(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, ThumbsdownError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_messages_are_descriptive() {
        let err = ThumbsdownError::InputNotFound(PathBuf::from("video.mp4"));
        assert!(err.to_string().contains("video.mp4"));
        assert!(err.to_string().contains("does not exist"));
    }

    #[test]
    fn output_exists_suggests_force_flag() {
        let err = ThumbsdownError::OutputExists(PathBuf::from("out.png"));
        assert!(err.to_string().contains("-f"));
    }

    #[test]
    fn ffprobe_not_found_suggests_install() {
        let err = ThumbsdownError::FfprobeNotFound;
        assert!(err.to_string().contains("install ffmpeg"));
    }
}
