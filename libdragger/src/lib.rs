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

    pub width: f64,
    pub height: f64,

    pub render_type: RenderType
}

impl Sprite {
    pub fn from_texture(path: &str, width: f64 , height: f64 ) -> Sprite {
        Sprite { id: Uuid::new_v4(), x: 0.0, y: 0.0, z: 0.0, width, height, render_type: RenderType::Sprite(path.to_owned())}
    }

    #[inline(always)]
    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    #[inline(always)]
    pub fn get_center_x(&self) -> f64 {
        self.x + self.width / 2.0
    }

    #[inline(always)]
    pub fn get_center_y(&self) -> f64 {
        self.y + self.height / 2.0
    }

    #[inline(always)]
    pub fn get_center(&self) -> (f64, f64) {
        (self.get_center_x(), self.get_center_y())
    }

    #[inline(always)]
    pub fn set_center_x(&mut self, x: f64 ) {
        self.x = x - self.width / 2.0;     
    }

    #[inline(always)]
    pub fn set_center_y(&mut self, y: f64) {
        self.y = y - self.height / 2.0;
    }

    #[inline(always)]
    pub fn set_center(&mut self, x: f64, y: f64) {
        self.set_center_x(x);
        self.set_center_y(y);
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
    last_mouse_state: MouseState,
    mouse_sprite: Option<Uuid>,
}

impl Game {
    pub fn create() -> Game {
        let mut game = Game::new();
        game.add_sprite(Sprite::from_texture("plank.png", 64.0 / 2.0, 64.0 / 2.0));
        game.add_sprite(Sprite::from_texture("cow.png", 128.0 / 2.0, 128.0 / 2.0));
        game
    }
    fn new() -> Game {
        Game {
            sprites: Vec::new(),
            sprite_indices: HashMap::new(),
            last_mouse_state: MouseState::new(false, 0.0, 0.0),
            mouse_sprite: None,
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
        self.drag(mouse_state);
        self.last_mouse_state = mouse_state;
    }

    fn drag(&mut self, mouse_state: MouseState) {
        if !self.last_mouse_state.down && mouse_state.down { //On mouse down
            if let Some(id) = self.hit(mouse_state.x, mouse_state.y) {
                self.mouse_sprite = Some(id.clone());
            }
        }
        else if self.last_mouse_state.down && !mouse_state.down { //On mouse up
            self.mouse_sprite = None;
        }
        else if mouse_state.down { //On drag
            if let Some(id) = self.mouse_sprite {
                let sprite = self.get_sprite_mut(id).unwrap();
                sprite.set_center(mouse_state.x, mouse_state.y)
            }
        }
    }

    pub fn get_renders(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    fn hit(&self, x: f64, y: f64) -> Option<Uuid> {
        let mut top_z = 0.0;
        let mut top_id = None;
        for sprite in &self.sprites {
            if ((sprite.get_center_x() - x).powi(2) + (sprite.get_center_y() - y).powi(2)).sqrt() < 50.0 && sprite.z >= top_z{
                top_id = Some(sprite.id());
                top_z = sprite.z;
            }
        }
        top_id
    }
}
