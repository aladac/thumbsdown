use std::path::Path;
use std::process::Command;

use serde::Deserialize;

use crate::error::{Result, ThumbsdownError};

#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub filename: String,
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub codec: String,
    pub fps: f64,
}

#[derive(Deserialize)]
struct FfprobeOutput {
    streams: Vec<FfprobeStream>,
    format: FfprobeFormat,
}

#[derive(Deserialize)]
struct FfprobeFormat {
    filename: Option<String>,
    duration: Option<String>,
}

#[derive(Deserialize)]
struct FfprobeStream {
    codec_type: Option<String>,
    codec_name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    r_frame_rate: Option<String>,
    duration: Option<String>,
}

pub fn check_dependencies() -> Result<()> {
    if Command::new("ffprobe")
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_err()
    {
        return Err(ThumbsdownError::FfprobeNotFound);
    }

    if Command::new("ffmpeg")
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_err()
    {
        return Err(ThumbsdownError::FfmpegNotFound);
    }

    Ok(())
}

pub fn probe(path: &Path) -> Result<VideoInfo> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
        ])
        .arg(path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ThumbsdownError::FfprobeFailed {
            path: path.to_path_buf(),
            reason: stderr.into_owned(),
        });
    }

    let data: FfprobeOutput = serde_json::from_slice(&output.stdout)?;

    let video_stream = data
        .streams
        .iter()
        .find(|s| s.codec_type.as_deref() == Some("video"))
        .ok_or_else(|| ThumbsdownError::NoVideoStream(path.to_path_buf()))?;

    let duration = data
        .format
        .duration
        .as_deref()
        .and_then(|d| d.parse::<f64>().ok())
        .or_else(|| {
            video_stream
                .duration
                .as_deref()
                .and_then(|d| d.parse::<f64>().ok())
        })
        .unwrap_or(0.0);

    let filename = data
        .format
        .filename
        .as_deref()
        .map(|f| {
            Path::new(f)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| f.to_string())
        })
        .unwrap_or_else(|| "unknown".to_string());

    let fps = video_stream
        .r_frame_rate
        .as_deref()
        .map(parse_frame_rate)
        .unwrap_or(0.0);

    Ok(VideoInfo {
        filename,
        duration,
        width: video_stream.width.unwrap_or(0),
        height: video_stream.height.unwrap_or(0),
        codec: video_stream
            .codec_name
            .clone()
            .unwrap_or_else(|| "unknown".to_string()),
        fps,
    })
}

pub fn capture_frame(video_path: &Path, time_secs: f64, output_path: &Path) -> Result<()> {
    let status = Command::new("ffmpeg")
        .args(["-y", "-ss"])
        .arg(format!("{time_secs:.3}"))
        .arg("-i")
        .arg(video_path)
        .args(["-frames:v", "1", "-q:v", "2"])
        .arg(output_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;

    if !status.success() {
        return Err(ThumbsdownError::FrameCaptureFailed {
            time: time_secs,
            reason: format!("ffmpeg exited with {status}"),
        });
    }

    Ok(())
}

fn parse_frame_rate(rate: &str) -> f64 {
    if let Some((num, den)) = rate.split_once('/') {
        let n: f64 = num.parse().unwrap_or(0.0);
        let d: f64 = den.parse().unwrap_or(1.0);
        if d == 0.0 {
            return 0.0;
        }
        (n / d * 100.0).round() / 100.0
    } else {
        rate.parse().unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_frame_rate_fraction() {
        assert!((parse_frame_rate("30000/1001") - 29.97).abs() < 0.01);
    }

    #[test]
    fn parse_frame_rate_simple_fraction() {
        assert!((parse_frame_rate("25/1") - 25.0).abs() < 0.01);
    }

    #[test]
    fn parse_frame_rate_zero_denominator() {
        assert!((parse_frame_rate("30/0") - 0.0).abs() < 0.01);
    }

    #[test]
    fn parse_frame_rate_plain_number() {
        assert!((parse_frame_rate("24") - 24.0).abs() < 0.01);
    }

    #[test]
    fn parse_frame_rate_garbage() {
        assert!((parse_frame_rate("abc") - 0.0).abs() < 0.01);
    }

    #[test]
    fn probe_nonexistent_file_returns_error() {
        let result = probe(Path::new("/nonexistent_video_xyz.mp4"));
        assert!(result.is_err());
    }

    #[test]
    fn ffprobe_json_parsing() {
        let json = r#"{
            "streams": [{
                "codec_type": "video",
                "codec_name": "h264",
                "width": 1920,
                "height": 1080,
                "r_frame_rate": "30000/1001",
                "duration": "120.5"
            }],
            "format": {
                "filename": "/path/to/video.mp4",
                "duration": "120.5"
            }
        }"#;

        let data: FfprobeOutput = serde_json::from_str(json).expect("parse");
        let stream = data
            .streams
            .iter()
            .find(|s| s.codec_type.as_deref() == Some("video"))
            .expect("video stream");

        assert_eq!(stream.width, Some(1920));
        assert_eq!(stream.height, Some(1080));
        assert_eq!(stream.codec_name.as_deref(), Some("h264"));

        let dur: f64 = data
            .format
            .duration
            .as_deref()
            .and_then(|d| d.parse().ok())
            .unwrap_or(0.0);
        assert!((dur - 120.5).abs() < 0.01);
    }
}
