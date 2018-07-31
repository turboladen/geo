use algorithm::intersects::Intersects;
use geo_types::{MultiPoint, Point};
use num_traits::Float;
use super::Intersection;
use super::intersection_enums::MultiPointIntersection;

impl<T> Intersection<Point<T>> for MultiPoint<T> where T: Float {
    type Output = MultiPointIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if self.intersects(rhs) {
            MultiPointIntersection::Point(rhs.clone())
        } else {
            MultiPointIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::Point;
    use super::*;

    #[test]
    fn with_point() {
        let multi_point = multi_point!((0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0));

        // Point on MultiPoint
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            multi_point.intersection(&point),
            MultiPointIntersection::Point(point.clone())
        );

        // Not in or on MultiPoint
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            multi_point.intersection(&point),
            MultiPointIntersection::None,
        );
    }
}
