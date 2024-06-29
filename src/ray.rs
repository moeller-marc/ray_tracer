pub mod ray {

    use crate::polygon::polygon::{self, Polygon};
    use crate::vectors::vectors::{Point3, Vector3};
    use crate::Vector_3;

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

        fn calculate_intersection_point(&self, plane_vector: Vec<f32>) -> Vector_3 {
            let lambda = self.calculate_lambda(plane_vector);
            let i_x = lambda * self.angle.x() + self.origin.x();
            let i_y = lambda * self.angle.y() + self.origin.y();
            let i_z = lambda * self.angle.z() + self.origin.z();
            let vector_out = Vector_3::new(i_x, i_y, i_z);
            vector_out
        }
        pub fn in_bounding_box(&self, polygon: Polygon) -> bool {
            let intersection_points = self.calculate_intersection_point(polygon.get_plane_vector());
            if intersection_points.x() > polygon.x_max {
                return false;
            }
            if intersection_points.x() < polygon.x_min {
                return false;
            }
            if intersection_points.y() > polygon.y_max {
                return false;
            }
            if intersection_points.y() < polygon.y_min {
                return false;
            }
            if intersection_points.z() > polygon.z_max {
                return false;
            }
            if intersection_points.z() < polygon.z_min {
                return false;
            }

            true
        }

        pub fn intersects_with_polygon(&self, poly: polygon::Polygon) -> bool {
            let intersection_points = self.calculate_intersection_point(poly.get_plane_vector());

            let ver_0 = poly.v0;
            let ver_1 = poly.v1;
            let ver_2 = poly.v2;
            {
                // check wether or not i is on the same side as vertecy 0 (ver_0)
                let v_bar = ver_1.subtract(ver_2);
                let a_bar = polygon::Polygon::get_cross_product(
                    v_bar,
                    (intersection_points.subtract(ver_2)),
                );
                let b_bar = polygon::Polygon::get_cross_product(v_bar, (ver_0.subtract(ver_2)));

                let c = polygon::Polygon::get_dot_product(a_bar, b_bar);

                if c < 0.0 {
                    return false;
                }
            }
            {
                // check wether or not i is on the same side as vertecy 1 (ver_1)
                let v_bar = ver_0.subtract(ver_2);
                let a_bar = polygon::Polygon::get_cross_product(
                    v_bar,
                    (intersection_points.subtract(ver_2)),
                );
                let b_bar = polygon::Polygon::get_cross_product(v_bar, (ver_1.subtract(ver_2)));

                let c = polygon::Polygon::get_dot_product(a_bar, b_bar);

                if c < 0.0 {
                    return false;
                }
            }
            {
                // check wether or not i is on the same side as vertecy 2 (ver_2)
                let v_bar = ver_0.subtract(ver_1);
                let a_bar = polygon::Polygon::get_cross_product(
                    v_bar,
                    (intersection_points.subtract(ver_1)),
                );
                let b_bar = polygon::Polygon::get_cross_product(v_bar, (ver_2.subtract(ver_1)));

                let c = polygon::Polygon::get_dot_product(a_bar, b_bar);

                if c < 0.0 {
                    return false;
                }
            }
            return true;
        }

        pub fn calculate_distance(&self, poly: Polygon) -> f32 {
            let intersection_points = self.calculate_intersection_point(poly.get_plane_vector());

            let a = (intersection_points.x() - self.origin.x())
                * (intersection_points.x() - self.origin.x());
            let b = (intersection_points.y() - self.origin.y())
                * (intersection_points.y() - self.origin.y());
            let c = (intersection_points.z() - self.origin.z())
                * (intersection_points.z() - self.origin.z());

            let d = f32::sqrt(a + b + c);
            d
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
