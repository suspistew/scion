use std::path::Path;

use image::{DynamicImage, ImageFormat};

use crate::{core::components::color::Color, utils::file::read_file};

/// Component used by the 2D Renderer to know which material to use when rendering a renderable object.
#[derive(Clone)]
pub enum Material2D {
    /// Fill with a color
    Color(Color),
    /// Use a texture. Note that this means the target object will need to have uv maps.
    Texture(String),
}

pub(crate) struct Texture {
    pub(crate) bytes: Vec<u8>,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl Texture {
    pub fn from_png(file_path: &Path) -> Texture {
        if let Ok(bytes) = read_file(&file_path) {
            let converted_image = image::load_from_memory_with_format(&bytes, ImageFormat::Png);
            if let Ok(image) = converted_image {
                return Texture::create_texture_from_dynamic_image(image);
            }
        }
        log::error!("Error while loading your texture, loading fallback texture instead.");
        Texture::fallback_texture()
    }

    fn fallback_texture() -> Texture {
        let bytes = include_bytes!("missing_texture.png");
        let image = image::load_from_memory_with_format(bytes, ImageFormat::Png)
            .expect("The fallback image has not been found !");
        return Texture::create_texture_from_dynamic_image(image);
    }

    fn create_texture_from_dynamic_image(dynamic_image: DynamicImage) -> Texture {
        let image = dynamic_image.to_rgba8();
        let width = image.width() as u16;
        let height = image.height() as u16;
        let bytes = image.into_raw();

        Texture {
            bytes,
            width,
            height,
        }
    }
}
