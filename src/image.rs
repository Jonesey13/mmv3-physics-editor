use crate::car_type::CarType;
use std::collections::HashMap;
use crate::language::Language;
use base64::encode;

const MMV3_LOGO: &'static [u8] = include_bytes!("../assets/MMV3Logo.png");

pub struct EncodedImageCollection {
    pub images: HashMap<Image, String>
}

impl EncodedImageCollection {
    pub fn build() -> Self {
        let image_enums = vec![Image::Logo, Image::Lang(Language::English)];

        let images: HashMap<Image, String> = image_enums
        .into_iter()
        .map(|image_enum| (image_enum, image_enum.get_encoded_image().get_html_src_string()))
        .collect();

        Self {
            images
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Image {
    Logo,
    Lang(Language),
    Car(CarType)
}

impl Image {
    pub fn get_bytes(&self) -> &'static [u8] {
        match self {
            Image::Logo => MMV3_LOGO,
            Image::Lang(language) => language.get_image(),
            Image::Car(car_type) => car_type.get_image(),
        }
    }

    pub fn get_encoded_image(&self) -> EncodedImage {
        EncodedImage::from_bytes(self.get_bytes(), "png")
    }
}

/// # A model for an image
///
/// As a webview does not have access to the local file system, the given
/// images are encoded into text (Base64) to be displayed.
///
/// ## Fields
///
/// ```text
/// data: String
/// extension: String
/// ```
pub struct EncodedImage {
    data: String,
    extension: String,
}

impl EncodedImage {
    pub fn from_bytes(data: &'static [u8], extension: &str) -> Self {
        Self { 
            data: encode(data), 
            extension: extension.to_string() 
        }
    }

    pub fn get_html_src_string(&self) -> String {
        format!("data:image/{};base64,{}", self.extension, self.data)
    }
}