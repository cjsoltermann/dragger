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
    last_mouse_state: MouseState
}

impl Game {
    pub fn new() -> Game {
        Game {
            sprites: vec![
                Sprite {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    render_type: RenderType::Sprite("planks.png".to_owned())
                },
                Sprite {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    render_type: RenderType::Sprite("cow.png".to_owned())
                }
            ],
            last_mouse_state: MouseState::new(false, 0.0, 0.0)
        }
    }

    pub fn update(&mut self, mouse_state: MouseState) {
        if self.last_mouse_state.down {
            self.sprites[0].x = mouse_state.x;
            self.sprites[0].y = mouse_state.y;
        }
        self.last_mouse_state = mouse_state;
    }

    pub fn get_renders(&self) -> &Vec<Sprite> {
        &self.sprites
    }
}
