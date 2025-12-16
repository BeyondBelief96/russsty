use std::time::Instant;

use russsty::engine::{Engine, RasterizerType, RenderMode};
use russsty::window::{FrameLimiter, Key, Window, WindowEvent, WINDOW_HEIGHT, WINDOW_WIDTH};

struct FpsCounter {
    frame_count: u32,
    last_update: Instant,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frame_count: 0,
            last_update: Instant::now(),
        }
    }

    /// Call each frame. Returns Some(fps) once per second, None otherwise.
    fn tick(&mut self) -> Option<f64> {
        self.frame_count += 1;
        let elapsed = self.last_update.elapsed();
        if elapsed.as_secs() >= 1 {
            let fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_update = Instant::now();
            Some(fps)
        } else {
            None
        }
    }
}

fn format_window_title(fps: f64, engine: &Engine) -> String {
    format!(
        "Russsty | FPS: {:.1} | {} | Cull: {} | {:?}",
        fps,
        engine.rasterizer(),
        if engine.backface_culling { "ON" } else { "OFF" },
        engine.render_mode()
    )
}

fn main() -> Result<(), String> {
    let mut window = Window::new("Russsty", WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let mut engine = Engine::new(window.width(), window.height());

    engine
        .load_mesh("assets/cube.obj")
        .map_err(|e| e.to_string())?;

    let mut frame_limiter = FrameLimiter::new(&window);
    let mut fps_counter = FpsCounter::new();

    loop {
        match window.poll_events() {
            WindowEvent::Quit => break,
            WindowEvent::Resize(w, h) => {
                window.resize(w, h)?;
                engine.resize(w, h);
            }
            WindowEvent::KeyPress(key) => match key {
                Key::Num1 => engine.set_render_mode(RenderMode::Wireframe),
                Key::Num2 => engine.set_render_mode(RenderMode::WireframeVertices),
                Key::Num3 => engine.set_render_mode(RenderMode::FilledWireframe),
                Key::Num4 => engine.set_render_mode(RenderMode::FilledWireframeVertices),
                Key::Num5 => engine.set_render_mode(RenderMode::Filled),
                Key::C => engine.backface_culling = !engine.backface_culling,
                Key::G => engine.draw_grid = !engine.draw_grid,
                Key::R => {
                    let next = match engine.rasterizer() {
                        RasterizerType::Scanline => RasterizerType::EdgeFunction,
                        RasterizerType::EdgeFunction => RasterizerType::Scanline,
                    };
                    engine.set_rasterizer(next);
                }
            },
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

        if let Some(fps) = fps_counter.tick() {
            window.set_title(&format_window_title(fps, &engine));
        }
    }

    Ok(())
}
