use uuid::Uuid;
use std::collections::{HashMap};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn test() {
    println!("test");
}

pub struct Sprite {
    pub id: Uuid,

    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub render_type: RenderType
}

impl Sprite {
    pub fn from_texture(path: &str) -> Sprite {
        Sprite { id: Uuid::new_v4(), x: 0.0, y: 0.0, z: 0.0, render_type: RenderType::Sprite(path.to_owned())}
    }

    #[inline(always)]
    pub fn id(&self) -> Uuid {
        self.id.clone()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MouseState {
    pub down: bool,
    pub x: f64,
    pub y: f64
}

impl MouseState {
    pub fn new(down: bool, x: f64, y: f64) -> MouseState {
        MouseState { down, x, y }
    }
}

pub enum RenderType {
    Sprite(String),
    Text(String),
}

pub struct Game {
    sprites: Vec<Sprite>,
    sprite_indices: HashMap<Uuid, usize>,
    last_mouse_state: MouseState
}

impl Game {
    pub fn create() -> Game {
        let mut game = Game::new();
        game.add_sprite(Sprite::from_texture("plank.png"));
        game.add_sprite(Sprite::from_texture("cow.png"));
        game
    }
    fn new() -> Game {
        Game {
            sprites: Vec::new(),
            sprite_indices: HashMap::new(),
            last_mouse_state: MouseState::new(false, 0.0, 0.0)
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) -> Uuid {
        let id = sprite.id();
        let index = self.sprites.len();
        self.sprites.push(sprite);
        self.sprite_indices.insert(id.clone(), index);
        id
    }

    pub fn get_sprite(&self, id: Uuid) -> Option<&Sprite> {
        let mut sprite = None;
        if let Some(i) = self.sprite_indices.get(&id) {
           sprite = Some(self.sprites.get(*i).unwrap());
        }
        sprite
    }

    pub fn get_sprite_mut(&mut self, id: Uuid) -> Option<&mut Sprite> {
        let mut sprite = None;
        if let Some(i) = self.sprite_indices.get(&id) {
           sprite = Some(self.sprites.get_mut(*i).unwrap());
        }
        sprite
    }

    pub fn update(&mut self, mouse_state: MouseState) {
        if !self.last_mouse_state.down && mouse_state.down {
            if let Some(id) = self.hit(mouse_state.x, mouse_state.y) {
                let sprite = self.get_sprite_mut(id).unwrap();
                sprite.x += 1.0;
                sprite.y += 1.0;
            }
        }
        self.last_mouse_state = mouse_state;
    }

    pub fn get_renders(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    fn hit(&self, x: f64, y: f64) -> Option<Uuid> {
        let mut top_y = 0.0;
        let mut top_id = None;
        for sprite in &self.sprites {
            if ((sprite.x - x).powi(2) + (sprite.y - y).powi(2)).sqrt() < 50.0 && sprite.y >= top_y{
                top_id = Some(sprite.id());
                top_y = sprite.y;
            }
        }
        top_id
    }
}
