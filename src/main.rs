use russsty::engine::{Engine, RenderMode};
use russsty::window::{FrameLimiter, Key, Window, WindowEvent, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> Result<(), String> {
    let mut window = Window::new("Russsty", WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let mut engine = Engine::new(window.width(), window.height());

    engine
        .load_mesh("assets/cube.obj")
        .map_err(|e| e.to_string())?;

    let mut frame_limiter = FrameLimiter::new(&window);

    loop {
        match window.poll_events() {
            WindowEvent::Quit => break,
            WindowEvent::Resize(w, h) => {
                window.resize(w, h)?;
                engine.resize(w, h);
            }
            WindowEvent::KeyPress(key) => {
                let mode = match key {
                    Key::Num1 => RenderMode::Wireframe,
                    Key::Num2 => RenderMode::WireframeVertices,
                    Key::Num3 => RenderMode::FilledWireframe,
                    Key::Num4 => RenderMode::FilledWireframeVertices,
                    Key::Num5 => RenderMode::Filled,
                };
                engine.set_render_mode(mode);
            }
            WindowEvent::None => {}
        }

        let _delta_time = frame_limiter.wait_and_get_delta(&window);

        // Rotate the mesh
        let rotation = engine.mesh_mut().rotation_mut();
        rotation.x += 0.01;
        rotation.y += 0.01;
        rotation.z += 0.01;

        engine.update();
        engine.render();
        window.present(engine.frame_buffer())?;
    }

    Ok(())
}
