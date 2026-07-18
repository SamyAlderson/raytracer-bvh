// src/image.rs

//! Module for image rendering and storage.

use nalgebra::{Vector3, Point3};
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufWriter, Write};
use image::{DynamicImage, GenericImageView, RgbaImage};

use crate::material::Material;
use crate::scene::Scene;
use crate::ray::Ray;
use crate::bvh::BVH;

/// Image rendering and storage.
pub struct Image {
    /// Width of the image in pixels.
    pub width: u32,
    /// Height of the image in pixels.
    pub height: u32,
    /// Image data.
    pub data: RgbaImage,
}

impl Image {
    /// Create a new image with the specified width and height.
    pub fn new(width: u32, height: u32) -> Self {
        let data = RgbaImage::new(width, height);
        Image { width, height, data }
    }

    /// Render the scene into the image.
    pub fn render(&mut self, scene: &Scene, bvh: &BVH, ray: &Ray, samples: u32) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut pixel_color = Vector3::zeros();
                for _ in 0..samples {
                    let u = (x as f64 + rand::random::<f64>()) / (self.width as f64 - 1.0);
                    let v = (y as f64 + rand::random::<f64>()) / (self.height as f64 - 1.0);
                    let ray_origin = ray.origin + (u * (ray.direction - ray.origin)).normalize();
                    let intersection = bvh.intersect(ray_origin, ray.direction);
                    if let Some(intersection) = intersection {
                        let material = scene.get_material(intersection);
                        if let Some(material) = material {
                            let (u, v) = material.get_texture_coords(intersection);
                            pixel_color += material.get_color(u, v);
                        }
                    }
                }
                let pixel_color = pixel_color / samples as f64;
                let pixel_color = (pixel_color * 255.0).ceil() as u8;
                self.data.put_pixel(x, y, image::Rgba([pixel_color, pixel_color, pixel_color, 255]));
            }
        }
    }

    /// Save the image to a file.
    pub fn save(&self, path: &PathBuf) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.data.save(&mut writer, image::ImageFormat::Png)?;
        Ok(())
    }
}

fn rand() -> f64 {
    rand::thread_rng().gen::<f64>()
}