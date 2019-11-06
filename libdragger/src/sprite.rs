use uuid::Uuid;

use crate::Game;

pub mod sprite_actions {
    use crate::Game;
    use uuid::Uuid;

    pub fn attach_to_mouse(game: &mut Game, id: Uuid) {
        game.mouse_sprite = Some(id);
    }

    pub fn debug(game: &mut Game, id: Uuid) {
        let sprite = game.get_sprite(id).unwrap();
        println!("Id: {}, x: {}, y: {}", id, sprite.x, sprite.y);
    }
}

pub struct Sprite {
    pub id: Uuid,

    pub x: f64,
    pub y: f64,
    pub z: f64,

    pub width: f64,
    pub height: f64,

    pub hit_action: Option<fn(&mut Game, Uuid) -> ()>,

    pub texture: String,
}

impl Sprite {
    pub fn new(path: &str, width: f64 , height: f64 ) -> Sprite {
        Sprite { id: Uuid::new_v4(), x: 0.0, y: 0.0, z: 0.0, width, height, hit_action: Some(sprite_actions::attach_to_mouse), texture: path.to_owned()}
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

