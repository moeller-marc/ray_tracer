mod image_handler{
    
    pub mod image_handler {
        use std::io::Cursor;
        use image::{io::Reader as ImageReader, GenericImageView};

        pub fn input_image(image_path: String) -> image::RgbImage {
            let img = ImageReader::open(image_path).unwrap().decode().unwrap().to_rgb8();

            img
        }

        pub fn save_image(image: image::RgbImage, image_path: String) -> () {
            image.save(image_path).unwrap();
        }

        pub fn create_image_file(width:i32, height:i32) -> image::RgbImage {
            let img = image::RgbImage::new(width as u32, height as u32);
            img
    }

    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use super::image_handler;


        #[test]
        fn test_input_image() {
            let image_path = String::from("test_images/cat_lying_blue_eye_small_ginger_fur_heal_pet_animal-609263.jpeg");
            let img = image_handler::input_image(image_path);
            assert_eq!(img.height(), 800);
            assert_eq!(img.width(), 1200);
        }

        #[test]
        fn test_save_image() {
            let image_path = String::from("test_images/cat_lying_blue_eye_small_ginger_fur_heal_pet_animal-609263.jpeg");
            let img = image_handler::input_image(image_path.clone());
            let new_image_path = String::from("test_images/cat_lying_blue_eye_small_ginger_fur_heal_pet_animal-609263_copy.jpeg");
            image_handler::save_image(img, new_image_path.clone());
            let img = image_handler::input_image(new_image_path);
            assert_eq!(img.height(), 800);
            assert_eq!(img.width(), 1200);
        } 

        #[test]
        fn test_create_image_file() {
            let img = image_handler::create_image_file(800, 1200);
            assert_eq!(img.width(), 800);
            assert_eq!(img.height(), 1200);
        }
    }
}
