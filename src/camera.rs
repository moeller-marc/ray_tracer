pub mod camera {
    use crate::image_handler::image_handler;
    use crate::{Color, Point3, Polygon, Ray, Vector3, Vector_3};
    use image::*;

    pub struct Camera {
        origin: Point3,
        pitch: f32,
        yaw: f32,
    }

    impl Camera {
        pub fn new(origin: Point3, pitch: f32, yaw: f32) -> Self {
            let camera = Camera {
                origin: origin,
                pitch: pitch,
                yaw: yaw,
            };
            camera
        }
        pub fn render(&self, polygons: Vec<Polygon>, witdh: f32, height: f32) -> Vec<Vec<Color>> {
            let mut image: Vec<Vec<Color>> = vec![];

            let pixel_x = 16.0 / witdh;
            let pixel_y = 9.0 / height;

            let pixel_middle_x = pixel_x / 2.0;
            let pixel_middle_y = pixel_y / 2.0;

            println!("{}", pixel_y);
            let mut grid: Vec<Vec<Ray>> = vec![];
            for row in 0..height as i32 {
                let mut tem_vec = vec![];
                let mut img_vec: Vec<Color> = vec![];

                for column in 0..witdh as i32 {
                    let ray = Ray::new(
                        self.origin,
                        Vector_3::new(
                            pixel_x * (column as f32 - pixel_middle_x),
                            pixel_y * (row as f32 - pixel_middle_y),
                            1.0,
                        ),
                    );

                    for polygon in polygons.clone() {
                        // println!("polygon: {:?}", polygon);
                        println!("ray: {:?}", ray);
                        if ray.in_bounding_box(polygon.clone()) {
                            if ray.intersects_with_polygon(polygon) {
                                img_vec.push(Color::new(255.0, 255.0, 255.0));
                                // println!("got it white");
                            } else {
                                img_vec.push(Color::new(0.0, 0.0, 0.0));
                                // println!("got it black");
                            }
                        }
                    }
                }
                grid.push(tem_vec);
                image.push(img_vec);
            }
            println!("{:?}", image);
            return image;
        }
    }
}
