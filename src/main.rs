// src/main.rs
// Main entry point for the ray tracer application.

use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use rayon::prelude::*;
use image::{DynamicImage, GenericImageView};
use nalgebra::Vector3;

use crate::bvh::{BVH, BVHNode};
use crate::scene::{Scene, Object};
use crate::ray::{Ray, Intersection};
use crate::material::{Material, Lambertian};
use crate::image::{Image, ImageBuffer};

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <input_scene> <output_image> <num_threads>", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid usage"));
    }

    let input_scene = args[1].clone();
    let output_image = args[2].clone();
    let num_threads = args[3].parse::<usize>().unwrap();

    // Load input scene
    let scene_file = File::open(input_scene)?;
    let scene = Scene::load(scene_file)?;

    // Create BVH acceleration structure
    let bvh = BVH::new(&scene.objects, num_threads)?;

    // Render image
    let image = render_image(&scene, &bvh, num_threads)?;

    // Save output image
    let image_file = File::create(output_image)?;
    image.save(image_file)?;

    Ok(())
}

fn render_image(scene: &Scene, bvh: &BVH, num_threads: usize) -> Result<Image, io::Error> {
    let image_width = scene.width;
    let image_height = scene.height;
    let image_buffer = ImageBuffer::new(image_width, image_height);

    // Render image in parallel using Rayon
    let rays = scene.rays().into_par_iter();
    let mut image_buffer = image_buffer.clone();
    for ray in rays {
        let intersection = bvh.intersect(ray);
        if let Some(intersection) = intersection {
            let mut image = image_buffer.clone();
            material::shade(&intersection, &scene.materials, &mut image);
        }
    }

    Ok(image_buffer)
}