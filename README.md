# raytracer-bvh

> Accelerating ray tracing with high-performance bounding volume hierarchies.

## Overview

raytracer-bvh is a high-performance ray tracing library for Rust, utilizing bounding volume hierarchies (BVH) to accelerate complex rendering tasks. It provides a robust and efficient framework for rendering 3D scenes, making it an ideal choice for applications requiring fast and accurate rendering. By leveraging the power of BVH, raytracer-bvh significantly reduces the computational overhead of ray tracing, enabling real-time rendering of complex scenes.

## Features

* **BVH Construction**: Efficiently builds a hierarchical structure of bounding volumes to accelerate ray tracing.
* **Ray Tracing**: Accurately intersects rays with 3D objects, taking into account material properties and lighting effects.
* **Material Shading**: Calculates and applies material properties to rendered objects, including diffuse, specular, and metallic reflections.
* **Image Rendering**: Stores and manages rendered images, supporting various image formats and resolutions.
* **Scene Management**: Represents and manages 3D scenes, including objects, lights, and cameras.
* **High-Performance**: Optimized for maximum performance, utilizing efficient data structures and algorithms.
* **Flexible Configuration**: Allows for easy customization and extension of rendering settings and parameters.

## Getting Started

### Prerequisites

* Rust 1.63.0 or later
* Cargo 1.63.0 or later

### Installation

```bash
git clone https://github.com/your-username/raytracer-bvh.git
cd raytracer-bvh
cargo build
```

### Usage

```bash
cargo run --example simple-scene
```

This will render a simple 3D scene with a sphere and a plane. You can customize the rendering settings and parameters by modifying the `src/main.rs` file.

## Architecture

The project structure is organized into the following key modules:

* `src/bvh.rs`: Implements the bounding volume hierarchy data structure and algorithms.
* `src/scene.rs`: Represents and manages 3D scenes, including objects, lights, and cameras.
* `src/ray.rs`: Calculates and intersects rays with 3D objects, taking into account material properties and lighting effects.
* `src/material.rs`: Calculates and applies material properties to rendered objects.
* `src/image.rs`: Stores and manages rendered images.
* `tests/main.rs`: Contains unit tests for the ray tracer.

## API Reference

The public interfaces of the library are documented in the `src/` directory. You can browse the API documentation using Cargo's built-in documentation tool:

```bash
cargo doc --open
```

## Testing

```bash
cargo test
```

This will run the unit tests for the ray tracer.

## Contributing

1. Fork the repository.
2. Create a feature branch.
3. Commit changes.
4. Push and open a PR.

## License

MIT License

Copyright (c) [Year] [Author]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.