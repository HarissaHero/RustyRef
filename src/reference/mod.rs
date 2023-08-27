use std::collections::HashMap;

use image::DynamicImage;

pub struct Image {
    pub position: [f32; 2],
    pub image: DynamicImage,
}

impl Image {
    pub fn new(position: [f32; 2], bytes: Vec<u8>) -> Self {
        let image = image::load_from_memory(&bytes).unwrap();

        Self { position, image }
    }
}

pub struct Library {
    images: HashMap<uuid::Uuid, Image>,
}

impl Library {
    pub fn new() -> Self {
        let images = HashMap::new();
        Self { images }
    }

    pub fn insert(&mut self, image: Image) -> Option<uuid::Uuid> {
        let key = uuid::Uuid::new_v4();
        println!("{:?}", image.position);
        let maybe_value = self.images.insert(key, image);
        match maybe_value {
            Some(value) => {
                println!("Key already exists ! rollback to old value");
                self.images.insert(key, value);
                return Some(key);
            }
            None => {
                println!("New image added to Library");
                return Some(key);
            }
        };
    }

    pub fn get(&self, key: &uuid::Uuid) -> Option<&Image> {
        self.images.get(key)
    }
}
