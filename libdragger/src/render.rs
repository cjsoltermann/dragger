use crate::sprite::Sprite;
use crate::Camera;

pub struct Render {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub render_type: RenderType,
}

pub enum RenderType {
    Sprite(String),
    Text(String),
}

pub struct RenderSystem {
    pub current_render: usize,
}

impl RenderSystem {
    pub fn next_render(&mut self, sprites: &Vec<Sprite>, camera: &Camera) -> Option<Render> {
        if let Some(sprite) = sprites.get(self.current_render) {
            self.current_render += 1;
            return Some(Render {x: (sprite.x - camera.x) / camera.width, y: (sprite.y - camera.y) / camera.height, width: sprite.width / camera.width, height: sprite.height / camera.height, render_type: RenderType::Sprite(sprite.texture.to_owned())})
        }
        None
    }
}