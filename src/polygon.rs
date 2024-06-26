pub mod polygon {
    use crate::{vectors::vectors::Point3, Vector3, Vector_3};

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Polygon {
        pub v0: Point3,
        pub v1: Point3,
        pub v2: Point3,

        pub x_max: f32,
        pub y_max: f32,
        pub z_max: f32,

        pub x_min: f32,
        pub y_min: f32,
        pub z_min: f32,
    }
    impl Polygon {
        pub fn new(v0: Point3, v1: Point3, v2: Point3) -> Self {
            let mut x_max: f32 = 0.0;
            let mut y_max: f32 = 0.0;
            let mut z_max: f32 = 0.0;

            let mut x_min: f32 = 0.0;
            let mut y_min: f32 = 0.0;
            let mut z_min: f32 = 0.0;

            let temp_vec = vec![v0, v1, v2];
            for item in temp_vec.iter() {
                if item.x() > x_max {
                    x_max = item.x();
                }
                if item.x() < x_min {
                    x_min = item.x();
                }

                if item.y() > y_max {
                    y_max = item.y();
                }
                if item.y() < y_min {
                    y_min = item.y();
                }

                if item.z() > z_max {
                    z_max = item.z();
                }
                if item.z() < z_min {
                    z_min = item.z();
                }
            }

            Polygon {
                v0: v0,
                v1: v1,
                v2: v2,

                x_max: x_max,
                y_max: y_max,
                z_max: z_max,

                x_min: x_min,
                y_min: y_min,
                z_min: z_min,
            }
        }

        pub fn get_cross_product<T: Vector3>(a_hat: T, b_hat: T) -> Point3 {
            let part_0 = (a_hat.y() * b_hat.z()) - (a_hat.z() * b_hat.y());
            let part_1 = (a_hat.z() * b_hat.x()) - (a_hat.x() * b_hat.z());
            let part_2 = (a_hat.x() * b_hat.y()) - (a_hat.y() * b_hat.x());

            let cross_product = Point3::new(part_0, part_1, part_2);
            cross_product
        }
        pub fn get_dot_product<T: Vector3>(a_hat: T, b_hat: T) -> f32 {
            let a = a_hat.x() * b_hat.x();
            let b = a_hat.y() * b_hat.y();
            let c = a_hat.z() * b_hat.z();
            a + b + c
        }
        fn get_normal_line(&self) -> Point3 {
            // calculate point in the middle of the line betwen two points of our polygon
            //
            let mut a_hat = self.v0.clone();
            let mut b_hat = self.v0.clone();

            a_hat.subtract(self.v1);
            b_hat.subtract(self.v2);

            let normal_line = Polygon::get_cross_product(a_hat, b_hat);
            normal_line
        }
        pub fn get_plane_vector(&self) -> Vec<f32> {
            let normal_line = self.get_normal_line();
            let s = self.v0;
            let mut n_negative = normal_line.clone();
            n_negative.multiply_by_number(-1.0);

            let k = (n_negative.x * s.x) + (n_negative.y * s.y) + (n_negative.z * s.z);

            let p = vec![normal_line.x, normal_line.y, normal_line.z, k];
            p
        }
    }

    #[cfg(test)]
    mod tests {
        use std::vec;

        use super::*;
        #[test]
        fn test_poligon_creation() -> () {
            let v0 = Point3::new(1.1, 2.2, 3.3);
            let v1 = Point3::new(2.1, 3.2, 4.3);
            let v2 = Point3::new(3.1, 4.2, 5.3);
            let poligon = Polygon::new(v0, v1, v2);
            assert_eq!(format!("{:?}", poligon), "Polygon { v0: Point3 { x: 1.1, y: 2.2, z: 3.3 }, v1: Point3 { x: 2.1, y: 3.2, z: 4.3 }, v2: Point3 { x: 3.1, y: 4.2, z: 5.3 } }")
        }
        #[test]
        fn test_normal_line_calculation() -> () {
            let v0 = Point3::new(-1.0, 0.0, 2.0);
            let v1 = Point3::new(1.0, 0.0, 2.0);
            let v2 = Point3::new(0.0, -1.0, 3.0);

            let poligon = Polygon::new(v0, v1, v2);

            assert_eq!(poligon.get_normal_line(), Point3::new(0.0, -2.0, -2.0));
        }
        #[test]
        fn test_calculating_plane_vector() -> () {
            let v0 = Point3::new(-1.0, 0.0, 2.0);
            let v1 = Point3::new(1.0, 0.0, 2.0);
            let v2 = Point3::new(0.0, -1.0, 3.0);

            let poligon = Polygon::new(v0, v1, v2);

            assert_eq!(poligon.get_plane_vector(), vec![0.0, -2.0, -2.0, 4.0]);
        }
    }
}
