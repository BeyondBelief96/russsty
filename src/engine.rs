use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::mesh::{LoadError, Mesh, CUBE_FACES, CUBE_VERTICES};
use crate::renderer::{Renderer, COLOR_BACKGROUND, COLOR_GREEN, COLOR_GRID, COLOR_MAGENTA};
use crate::triangle::Triangle;

const DEFAULT_FOV_FACTOR: f32 = 640.0;

pub struct Engine {
    renderer: Renderer,
    triangles_to_render: Vec<Triangle>,
    mesh: Mesh,
    camera_position: Vec3,
    fov_factor: f32,
    pub backface_culling: bool,
    pub draw_grid: bool,
    pub draw_vertices: bool,
    pub draw_wireframe: bool,
}

impl Engine {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            renderer: Renderer::new(width, height),
            triangles_to_render: Vec::new(),
            mesh: Mesh::new(vec![], vec![], Vec3::ZERO),
            camera_position: Vec3::new(0.0, 0.0, -5.0),
            fov_factor: DEFAULT_FOV_FACTOR,
            backface_culling: true,
            draw_grid: true,
            draw_vertices: true,
            draw_wireframe: true,
        }
    }

    pub fn load_cube_mesh(&mut self) {
        self.mesh = Mesh::new(CUBE_VERTICES.to_vec(), CUBE_FACES.to_vec(), Vec3::ZERO);
    }

    pub fn load_mesh(&mut self, file_path: &str) -> Result<(), LoadError> {
        self.mesh = Mesh::from_obj(file_path)?;
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    pub fn set_camera_position(&mut self, position: Vec3) {
        self.camera_position = position;
    }

    pub fn camera_position(&self) -> Vec3 {
        self.camera_position
    }

    pub fn set_fov_factor(&mut self, fov: f32) {
        self.fov_factor = fov;
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    /// Returns the rendered frame as bytes (ARGB8888 format)
    pub fn frame_buffer(&self) -> &[u8] {
        self.renderer.as_bytes()
    }

    /// Project a 3D point to 2D screen coordinates
    fn project(&self, point: Vec3) -> Option<Vec2> {
        // Clip points that are behind or too close to the camera
        if point.z < 0.1 {
            return None;
        }

        Some(Vec2::new(
            self.fov_factor * point.x / point.z,
            self.fov_factor * point.y / point.z,
        ))
    }

    /// Update the engine state - transforms vertices and builds triangles to render
    pub fn update(&mut self) {
        let faces = self.mesh.faces().to_vec();
        let vertices = self.mesh.vertices().to_vec();
        let rotation = self.mesh.rotation();
        let buffer_width = self.renderer.width();
        let buffer_height = self.renderer.height();
        let camera_position = self.camera_position;
        let backface_culling = self.backface_culling;

        let mut triangles = Vec::new();

        for face in faces.iter() {
            let face_vertices = [
                vertices[face.a as usize - 1],
                vertices[face.b as usize - 1],
                vertices[face.c as usize - 1],
            ];

            let mut transformed_vertices = Vec::new();
            for vertex in face_vertices.iter() {
                let mut transformed_vertex = *vertex;
                transformed_vertex = transformed_vertex.rotate_x(rotation.x);
                transformed_vertex = transformed_vertex.rotate_y(rotation.y);
                transformed_vertex = transformed_vertex.rotate_z(rotation.z);
                transformed_vertex.z -= camera_position.z;
                transformed_vertices.push(transformed_vertex);
            }

            // Apply backface culling
            if backface_culling {
                let vec_ba = transformed_vertices[1] - transformed_vertices[0];
                let vec_ca = transformed_vertices[2] - transformed_vertices[0];
                let normal = vec_ba.cross(vec_ca);
                let camera_ray = camera_position - transformed_vertices[0];
                if normal.dot(camera_ray) < 0.0 {
                    continue;
                }
            }

            let mut projected_points = Vec::new();
            for transformed_vertex in transformed_vertices.iter() {
                if let Some(mut projected) = self.project(*transformed_vertex) {
                    // Adjust triangle points to be centered on screen
                    projected.x += buffer_width as f32 / 2.0;
                    projected.y += buffer_height as f32 / 2.0;
                    projected_points.push(projected);
                }
            }

            // Only create triangle if all three vertices were successfully projected
            if projected_points.len() == 3 {
                triangles.push(Triangle {
                    points: projected_points,
                    color: COLOR_GREEN,
                });
            }
        }

        self.triangles_to_render = triangles;
    }

    /// Render the current frame
    pub fn render(&mut self) {
        self.renderer.clear(COLOR_BACKGROUND);

        if self.draw_grid {
            self.renderer.draw_grid(50, COLOR_GRID);
        }

        for triangle in self.triangles_to_render.iter() {
            if self.draw_vertices {
                for vertex in triangle.points.iter() {
                    self.renderer.draw_rect(vertex.x as i32, vertex.y as i32, 4, 4, COLOR_MAGENTA);
                }
            }
            if self.draw_wireframe {
                self.renderer.draw_triangle(triangle);
            }
        }
    }
}
