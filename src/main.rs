mod camera;
mod image_handler;
mod polygon;
mod ray;
mod vectors;

use camera::camera::Camera;
use image_handler::*;
use polygon::polygon::*;
use ray::ray::*;
use vectors::vectors::*;

fn main() {
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), 0.0, 0.0);
    // let poly = Polygon::new(
    //     Point3::new(0.0, 0.0, 0.0),
    //     Point3::new(3.0, 8.0, 4.0),
    //     Point3::new(13.0, 12.0, 13.0),
    // );
    let poly = Polygon::new(
        Point3::new(0.0, 0.0, -1000000.0),
        Point3::new(0.0, 0.0, -1000000.0),
        Point3::new(0.0, 0.0, -1000000.0),
    );
    let polygon_list = vec![poly];
    // let polygon_list = vec![];
    let out = camera.render(polygon_list, 300.0, 200.0);
    println!("{:?}", out);
}
