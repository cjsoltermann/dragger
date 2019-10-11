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
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub render_type: RenderType
}

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
}

impl Game {
    pub fn new() -> Game {
        Game {
            sprites: vec![Sprite {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                render_type: RenderType::Sprite("planks.png".to_owned())
            }],
        }
    }

    pub fn update(&mut self, mouse_state: &MouseState) {
        if mouse_state.down {
            let sprite = self.sprites.get_mut(0).unwrap();
            sprite.x = mouse_state.x;
            sprite.y = mouse_state.y;
        }
    }

    pub fn get_next_render(&self) -> &Sprite {
        self.sprites.get(0).unwrap()
    }
}
