use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};

use crate::error::{Result, ThumbsdownError};
use crate::video::VideoInfo;

const FONT_DATA: &[u8] = include_bytes!("../fonts/DejaVuSans.ttf");
const FONT_SIZE: f32 = 18.0;
const LINE_SPACING: i32 = 4;
const PADDING: i32 = 8;
const TEXT_COLOR: Rgb<u8> = Rgb([0, 0, 0]);
const BG_COLOR: Rgb<u8> = Rgb([255, 255, 255]);

pub fn render_header(info: &VideoInfo) -> Result<RgbImage> {
    let font = FontRef::try_from_slice(FONT_DATA)
        .map_err(|e| ThumbsdownError::FontError(e.to_string()))?;
    let scale = PxScale::from(FONT_SIZE);

    let line1 = &info.filename;
    let line2 = format!(
        "vcodec: {}, fps: {:.2}, resolution: {}x{}",
        info.codec, info.fps, info.width, info.height
    );
    let lines = [line1.as_str(), line2.as_str()];

    let measurements: Vec<(u32, u32)> = lines
        .iter()
        .map(|line| text_size(scale, &font, line))
        .collect();

    let max_width = measurements.iter().map(|(w, _)| *w).max().unwrap_or(0);
    let total_text_height: u32 = measurements.iter().map(|(_, h)| *h).sum();
    let total_spacing = LINE_SPACING * (lines.len() as i32 - 1);

    let img_width = max_width + (PADDING as u32 * 2);
    let img_height = total_text_height + total_spacing as u32 + (PADDING as u32 * 2);

    let mut image = RgbImage::from_pixel(img_width, img_height, BG_COLOR);

    let mut y = PADDING;
    for (i, line) in lines.iter().enumerate() {
        draw_text_mut(&mut image, TEXT_COLOR, PADDING, y, scale, &font, line);
        y += measurements[i].1 as i32 + LINE_SPACING;
    }

    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_info() -> VideoInfo {
        VideoInfo {
            filename: "test_video.mp4".to_string(),
            duration: 120.5,
            width: 1920,
            height: 1080,
            codec: "h264".to_string(),
            fps: 29.97,
        }
    }

    #[test]
    fn render_header_produces_image() {
        let img = render_header(&test_info()).expect("render");
        assert!(img.width() > 0);
        assert!(img.height() > 0);
    }

    #[test]
    fn render_header_has_reasonable_height() {
        let img = render_header(&test_info()).expect("render");
        // Two lines of 18px text + spacing + padding should be > 40px
        assert!(img.height() > 40);
        assert!(img.height() < 200);
    }

    #[test]
    fn render_header_contains_non_white_pixels() {
        let img = render_header(&test_info()).expect("render");
        let has_text = img.pixels().any(|p| *p != Rgb([255, 255, 255]));
        assert!(has_text, "header should contain drawn text");
    }
}
