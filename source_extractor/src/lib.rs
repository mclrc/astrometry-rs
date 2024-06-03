mod bitmatrix;

use anyhow::Result;
use bitmatrix::BitMatrix;
use image::Rgb;
use image::{GrayImage, ImageBuffer, Luma};
use imageproc::filter::median_filter;
use itertools::{iproduct, Itertools};
use serde::Serialize;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct DetectedObject {
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    center_x: f64,
    center_y: f64,
}

impl DetectedObject {
    pub fn write_as_string(&self, mut f: impl Write) -> Result<()> {
        writeln!(
            f,
            "{} {} {} {} {} {}",
            self.x, self.y, self.width, self.height, self.center_x, self.center_y
        )?;

        Ok(())
    }
}

fn load_grayscale_image(path: &Path) -> Result<GrayImage> {
    let img = image::open(path)?;

    Ok(img.to_luma8())
}

fn median_smooth(img: &GrayImage, radius: u32) -> GrayImage {
    let smoothed_img = median_filter(img, radius, radius);

    ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
        let Luma([p1]) = img.get_pixel(x, y);
        let Luma([p2]) = smoothed_img.get_pixel(x, y);

        Luma([p1.saturating_sub(*p2)])
    })
}

const NOISE_SAMPLE_RADIUS: u32 = 5;

fn calculate_noise(img: &GrayImage) -> f64 {
    let approx_samples = (img.width() * img.height()) as f64 / (NOISE_SAMPLE_RADIUS.pow(2)) as f64;

    let mut flux_diffs = Vec::with_capacity(approx_samples as usize);

    for (x, y) in iproduct!(
        (0..img.width()).step_by(NOISE_SAMPLE_RADIUS as usize * 2),
        (0..img.height()).step_by(NOISE_SAMPLE_RADIUS as usize * 2)
    ) {
        let center_flux = img.get_pixel(x, y)[0] as i32;

        for (dx, dy) in iproduct!(-1..=1, -1..=1) {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx * NOISE_SAMPLE_RADIUS as i32;
            let ny = y as i32 + dy * NOISE_SAMPLE_RADIUS as i32;
            if nx < 0 || nx >= img.width() as i32 || ny < 0 || ny >= img.height() as i32 {
                continue;
            }

            let flux = img.get_pixel(nx as u32, ny as u32)[0] as i32;

            flux_diffs.push(flux - center_flux);
        }
    }

    let mean = flux_diffs.iter().sum::<i32>() as f64 / flux_diffs.len() as f64;

    let variance = flux_diffs
        .iter()
        .map(|&x| (x as f64 - mean).powi(2))
        .sum::<f64>()
        / flux_diffs.len() as f64;

    variance
}

fn find_object(
    img: &GrayImage,
    x: i32,
    y: i32,
    threshold: f64,
    visited: &mut BitMatrix,
) -> Option<DetectedObject> {
    let mut pixels = Vec::new();
    let mut queue = Vec::new();

    queue.push((x, y));

    while let Some((x, y)) = queue.pop() {
        visited.set(x as usize, y as usize, true);

        if (img.get_pixel(x as u32, y as u32)[0] as f64) < threshold {
            continue;
        }

        pixels.push((x, y));

        for (dx, dy) in iproduct!(-1..=1, -1..=1) {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x + dx;
            let ny = y + dy;

            let out_of_bounds =
                nx < 0 || nx >= img.width() as i32 || ny < 0 || ny >= img.height() as i32;

            if out_of_bounds || visited.get(nx as usize, ny as usize) {
                continue;
            }

            queue.push((nx, ny));
        }
    }

    if pixels.is_empty() {
        return None;
    }

    let (min_x, max_x) = pixels
        .iter()
        .map(|(x, _)| *x)
        .minmax()
        .into_option()
        .unwrap();

    let (min_y, max_y) = pixels
        .iter()
        .map(|(_, y)| *y)
        .minmax()
        .into_option()
        .unwrap();

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let mut grid = (0..height).map(|_| vec![0; width]).collect::<Vec<_>>();

    for (x, y) in pixels {
        let rel_x = (x - min_x) as usize;
        let rel_y = (y - min_y) as usize;
        grid[rel_y][rel_x] = img.get_pixel(x as u32, y as u32)[0];
    }

    let (center_offset_x, center_offset_y) = find_center(&grid);

    Some(DetectedObject {
        x: min_x,
        y: min_y,
        width,
        height,
        center_x: min_x as f64 + center_offset_x,
        center_y: min_y as f64 + center_offset_y,
    })
}

fn find_objects(img: &GrayImage, threshold: f64) -> Vec<DetectedObject> {
    let mut visited = BitMatrix::new(img.width() as usize, img.height() as usize);
    let mut objects = Vec::new();

    for (x, y) in iproduct!(0..img.width() as usize, 0..img.height() as usize) {
        if visited.get(x, y) {
            continue;
        }

        visited.set(x, y, true);

        let flux = img.get_pixel(x as u32, y as u32)[0] as f64;

        if flux < threshold {
            continue;
        }

        if let Some(object) = find_object(img, x as i32, y as i32, threshold, &mut visited) {
            objects.push(object);
        }
    }

    objects
}

#[allow(dead_code)]
fn is_peak(pixels: &[Vec<u8>], x: usize, y: usize) -> bool {
    let center = pixels[y][x] as f64;

    print!("{} ", center);

    for (dx, dy) in iproduct!(-1..=1, -1..=1) {
        if dx == 0 && dy == 0 {
            continue;
        }
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || nx >= pixels[0].len() as i32 || ny < 0 || ny >= pixels.len() as i32 {
            continue;
        }
        if pixels[ny as usize][nx as usize] as f64 >= center {
            return false;
        }
    }
    true
}

// TODO
fn find_center(pixels: &[Vec<u8>]) -> (f64, f64) {
    (pixels[0].len() as f64 / 2.0, pixels.len() as f64 / 2.0)
}

pub fn extract_sources(image_path: &Path) -> Result<Vec<DetectedObject>> {
    let img = load_grayscale_image(image_path)?;
    let smoothed_img = median_smooth(&img, 100);
    let noise = calculate_noise(&smoothed_img);
    let objects = find_objects(&smoothed_img, 8.0 * noise.sqrt());

    Ok(objects)
}

pub fn draw_objects(
    image_path: &Path,
    objects: &[DetectedObject],
) -> Result<ImageBuffer<Rgb<u16>, Vec<u16>>> {
    let mut colored_img = image::open(image_path)?.to_rgb16();
    for object in objects {
        for x in object.x..object.x + object.width as i32 {
            colored_img.put_pixel(x as u32, object.y as u32, Rgb([0, u16::MAX, 0]));
            colored_img.put_pixel(
                x as u32,
                (object.y + object.height as i32 - 1) as u32,
                Rgb([0, u16::MAX, 0]),
            );
        }
        for y in object.y..object.y + object.height as i32 {
            colored_img.put_pixel(object.x as u32, y as u32, Rgb([0, u16::MAX, 0]));
            colored_img.put_pixel(
                (object.x + object.width as i32 - 1) as u32,
                y as u32,
                Rgb([0, u16::MAX, 0]),
            );
        }
        colored_img.put_pixel(
            object.center_x as u32,
            object.center_y as u32,
            Rgb([u16::MAX, 0, 0]),
        );
    }

    Ok(colored_img)
}
