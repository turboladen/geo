use algorithm::intersects::Intersects;
use geo_types::{Point, Polygon};
use num_traits::Float;
use super::Intersection;
use super::intersection_enums::PolygonIntersection;

impl<T> Intersection<Point<T>> for Polygon<T> where T: Float {
    type Output = PolygonIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if self.intersects(rhs) {
            PolygonIntersection::Point(rhs.clone())
        } else {
            PolygonIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{Coordinate, LineString, Point, Polygon};
    use super::*;

    #[test]
    fn with_point() {
        let exterior = line_string![(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)];
        let interiors = vec![
            line_string![(0.1, 0.1), (0.9, 0.9), (0.9, 0.1), (0.1, 0.1)],
        ];
        let polygon = Polygon::new(exterior, interiors);

        // Start of Polygon exterior
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            polygon.intersection(&point),
            PolygonIntersection::Point(point.clone())
        );

        // On Polygon exterior
        let point = Point::new(0.5, 0.0);

        assert_eq!(
            polygon.intersection(&point),
            PolygonIntersection::Point(point.clone())
        );

        // Start of Polygon interior
        let point = Point::new(0.1, 0.1);

        assert_eq!(
            polygon.intersection(&point),
            PolygonIntersection::Point(point.clone())
        );

        // On Polygon interior
        let point = Point::new(0.9, 0.4);

        assert_eq!(
            polygon.intersection(&point),
            PolygonIntersection::Point(point.clone())
        );

        // In Polygon interior
        let point = Point::new(0.05, 0.05);

        assert_eq!(
            polygon.intersection(&point),
            PolygonIntersection::Point(point.clone())
        );

        // Not in or on Polygon
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            polygon.intersection(&point),
            PolygonIntersection::None,
        );
    }
}
