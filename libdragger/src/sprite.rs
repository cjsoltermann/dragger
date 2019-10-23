use uuid::Uuid;

pub enum RenderType {
    Sprite(String),
    Text(String),
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

