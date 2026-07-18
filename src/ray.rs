//! Ray tracing and intersection calculations

use nalgebra::Vector3;
use crate::scene::{Object, Scene};

/// Represents a ray in 3D space.
///
/// A ray is defined by its origin and direction.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    /// Creates a new ray from the given origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin`: The starting point of the ray.
    /// * `direction`: The direction of the ray.
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }

    /// Calculates the point where the ray intersects with an object.
    ///
    /// # Arguments
    ///
    /// * `object`: The object to intersect with.
    /// * `scene`: The scene containing the object.
    ///
    /// # Returns
    ///
    /// The intersection point, or None if no intersection is found.
    pub fn intersect(&self, object: &Object, scene: &Scene) -> Option<(f64, Vector3<f64>)> {
        if let Some(intersection) = object.intersect(self, &scene.transform) {
            Some(intersection)
        } else if scene.intersect_bvh(self) {
            Some((self.origin + self.direction, self.direction))
        } else {
            None
        }
    }
}

/// Calculates the distance from the ray origin to the ray's intersection with an object.
///
/// # Arguments
///
/// * `ray`: The ray to check.
/// * `object`: The object to intersect with.
/// * `scene`: The scene containing the object.
///
/// # Returns
///
/// The distance to the intersection, or f64::MAX if no intersection is found.
pub fn distance_to_intersection(ray: &Ray, object: &Object, scene: &Scene) -> f64 {
    if let Some(intersection) = object.intersect(ray, &scene.transform) {
        intersection.0.distance(ray.origin)
    } else if scene.intersect_bvh(ray) {
        f64::MAX
    } else {
        f64::MAX
    }
}