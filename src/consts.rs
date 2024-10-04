pub const CHAR_ASPECT_RATIO: f32 = 1.5;
pub const CHAR_SIZE: f32 = 16.0;

pub mod ui {
    pub const FONT_SIZE: f32 = 16.;
}

pub mod colors {
    macro_rules! rgb {
        ($r: expr, $g: expr, $b: expr) => {
            color_u8!($r, $g, $b, 255)
        };
    }

    use macroquad::{color::Color, color_u8};

    pub const BG: Color = rgb!(241, 241, 241);
    pub const BG2: Color = rgb!(210, 210, 210);
    pub const TEXT: Color = rgb!(10, 10, 10);
    pub const INFO: Color = rgb!(37, 54, 141);
    pub const ERROR: Color = rgb!(112, 36, 31);
    pub const SELECTED: Color = rgb!(107, 144, 211);
}

pub mod textures {
    use lazy_static::lazy_static;
    use macroquad::{
        prelude::ImageFormat,
        texture::{Image, Texture2D},
    };

    struct Textures {
        cursor: Texture2D,
        paint: Texture2D,
        line: Texture2D,
    }

    impl Textures {
        fn new() -> Self {
            macro_rules! include_image {
                ($path:expr) => {{
                    macroquad::texture::Texture2d
                    include_bytes!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/assets/textures/",
                        $path,
                        ".png"
                    ))
                }};
            }

            fn get_texture(name: &str) -> Texture2D {
                Texture2D::from_image(
                    &Image::from_file_with_format(include_image!(name), Some(ImageFormat::Png))
                        .unwrap(),
                )
            }

            Self {
                cursor: get_texture("cursor.png"),
            }
        }
    }

    lazy_static! {
        pub static ref TEXTURES: Textures = Textures { cursor };
    }
}
