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
    game.set_aspect_ratio(window.size().width / window.size().height);

    while let Some(e) = window.next() {
        let size = window.size();
        window.draw_2d(&e, |c: Context, g, _| {
            clear([1.0; 4], g);
            for r in &mut game {
                if let libdragger::RenderType::Sprite(tex) = &r.render_type {
                    let tex = assets.get(&tex).unwrap();
                    let image = Image::new().rect([0.0, 0.0, r.width * size.width, r.height * size.height]);
                    let trans = c.transform.trans(r.x * size.width, r.y * size.height);
                    image.draw(tex, &Default::default(), trans, g);
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
        if let Some(resize) = e.resize_args() {
            game.set_aspect_ratio(resize.window_size[0] / resize.window_size[1])
        }
        e.mouse_cursor(|pos| {
            mouse_state.x = pos[0] / window.size().width;
            mouse_state.y = pos[1] / window.size().height;
        });
        if let Some(_) = e.update_args() {
            game.update(mouse_state);
        }
    }
}
