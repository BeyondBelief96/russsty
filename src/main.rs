mod engine;
mod window;

use engine::{Engine, COLOR_BACKGROUND, COLOR_GRID, COLOR_MAGENTA};
use window::{Window, WindowEvent, WINDOW_WIDTH, WINDOW_HEIGHT};

fn update() {
    // Update game logic here
}

fn render(engine: &mut Engine) {
    engine.clear_color_buffer(COLOR_BACKGROUND);
    engine.draw_grid(50, COLOR_GRID);
    engine.draw_rect(300, 200, 300, 150, COLOR_MAGENTA);
}

fn main() -> Result<(), String> {
    let mut window = Window::new("Russsty", WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let mut engine = Engine::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    loop {
        match window.poll_events() {
            WindowEvent::Quit => break,
            WindowEvent::Resize(w, h) => {
                window.resize(w, h)?;
                engine = Engine::new(w, h);
            }
            WindowEvent::None => {}
        }

        update();
        render(&mut engine);
        window.present(engine.get_buffer_as_bytes())?;
    }

    Ok(())
}
