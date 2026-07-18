// src/bvh.rs

/// A bounding volume hierarchy (BVH) is a tree-like data structure used for
/// accelerating ray tracing by efficiently pruning the scene graph.
///
/// The BVH is constructed by recursively splitting the scene into two child
/// nodes, each containing a subset of the scene's primitives. The splitting
/// process is based on the bounding volumes of the primitives, and the
/// resulting tree is traversed by the ray tracer to determine which
/// primitives intersect with the current ray.
pub struct Bvh {
    /// The root node of the BVH.
    pub root: Node,
}

impl Bvh {
    /// Creates a new BVH from a given scene.
    ///
    /// # Arguments
    ///
    /// * `scene`: The scene to be accelerated by the BVH.
    pub fn new(scene: &Scene) -> Self {
        Self {
            root: Node::new(scene),
        }
    }

    /// Traverses the BVH to find the first primitive that intersects with
    /// the given ray.
    ///
    /// # Arguments
    ///
    /// * `ray`: The ray to be intersected with the BVH.
    ///
    /// # Returns
    ///
    /// The first primitive that intersects with the ray, or `None` if no
    /// intersection is found.
    pub fn intersect(&self, ray: &Ray) -> Option<&Primitive> {
        self.root.intersect(ray)
    }
}

/// A node in the BVH tree.
pub struct Node {
    /// The bounding box of the node.
    pub bbox: BBox,
    /// The child nodes of the node.
    pub children: [Option<Box<Node>>; 2],
    /// The primitives contained in the node.
    pub primitives: Vec<Primitive>,
}

impl Node {
    /// Creates a new node with the given bounding box and primitives.
    ///
    /// # Arguments
    ///
    /// * `bbox`: The bounding box of the node.
    /// * `primitives`: The primitives contained in the node.
    pub fn new(bbox: BBox, primitives: Vec<Primitive>) -> Self {
        Self {
            bbox,
            children: [None, None],
            primitives,
        }
    }

    /// Traverses the node to find the first primitive that intersects with
    /// the given ray.
    ///
    /// # Arguments
    ///
    /// * `ray`: The ray to be intersected with the node.
    ///
    /// # Returns
    ///
    /// The first primitive that intersects with the ray, or `None` if no
    /// intersection is found.
    fn intersect(&self, ray: &Ray) -> Option<&Primitive> {
        // Check if the ray intersects with the node's bounding box.
        if !self.bbox.intersects(ray) {
            return None;
        }

        // Check if the node contains any intersecting primitives.
        for primitive in &self.primitives {
            if let Some(intersection) = primitive.intersect(ray) {
                return Some(primitive);
            }
        }

        // If no intersecting primitives are found, traverse the child nodes.
        for child in &self.children {
            if let Some(child_node) = child {
                if let Some(intersection) = child_node.intersect(ray) {
                    return Some(intersection);
                }
            }
        }

        // If no intersection is found, return None.
        None
    }
}

/// A bounding box.
pub struct BBox {
    /// The minimum and maximum coordinates of the box.
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl BBox {
    /// Checks if the ray intersects with the box.
    ///
    /// # Arguments
    ///
    /// * `ray`: The ray to be intersected with the box.
    ///
    /// # Returns
    ///
    /// `true` if the ray intersects with the box, `false` otherwise.
    pub fn intersects(&self, ray: &Ray) -> bool {
        // Check if the ray's origin is inside the box.
        let t_min = (self.min[0] - ray.origin[0]) / ray.direction[0];
        let t_max = (self.max[0] - ray.origin[0]) / ray.direction[0];
        if t_min > t_max {
            return false;
        }

        // Check if the ray intersects with the box's edges.
        for i in 0..3 {
            if ray.direction[i] == 0.0 {
                return self.min[i] <= ray.origin[i] && ray.origin[i] <= self.max[i];
            }

            let t1 = (self.min[i] - ray.origin[i]) / ray.direction[i];
            let t2 = (self.max[i] - ray.origin[i]) / ray.direction[i];
            if t1 > t2 {
                return false;
            }

            if t_min > t1 && t_max < t2 {
                return false;
            }

            t_min = t_min.max(t1);
            t_max = t_max.min(t2);
        }

        true
    }
}

/// A scene representation.
pub struct Scene {
    /// The primitives in the scene.
    pub primitives: Vec<Primitive>,
}

/// A primitive in the scene.
pub trait Primitive {
    /// Checks if the ray intersects with the primitive.
    ///
    /// # Arguments
    ///
    /// * `ray`: The ray to be intersected with the primitive.
    ///
    /// # Returns
    ///
    /// The intersection point of the ray and the primitive, or `None` if no
    /// intersection is found.
    fn intersect(&self, ray: &Ray) -> Option<(f64, f64, f64)>;
}

/// A ray.
pub struct Ray {
    /// The origin of the ray.
    pub origin: [f64; 3],
    /// The direction of the ray.
    pub direction: [f64; 3],
}