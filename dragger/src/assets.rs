use piston_window::*;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;

pub struct Assets {
    textures: HashMap<String, G2dTexture>,
}

impl Assets {
    pub fn create(window: &mut PistonWindow) -> Self {
        let mut textures = HashMap::new();
        let assets = Path::new("dragger/assets/sprites");
        for entry in read_dir(assets).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                //println!("Inserted {}", name);
                let tex = create_texture(window, path);
                textures.insert(name, tex);
            }
        }
        Assets { textures }
    }

    pub fn get(&self, name: &str) -> Option<&G2dTexture> {
        self.textures.get(name)
    }
}

fn create_context(window: &mut PistonWindow) -> G2dTextureContext {
    TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    }
}

fn create_texture(window: &mut PistonWindow, path: PathBuf) -> G2dTexture {
    let mut texture_context = create_context(window);
    Texture::from_path(
        &mut texture_context,
        path,
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap()
}
