use crate::math::vec3::Vec3;
use crate::mesh::{LoadError, Mesh, CUBE_FACES, CUBE_VERTICES};
use crate::renderer::Renderer;
use crate::triangle::Triangle;

pub struct Engine {
    renderer: Renderer,
    triangles_to_render: Vec<Triangle>,
    mesh: Mesh,
    pub backface_culling: bool,
}

impl Engine {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            renderer: Renderer::new(width, height),
            triangles_to_render: Vec::new(),
            mesh: Mesh::new(vec![], vec![], Vec3::ZERO),
            backface_culling: true,
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

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn set_triangles_to_render(&mut self, triangles: Vec<Triangle>) {
        self.triangles_to_render = triangles;
    }

    pub fn get_triangles_to_render(&self) -> &[Triangle] {
        &self.triangles_to_render
    }

    pub fn get_triangles_to_render_mut(&mut self) -> &mut Vec<Triangle> {
        &mut self.triangles_to_render
    }

    pub fn clear_triangles_to_render(&mut self) {
        self.triangles_to_render.clear();
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
