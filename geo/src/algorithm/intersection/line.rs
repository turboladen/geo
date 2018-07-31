use algorithm::intersects::Intersects;
use geo_types::{Line, Point};
use num_traits::Float;
use super::Intersection;
use super::intersection_enums::LineIntersection;

impl<T> Intersection<Point<T>> for Line<T> where T: Float {
    type Output = LineIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if rhs.intersects(self) {
            LineIntersection::Point(rhs.clone())
        } else {
            LineIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{Coordinate, Line, Point};
    use super::*;

    #[test]
    fn with_point() {
        let line = build_line![(0.0, 0.0), (10.0, 0.0)];

        // Start of the line
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            line.intersection(&point),
            LineIntersection::Point(point.clone())
        );

        // On the line
        let point = Point::new(5.0, 0.0);

        assert_eq!(
            line.intersection(&point),
            LineIntersection::Point(point.clone())
        );

        // End of the line
        let point = Point::new(10.0, 0.0);

        assert_eq!(
            line.intersection(&point),
            LineIntersection::Point(point.clone())
        );

        // Not on the line
        let point = Point::new(1.0, 1.0);

        assert_eq!(
            line.intersection(&point),
            LineIntersection::None
        );
    }
}
