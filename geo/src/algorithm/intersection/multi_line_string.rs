use algorithm::intersects::Intersects;
use geo_types::{MultiLineString,Point};
use num_traits::Float;
use super::Intersection;
use super::intersection_enums::MultiLineStringIntersection;

impl<T> Intersection<Point<T>> for MultiLineString<T> where T: Float {
    type Output = MultiLineStringIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if self.intersects(rhs) {
            MultiLineStringIntersection::Point(rhs.clone())
        } else {
            MultiLineStringIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{Coordinate, LineString, Point};
    use super::*;

    #[test]
    fn with_point() {
        let line_string1 = line_string![(0.0, 0.0), (10.0, 0.0)];
        let line_string2 = line_string![(100.0, 100.0), (200.0, 100.0)];
        let mls = MultiLineString(vec![line_string1, line_string2]);

        // Start of first LineString in MultiLineString
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            mls.intersection(&point),
            MultiLineStringIntersection::Point(point.clone())
        );

        // On first LineString in MultiLineString
        let point = Point::new(5.0, 0.0);

        assert_eq!(
            mls.intersection(&point),
            MultiLineStringIntersection::Point(point.clone())
        );

        // On second LineString in MultiLineString
        let point = Point::new(150.0, 100.0);

        assert_eq!(
            mls.intersection(&point),
            MultiLineStringIntersection::Point(point.clone())
        );

        // Not on MultiLineString
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            mls.intersection(&point),
            MultiLineStringIntersection::None,
        );
    }
}
