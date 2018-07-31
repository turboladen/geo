use algorithm::intersects::Intersects;
use geo_types::{Geometry, Point};
use num_traits::Float;
use super::Intersection;
use super::intersection_enums::GeometryIntersection;

impl<T> Intersection<Point<T>> for Geometry<T> where T: Float {
    type Output = GeometryIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if self.intersects(rhs) {
            GeometryIntersection::Point(rhs.clone())
        } else {
            GeometryIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_point_with_point() {
        let p1 = Point::new(180.0, 180.0);
        let p2 = p1.clone();
        let geometry_point = Geometry::Point(p1);

        assert_eq!(geometry_point.intersection(&p2), GeometryIntersection::Point(p1.clone()));

        // Not intersecting
        let p2 = Point::new(2.0, 2.0);

        assert_eq!(geometry_point.intersection(&p2), GeometryIntersection::None)
    }
}
