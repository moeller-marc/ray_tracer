pub mod ray {
    use super::super::vectors::vectors::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Ray {
        origin: Point3,
        angle: Vector_3,
    }

    impl Ray {
        pub fn new(origin: Point3, angle: Vector_3) -> Self {
            Ray {
                origin: origin,
                angle: angle,
            }
        }

        fn calculate_lambda(&self, plane_vector: Vec<f32>) -> f32 {
            let upper_part = (plane_vector[0] * self.origin.x())
                + (plane_vector[1] * self.origin.y())
                + (plane_vector[2] * self.origin.z())
                + plane_vector[3];

            let lower_part = (plane_vector[0] * self.angle.x())
                + (plane_vector[1] * self.angle.y())
                + (plane_vector[2] * self.angle.z());

            let combination = upper_part / lower_part;

            let combination = combination * -1.0;

            combination
        }

        pub fn calculate_intersection_point(&self, plane_vector: Vec<f32>) -> Vector_3 {
            let lambda = self.calculate_lambda(plane_vector);
            let i_x = lambda * self.angle.x() + self.origin.x();
            let i_y = lambda * self.angle.y() + self.origin.y();
            let i_z = lambda * self.angle.z() + self.origin.z();
            let vector_out = Vector_3::new(i_x, i_y, i_z);
            vector_out
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate_intersection_point() -> () {
            let origin = Point3::new(4.0, 5.0, 6.0);
            let angle = Vector_3::new(1.0, 2.0, 3.0);
            let plane_vector = vec![1.0, 2.0, -1.0, 3.0];
            let ray = Ray::new(origin, angle);
            let result = ray.calculate_intersection_point(plane_vector);

            let intended_result = Vector_3::new(-3.0 / 2.0, -6.0, -21.0 / 2.0);

            assert_eq!(result, intended_result);
        }

        #[test]
        fn test_calculate_lamda() -> () {
            let origin = Point3::new(4.0, 5.0, 6.0);
            let angle = Vector_3::new(1.0, 2.0, 3.0);
            let plane_vector = vec![1.0, 2.0, -1.0, 3.0];
            let ray = Ray::new(origin, angle);
            let result = ray.calculate_lambda(plane_vector);

            let intended_result = -11.0 / 2.0;

            assert_eq!(result, intended_result);
        }
    }
}
