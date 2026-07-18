// tests/main.rs

// Import necessary dependencies
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    // Test the scene generation
    #[test]
    fn test_scene_generation() {
        // Set up a test scene
        let scene = Scene::new();

        // Add some test objects to the scene
        scene.add_object(Object::new_cube());
        scene.add_object(Object::new_sphere());
        scene.add_object(Object::new_plane());

        // Test that the scene has the expected number of objects
        assert_eq!(scene.objects.len(), 3);
    }

    // Test the ray intersection algorithm
    #[test]
    fn test_ray_intersection() {
        // Set up a test ray
        let ray = Ray::new_point(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Test that the ray intersects the expected objects
        let objects = Object::new_cube().intersect(ray);
        assert!(objects.len() > 0);

        let plane = Object::new_plane();
        let intersections = plane.intersect(ray);
        assert!(intersections.is_empty());
    }

    // Test the BVH acceleration structure
    #[test]
    fn test_bvh_acceleration() {
        // Set up a test scene with a BVH acceleration structure
        let scene = Scene::new();
        scene.add_object(Object::new_cube());
        scene.add_object(Object::new_sphere());
        scene.build_bvh();

        // Test that the BVH acceleration structure correctly intersects the test ray
        let ray = Ray::new_point(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let objects = scene.bvh.intersect(ray);
        assert!(objects.len() > 0);
    }

    // Test the image rendering
    #[test]
    fn test_image_rendering() {
        // Set up a test scene and image
        let scene = Scene::new();
        let image = Image::new(512, 512);

        // Render the test scene to the image
        scene.render_to_image(&mut image);

        // Test that the image has the expected dimensions and pixel count
        assert_eq!(image.width, 512);
        assert_eq!(image.height, 512);
        assert_eq!(image.pixels.len(), image.width * image.height);
    }
}