use uuid::Uuid;
use std::collections::{HashMap};

mod sprite;

pub use sprite::{*};

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

#[derive(Debug, Copy, Clone)]
pub struct MouseState {
    pub down: bool,
    pub x: f64,
    pub y: f64
}

pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Camera {
    fn screen_to_game(&self, x: f64, y: f64) -> (f64, f64) {
        (self.x + x * self.width, self.y + y * self.height)    
    }
}

impl MouseState {
    pub fn new(down: bool, x: f64, y: f64) -> MouseState {
        MouseState { down, x, y }
    }
}

pub struct Game {
    sprites: Vec<Sprite>,
    sprite_indices: HashMap<Uuid, usize>,
    last_mouse_state: MouseState,
    mouse_sprite: Option<Uuid>,
    camera: Camera,
    last_render: usize,
}

impl Iterator for &mut Game {
    type Item = Render;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sprite) = self.sprites.get(self.last_render) {
            self.last_render += 1;
            return Some(Render {x: (sprite.x - self.camera.x) / self.camera.width, y: (sprite.y - self.camera.y) / self.camera.height, width: sprite.width / self.camera.width, height: sprite.height / self.camera.height, render_type: RenderType::Sprite(sprite.texture.to_owned())})
        }
        None
    }
}

impl Game {
    pub fn create() -> Game {
        let mut game = Game::new();
        game.add_sprite(Sprite::new("plank.png", 5.0, 5.0));
        game.add_sprite(Sprite::new("cow.png", 5.0, 5.0));
        game
    }
    fn new() -> Game {
        Game {
            sprites: Vec::new(),
            sprite_indices: HashMap::new(),
            last_mouse_state: MouseState::new(false, 0.0, 0.0),
            mouse_sprite: None,
            camera: Camera {x: 0.0, y: 0.0, width: 50.0, height: 50.0},
            last_render: 0,
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
        self.last_render = 0;
        self.handle_mouse(mouse_state);
        self.last_mouse_state = mouse_state;
    }

    fn handle_mouse(&mut self, mouse_state: MouseState) {
        let (x, y) = self.camera.screen_to_game(mouse_state.x, mouse_state.y);
        println!("Mouse Pos {}, {}. Camera Pos {}, {}", x, y, self.camera.x, self.camera.y);
        if !self.last_mouse_state.down && mouse_state.down { //On mouse down
            if let Some(id) = self.hit(x, y) {
                if let Some(action) = self.get_sprite_mut(id).unwrap().hit_action {
                    action(self, id);
                }
            }
        }
        else if self.last_mouse_state.down && !mouse_state.down { //On mouse up
            self.mouse_sprite = None;
        }
        else if mouse_state.down { //On drag
            if let Some(id) = self.mouse_sprite {
                let sprite = self.get_sprite_mut(id).unwrap();
                sprite.set_center(x, y);
            }
            else {
                self.camera.x -= (mouse_state.x - self.last_mouse_state.x) * self.camera.width;
                self.camera.y -= (mouse_state.y - self.last_mouse_state.y) * self.camera.height;
            }
        }
    }

    fn hit(&self, x: f64, y: f64) -> Option<Uuid> {
        let mut top_z = 0.0;
        let mut top_id = None;
        for sprite in &self.sprites {
            if ((sprite.get_center_x() - x).powi(2) + (sprite.get_center_y() - y).powi(2)).sqrt() < 2.5 && sprite.z >= top_z{
                top_id = Some(sprite.id());
                top_z = sprite.z;
            }
        }
        top_id
    }
}
