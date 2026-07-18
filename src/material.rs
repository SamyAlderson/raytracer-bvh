//! Material properties and shading module.

/// Represents a material with its properties and shading behavior.
pub trait Material {
    /// Returns the material's color at a given point.
    fn color(&self, point: &NalgebraVector3<f64>) -> NalgebraVector3<f64>;

    /// Returns the material's diffuse color at a given point.
    fn diffuse_color(&self, point: &NalgebraVector3<f64>) -> NalgebraVector3<f64>;

    /// Returns the material's specular color at a given point.
    fn specular_color(&self, point: &NalgebraVector3<f64>) -> NalgebraVector3<f64>;

    /// Returns the material's reflectivity at a given point.
    fn reflectivity(&self, point: &NalgebraVector3<f64>) -> f64;

    /// Returns the material's refraction index at a given point.
    fn refraction_index(&self, point: &NalgebraVector3<f64>) -> f64;
}

/// A Lambertian material with a uniform diffuse color.
pub struct Lambertian {
    color: NalgebraVector3<f64>,
}

impl Lambertian {
    /// Creates a new Lambertian material with the given color.
    pub fn new(color: NalgebraVector3<f64>) -> Self {
        Lambertian { color }
    }
}

impl Material for Lambertian {
    fn color(&self, _point: &NalgebraVector3<f64>) -> NalgebraVector3<f64> {
        self.color
    }

    fn diffuse_color(&self, _point: &NalgebraVector3<f64>) -> NalgebraVector3<f64> {
        self.color
    }

    fn specular_color(&self, _point: &NalgebraVector3<f64>) -> NalgebraVector3<f64> {
        NalgebraVector3::zeros()
    }

    fn reflectivity(&self, _point: &NalgebraVector3<f64>) -> f64 {
        0.0
    }

    fn refraction_index(&self, _point: &NalgebraVector3<f64>) -> f64 {
        1.0
    }
}

/// A perfect specular material.
pub struct PerfectSpecular {
    color: NalgebraVector3<f64>,
}

impl PerfectSpecular {
    /// Creates a new perfect specular material with the given color.
    pub fn new(color: NalgebraVector3<f64>) -> Self {
        PerfectSpecular { color }
    }
}

impl Material for PerfectSpecular {
    fn color(&self, _point: &NalgebraVector3<f64>) -> NalgebraVector3<f64> {
        NalgebraVector3::zeros()
    }

    fn diffuse_color(&self, _point: &NalgebraVector3<f64>) -> NalgebraVector3<f64> {
        NalgebraVector3::zeros()
    }

    fn specular_color(&self, _point: &NalgebraVector3<f64>) -> NalgebraVector3<f64> {
        self.color
    }

    fn reflectivity(&self, _point: &NalgebraVector3<f64>) -> f64 {
        1.0
    }

    fn refraction_index(&self, _point: &NalgebraVector3<f64>) -> f64 {
        1.0
    }
}