pub mod image_handler {
    use core::panic;
    use image::{io::Reader as ImageReader, GenericImageView};
    use std::fs::File;
    use std::{
        fmt::format,
        io::{Cursor, ErrorKind},
    };

    pub fn input_image(image_path: String) -> image::RgbImage {
        let img = ImageReader::open(image_path)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgb8(); // TODO:remove unwpap statement

        img
    }

    pub fn save_image(image: image::RgbImage, image_path: String) -> String {
        // check and if nessesary create image
        let file = match File::create_new(image_path.clone()) {
            Ok(file) => file,
            Err(error) => File::open(image_path.clone()).unwrap(), // TODO: handle this unwrap statement gracefully
        };

        // save image
        match image.save(image_path) {
            Ok(T) => String::from("succes"),
            Err(err) => panic!("could not save file because of error:{:?}", err),
        }
    }

    pub fn new(width: i32, height: i32) -> image::RgbImage {
        let img = image::RgbImage::new(width as u32, height as u32);
        img
    }
}
#[cfg(test)]
mod tests {
    use super::image_handler;
    use super::*;

    #[test]
    fn test_input_image() {
        let image_path = String::from(
            "test_images/cat_lying_blue_eye_small_ginger_fur_heal_pet_animal-609263.jpeg",
        );
        let img = image_handler::input_image(image_path);
        assert_eq!(img.height(), 800);
        assert_eq!(img.width(), 1200);
    }

    #[test]
    fn test_save_image() {
        let image_path = String::from(
            "test_images/cat_lying_blue_eye_small_ginger_fur_heal_pet_animal-609263.jpeg",
        );
        let img = image_handler::input_image(image_path.clone());
        let new_image_path = String::from(
            "test_images/cat_lying_blue_eye_small_ginger_fur_heal_pet_animal-609263_copy.jpeg",
        );
        image_handler::save_image(img, new_image_path.clone());
        let img = image_handler::input_image(new_image_path);
        assert_eq!(img.height(), 800);
        assert_eq!(img.width(), 1200);
    }

    #[test]
    fn test_create_image_file() {
        let img = image_handler::new(800, 1200);
        assert_eq!(img.width(), 800);
        assert_eq!(img.height(), 1200);
    }
}
