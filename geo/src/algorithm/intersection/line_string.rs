use algorithm::intersects::Intersects;
use geo_types::{LineString, Point};
use num_traits::Float;
use super::Intersection;
use super::intersection_enums::LineStringIntersection;

impl<T> Intersection<Point<T>> for LineString<T> where T: Float {
    type Output = LineStringIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if self.intersects(rhs) {
            LineStringIntersection::Point(rhs.clone())
        } else {
            LineStringIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{Coordinate, LineString, Point};
    use super::*;

    #[test]
    fn with_point() {
        let line_string = line_string![(0.0, 0.0), (10.0, 0.0)];

        // Start of LineString
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            line_string.intersection(&point),
            LineStringIntersection::Point(point.clone())
        );

        // On of LineString
        let point = Point::new(5.0, 0.0);

        assert_eq!(
            line_string.intersection(&point),
            LineStringIntersection::Point(point.clone())
        );

        // End of LineString
        let point = Point::new(10.0, 0.0);

        assert_eq!(
            line_string.intersection(&point),
            LineStringIntersection::Point(point.clone())
        );

        // Not on LineString
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            line_string.intersection(&point),
            LineStringIntersection::None,
        );
    }
}
