use libdragger;

extern crate piston_window;

mod assets;

use assets::Assets;
use piston_window::*;

use libdragger::*;

const SCALE_X: f64 = 0.5;
const SCALE_Y: f64 = 0.5;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let assets = Assets::create(&mut window);

    let mut mouse_state = MouseState::new(false, 0.0, 0.0);

    let mut game = libdragger::Game::create();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c: Context, g, _| {
            clear([1.0; 4], g);
            for r in game.get_renders() {
                if let libdragger::RenderType::Sprite(tex) = &r.render_type {
                    let tex = assets.get(&tex).unwrap();
                    let trans = c.transform.trans(r.x, r.y).scale(SCALE_X, SCALE_Y);
                    image(tex, trans, g);
                }
            }
        });
        if let Some(button) = e.press_args() {
            if let Button::Mouse(MouseButton::Left) = button {
                mouse_state.down = true;
            }
        }
        if let Some(button) = e.release_args() {
            if let Button::Mouse(MouseButton::Left) = button {
                mouse_state.down = false;
            }
        }
        e.mouse_cursor(|pos| {
            mouse_state.x = pos[0];
            mouse_state.y = pos[1];
        });
        if let Some(_) = e.update_args() {
            game.update(mouse_state);
        }
    }
}
