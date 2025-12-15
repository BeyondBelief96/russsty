use russsty::engine::Engine;
use russsty::math::{vec2::Vec2, vec3::Vec3};
use russsty::renderer::{COLOR_BACKGROUND, COLOR_GREEN, COLOR_GRID, COLOR_MAGENTA};
use russsty::triangle::Triangle;
use russsty::window::{FrameLimiter, Window, WindowEvent, WINDOW_HEIGHT, WINDOW_WIDTH};

const FOV_FACTOR: f32 = 640.0;

// Project a 3D point to a 2D point using perspective division
fn project(point: &Vec3) -> Option<Vec2> {
    // Clip points that are behind or too close to the camera
    if point.z < 0.1 {
        return None;
    }

    Some(Vec2::new(
        FOV_FACTOR * point.x / point.z,
        FOV_FACTOR * point.y / point.z,
    ))
}

fn update(camera_position: Vec3, engine: &mut Engine) {
    let faces = engine.mesh().faces().to_vec();
    let vertices = engine.mesh().vertices().to_vec();
    let rotation = engine.mesh().rotation();
    let buffer_width = engine.renderer().width();
    let buffer_height = engine.renderer().height();
    let backface_culling = engine.backface_culling;

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

            // Camera ray points from vertex to camera. If dot product is negative, triangle is facing away from camera
            let camera_ray = camera_position - transformed_vertices[0];
            if normal.dot(camera_ray) < 0.0 {
                continue;
            };
        }

        let mut projected_points = Vec::new();
        for transformed_vertex in transformed_vertices.iter() {
            if let Some(mut projected) = project(&transformed_vertex) {
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

    engine.set_triangles_to_render(triangles);
}

fn render(engine: &mut Engine) {
    let triangles: Vec<Triangle> = engine.get_triangles_to_render().to_vec();

    let renderer = engine.renderer_mut();
    renderer.clear(COLOR_BACKGROUND);
    renderer.draw_grid(50, COLOR_GRID);

    for triangle in triangles.iter() {
        for vertex in triangle.points.iter() {
            renderer.draw_rect(vertex.x as i32, vertex.y as i32, 4, 4, COLOR_MAGENTA);
        }
        renderer.draw_triangle(triangle);
    }
}

fn main() -> Result<(), String> {
    let mut window = Window::new("Russsty", WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let mut engine = Engine::new(window.width(), window.height());
    engine
        .load_mesh("assets/f22.obj")
        .map_err(|e| e.to_string())?;

    let camera_position = Vec3::new(0.0, 0.0, -5.0);

    let mut frame_limiter = FrameLimiter::new(&window);
    loop {
        match window.poll_events() {
            WindowEvent::Quit => break,
            WindowEvent::Resize(w, h) => {
                window.resize(w, h)?;
                engine.resize(w, h);
            }
            WindowEvent::None => {}
        }

        // Get delta time (in milliseconds) after frame limiting
        let delta_time = frame_limiter.wait_and_get_delta(&window);

        // Only run update/render after enough time has passed for this frame.
        engine.mesh_mut().rotation_mut().y += 0.01;
        engine.mesh_mut().rotation_mut().z += 0.01;
        engine.mesh_mut().rotation_mut().x += 0.01;

        engine.clear_triangles_to_render();
        update(camera_position, &mut engine);
        render(&mut engine);
        window.present(engine.renderer().as_bytes())?;
    }

    Ok(())
}
