use std::path::Path;

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, Rgb, RgbImage};

use crate::error::Result;

const BORDER_COLOR: Rgb<u8> = Rgb([255, 255, 255]);
const BG_COLOR: Rgb<u8> = Rgb([255, 255, 255]);

pub fn process_thumbnail(path: &Path, target_width: u32, border_size: u32) -> Result<DynamicImage> {
    let img = image::open(path)?;
    let bordered = add_border(&img, border_size);
    let resized = bordered.resize(target_width, u32::MAX, FilterType::Lanczos3);
    Ok(resized)
}

pub fn compose_grid(thumbnails: &[DynamicImage], columns: u32) -> RgbImage {
    let cols = columns as usize;
    let row_images: Vec<RgbImage> = thumbnails.chunks(cols).map(concat_horizontal).collect();

    concat_vertical(&row_images)
}

pub fn assemble_final(header: &RgbImage, grid: &RgbImage) -> RgbImage {
    let width = header.width().max(grid.width());
    let height = header.height() + grid.height();
    let mut result = RgbImage::from_pixel(width, height, BG_COLOR);

    let header_x = (width.saturating_sub(header.width())) as i64 / 2;
    image::imageops::overlay(&mut result, header, header_x, 0);

    let grid_x = (width.saturating_sub(grid.width())) as i64 / 2;
    image::imageops::overlay(&mut result, grid, grid_x, header.height() as i64);

    result
}

fn add_border(img: &DynamicImage, border: u32) -> DynamicImage {
    let (w, h) = img.dimensions();
    let new_w = w + border * 2;
    let new_h = h + border * 2;
    let mut bordered = RgbImage::from_pixel(new_w, new_h, BORDER_COLOR);
    image::imageops::overlay(&mut bordered, &img.to_rgb8(), border as i64, border as i64);
    DynamicImage::ImageRgb8(bordered)
}

fn concat_horizontal(images: &[DynamicImage]) -> RgbImage {
    let total_width: u32 = images.iter().map(|img| img.width()).sum();
    let max_height: u32 = images.iter().map(|img| img.height()).max().unwrap_or(0);
    let mut result = RgbImage::from_pixel(total_width, max_height, BG_COLOR);

    let mut x_offset: i64 = 0;
    for img in images {
        image::imageops::overlay(&mut result, &img.to_rgb8(), x_offset, 0);
        x_offset += img.width() as i64;
    }

    result
}

fn concat_vertical(images: &[RgbImage]) -> RgbImage {
    let max_width: u32 = images.iter().map(|img| img.width()).max().unwrap_or(0);
    let total_height: u32 = images.iter().map(|img| img.height()).sum();
    let mut result = RgbImage::from_pixel(max_width, total_height, BG_COLOR);

    let mut y_offset: i64 = 0;
    for img in images {
        image::imageops::overlay(&mut result, img, 0, y_offset);
        y_offset += img.height() as i64;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_image(w: u32, h: u32, color: Rgb<u8>) -> DynamicImage {
        DynamicImage::ImageRgb8(RgbImage::from_pixel(w, h, color))
    }

    #[test]
    fn add_border_increases_dimensions() {
        let img = make_test_image(10, 10, Rgb([128, 128, 128]));
        let bordered = add_border(&img, 5);
        assert_eq!(bordered.width(), 20);
        assert_eq!(bordered.height(), 20);
    }

    #[test]
    fn add_border_has_white_edges() {
        let img = make_test_image(10, 10, Rgb([0, 0, 0]));
        let bordered = add_border(&img, 5);
        let rgb = bordered.to_rgb8();
        assert_eq!(*rgb.get_pixel(0, 0), Rgb([255, 255, 255]));
        assert_eq!(*rgb.get_pixel(19, 19), Rgb([255, 255, 255]));
    }

    #[test]
    fn add_border_preserves_content() {
        let img = make_test_image(4, 4, Rgb([100, 100, 100]));
        let bordered = add_border(&img, 2);
        let rgb = bordered.to_rgb8();
        assert_eq!(*rgb.get_pixel(2, 2), Rgb([100, 100, 100]));
        assert_eq!(*rgb.get_pixel(5, 5), Rgb([100, 100, 100]));
    }

    #[test]
    fn concat_horizontal_joins_images() {
        let images = vec![
            make_test_image(10, 20, Rgb([0, 0, 0])),
            make_test_image(15, 20, Rgb([128, 128, 128])),
        ];
        let result = concat_horizontal(&images);
        assert_eq!(result.width(), 25);
        assert_eq!(result.height(), 20);
    }

    #[test]
    fn concat_vertical_joins_images() {
        let a = RgbImage::from_pixel(10, 5, Rgb([0, 0, 0]));
        let b = RgbImage::from_pixel(10, 8, Rgb([128, 128, 128]));
        let result = concat_vertical(&[a, b]);
        assert_eq!(result.width(), 10);
        assert_eq!(result.height(), 13);
    }

    #[test]
    fn compose_grid_layout_3x2() {
        let thumbs: Vec<DynamicImage> = (0..6)
            .map(|_| make_test_image(10, 10, Rgb([50, 50, 50])))
            .collect();
        let result = compose_grid(&thumbs, 3);
        assert_eq!(result.width(), 30);
        assert_eq!(result.height(), 20);
    }

    #[test]
    fn compose_grid_incomplete_last_row() {
        let thumbs: Vec<DynamicImage> = (0..5)
            .map(|_| make_test_image(10, 10, Rgb([50, 50, 50])))
            .collect();
        let result = compose_grid(&thumbs, 3);
        assert_eq!(result.width(), 30);
        assert_eq!(result.height(), 20);
    }

    #[test]
    fn assemble_final_centers_header() {
        let header = RgbImage::from_pixel(20, 5, Rgb([0, 0, 0]));
        let grid = RgbImage::from_pixel(40, 10, Rgb([128, 128, 128]));
        let result = assemble_final(&header, &grid);
        assert_eq!(result.width(), 40);
        assert_eq!(result.height(), 15);
    }
}
