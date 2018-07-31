use algorithm::intersects::Intersects;
use geo_types::{MultiPolygon, Point};
use num_traits::Float;
use super::Intersection;
use super::intersection_enums::MultiPolygonIntersection;

impl<T> Intersection<Point<T>> for MultiPolygon<T> where T: Float {
    type Output = MultiPolygonIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if self.intersects(rhs) {
            MultiPolygonIntersection::Point(rhs.clone())
        } else {
            MultiPolygonIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{Coordinate, LineString, MultiPolygon, Point, Polygon};
    use super::*;

    #[test]
    fn with_point() {
        let polygon1 = {
            let exterior = line_string![(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)];
            let interiors = vec![
                line_string![(0.1, 0.1), (0.9, 0.9), (0.9, 0.1), (0.1, 0.1)],
            ];
            Polygon::new(exterior, interiors)
        };

        let polygon2 = {
            let exterior = line_string![(100.0, 100.0), (101.0, 101.0), (101.0, 100.0), (100.0, 100.0)];

            let interiors = vec![
                line_string![(100.1, 100.1), (100.9, 100.9), (100.9, 100.1), (100.1, 100.1)],
            ];
            Polygon::new(exterior, interiors)
        };

        let multi_polygon = MultiPolygon(vec![polygon1, polygon2]);

        //-----------------------------------------------------------------------------------------
        // FIRST POLYGON
        //-----------------------------------------------------------------------------------------
        // Start of first Polygon exterior
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        // On Polygon exterior
        let point = Point::new(0.5, 0.0);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        // Start of first Polygon interior
        let point = Point::new(0.1, 0.1);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        // On first Polygon interior
        let point = Point::new(0.9, 0.4);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        // In first Polygon interior
        let point = Point::new(0.05, 0.05);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        //-----------------------------------------------------------------------------------------
        // SECOND POLYGON
        //-----------------------------------------------------------------------------------------
        // Start of second Polygon exterior
        let point = Point::new(100.0, 100.0);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        // On Polygon exterior
        let point = Point::new(100.5, 100.0);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        // Start of second Polygon interior
        let point = Point::new(100.1, 100.1);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        // On second Polygon interior
        let point = Point::new(100.9, 100.4);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        // In second Polygon interior
        let point = Point::new(100.05, 100.05);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::Point(point.clone())
        );

        //-----------------------------------------------------------------------------------------
        // Not in or on Polygon
        //-----------------------------------------------------------------------------------------
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            multi_polygon.intersection(&point),
            MultiPolygonIntersection::None,
        );
    }
}
