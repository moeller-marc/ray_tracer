pub mod vectors {

    pub trait Vector3 {
        fn new(x: f32, y: f32, z: f32) -> Self;

        fn x(&self) -> f32;
        fn y(&self) -> f32;
        fn z(&self) -> f32;

        fn x_mut(&mut self) -> &mut f32;
        fn y_mut(&mut self) -> &mut f32;
        fn z_mut(&mut self) -> &mut f32;

        fn add<T: Vector3>(&mut self, other: T) -> () {
            *self.x_mut() += other.x();
            *self.y_mut() += other.y();
            *self.z_mut() += other.z();
        }

        fn subtract<T: Vector3>(&mut self, other: T) -> () {
            *self.x_mut() -= other.x();
            *self.y_mut() -= other.y();
            *self.z_mut() -= other.z();
        }

        fn multiply_by_number(&mut self, num: f32) -> () {
            *self.x_mut() *= num;
            *self.y_mut() *= num;
            *self.z_mut() *= num;
        }

        fn divide_by_number(&mut self, num: f32) -> () {
            *self.x_mut() /= num;
            *self.y_mut() /= num;
            *self.z_mut() /= num;
        }

        fn length(&self) -> f32 {
            self.length_squared().sqrt()
        }
        fn length_squared(&self) -> f32 {
            self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Point3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl Vector3 for Point3 {
        fn new(x: f32, y: f32, z: f32) -> Self {
            let point_out = Point3 { x: x, y: y, z: z };
            point_out
        }

        fn x(&self) -> f32 {
            self.x
        }
        fn y(&self) -> f32 {
            self.y
        }
        fn z(&self) -> f32 {
            self.z
        }

        fn x_mut(&mut self) -> &mut f32 {
            &mut self.x
        }
        fn y_mut(&mut self) -> &mut f32 {
            &mut self.y
        }
        fn z_mut(&mut self) -> &mut f32 {
            &mut self.z
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Color {
        r: f32,
        g: f32,
        b: f32,
    }
    impl Vector3 for Color {
        fn new(r: f32, g: f32, b: f32) -> Self {
            let point_out = Color { r: r, g: g, b: b };
            point_out
        }

        fn x(&self) -> f32 {
            self.r
        }
        fn y(&self) -> f32 {
            self.g
        }
        fn z(&self) -> f32 {
            self.b
        }

        fn x_mut(&mut self) -> &mut f32 {
            &mut self.r
        }
        fn y_mut(&mut self) -> &mut f32 {
            &mut self.g
        }
        fn z_mut(&mut self) -> &mut f32 {
            &mut self.b
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_point3_new() {
            let p = Point3::new(1.0, 2.0, 3.0);
            assert_eq!(
                p,
                Point3 {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0
                }
            );
        }

        #[test]
        fn test_color_new() {
            let c = Color::new(0.1, 0.2, 0.3);
            assert_eq!(
                c,
                Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3
                }
            );
        }

        #[test]
        fn test_point3_add() {
            let mut p1 = Point3::new(1.0, 2.0, 3.0);
            let p2 = Point3::new(4.0, 5.0, 6.0);
            p1.add(p2);
            assert_eq!(
                p1,
                Point3 {
                    x: 5.0,
                    y: 7.0,
                    z: 9.0
                }
            );
        }

        #[test]
        fn test_color_add() {
            let mut c1 = Color::new(0.1, 0.2, 0.3);
            let c2 = Color::new(0.4, 0.5, 0.6);
            c1.add(c2);
            c1.b = c1.b * 100.0;
            c1.b.round();
            c1.b = c1.b / 100.0;
            assert_eq!(
                c1,
                Color {
                    r: 0.5,
                    g: 0.7,
                    b: 0.9
                }
            );
        }

        #[test]
        fn test_point3_multiply_by_number() {
            let mut p = Point3::new(1.0, 2.0, 3.0);
            p.multiply_by_number(2.0);
            assert_eq!(
                p,
                Point3 {
                    x: 2.0,
                    y: 4.0,
                    z: 6.0
                }
            );
        }

        #[test]
        fn test_color_multiply_by_number() {
            let mut c = Color::new(0.1, 0.2, 0.3);
            c.multiply_by_number(2.0);
            assert_eq!(
                c,
                Color {
                    r: 0.2,
                    g: 0.4,
                    b: 0.6
                }
            );
        }

        #[test]
        fn test_point3_divide_by_number() {
            let mut p = Point3::new(2.0, 4.0, 6.0);
            p.divide_by_number(2.0);
            assert_eq!(
                p,
                Point3 {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0
                }
            );
        }

        #[test]
        fn test_color_divide_by_number() {
            let mut c = Color::new(0.2, 0.4, 0.6);
            c.divide_by_number(2.0);
            assert_eq!(
                c,
                Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3
                }
            );
        }

        #[test]
        fn test_point3_length() {
            let p = Point3::new(1.0, 2.0, 2.0);
            assert_eq!(p.length(), 3.0);
        }

        #[test]
        fn test_color_length() {
            let c = Color::new(1.0, 2.0, 2.0);
            assert_eq!(c.length(), 3.0);
        }

        #[test]
        fn test_point3_length_squared() {
            let p = Point3::new(1.0, 2.0, 2.0);
            assert_eq!(p.length_squared(), 9.0);
        }

        #[test]
        fn test_color_length_squared() {
            let c = Color::new(1.0, 2.0, 2.0);
            assert_eq!(c.length_squared(), 9.0);
        }
    }
}
