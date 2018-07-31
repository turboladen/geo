use algorithm::intersects::Intersects;
use geo_types::{Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon};
use num_traits::Float;
use super::Intersection;
use super::intersection_enums::*;

impl<T> Intersection for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if self.intersects(rhs) {
            PointIntersection::Point(self.clone())
        } else {
            PointIntersection::None
        }
    }
}

impl<T> Intersection<Line<T>> for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &Line<T>) -> Self::Output {
        match rhs.intersection(self) {
            LineIntersection::Point(p) => PointIntersection::Point(p.clone()),
            _ => PointIntersection::None,
        }
    }
}

impl<T> Intersection<LineString<T>> for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &LineString<T>) -> Self::Output {
        match rhs.intersection(self) {
            LineStringIntersection::Point(p) => PointIntersection::Point(p.clone()),
            _ => PointIntersection::None,
        }
    }
}

impl<T> Intersection<Polygon<T>> for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &Polygon<T>) -> Self::Output {
        match rhs.intersection(self) {
            PolygonIntersection::Point(p) => PointIntersection::Point(p.clone()),
            _ => PointIntersection::None,
        }
    }
}

impl<T> Intersection<MultiPoint<T>> for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &MultiPoint<T>) -> Self::Output {
        match rhs.intersection(self) {
            MultiPointIntersection::Point(p) => PointIntersection::Point(p.clone()),
            _ => PointIntersection::None,
        }
    }
}

impl<T> Intersection<MultiLineString<T>> for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &MultiLineString<T>) -> Self::Output {
        match rhs.intersection(self) {
            MultiLineStringIntersection::Point(p) => PointIntersection::Point(p.clone()),
            _ => PointIntersection::None,
        }
    }
}

impl<T> Intersection<MultiPolygon<T>> for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &MultiPolygon<T>) -> Self::Output {
        match rhs.intersection(self) {
            MultiPolygonIntersection::Point(p) => PointIntersection::Point(p.clone()),
            _ => PointIntersection::None,
        }
    }
}

impl<T> Intersection<GeometryCollection<T>> for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &GeometryCollection<T>) -> Self::Output {
        match rhs.intersection(self) {
            GeometryCollectionIntersection::Point(p) => PointIntersection::Point(p.clone()),
            _ => PointIntersection::None,
        }
    }
}

impl<T> Intersection<Geometry<T>> for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &Geometry<T>) -> Self::Output {
        match rhs.intersection(self) {
            GeometryIntersection::Point(p) => PointIntersection::Point(p.clone()),
            _ => PointIntersection::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{Geometry, Coordinate, Line, Point};
    use super::*;

    #[test]
    fn with_point() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = p1.clone();

        assert_eq!(p1.intersection(&p2), PointIntersection::Point(p1.clone()));

        // Not intersecting
        let p2 = Point::new(2.0, 2.0);

        assert_eq!(p1.intersection(&p2), PointIntersection::None)
    }

    #[test]
    fn with_line() {
        let line = build_line![(0.0, 0.0), (10.0, 0.0)];

        // Start of the line
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            point.intersection(&line),
            PointIntersection::Point(point.clone())
        );

        // On the line
        let point = Point::new(5.0, 0.0);

        assert_eq!(
            point.intersection(&line),
            PointIntersection::Point(point.clone())
        );

        // End of the line
        let point = Point::new(10.0, 0.0);

        assert_eq!(
            point.intersection(&line),
            PointIntersection::Point(point.clone())
        );

        // Not on the line
        let point = Point::new(1.0, 1.0);

        assert_eq!(
            point.intersection(&line),
            PointIntersection::None
        );
    }

    #[test]
    fn with_linestring() {
        let line_string = line_string![(0.0, 0.0), (10.0, 0.0)];

        // Start of LineString
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            point.intersection(&line_string),
            PointIntersection::Point(point.clone())
        );

        // On of LineString
        let point = Point::new(5.0, 0.0);

        assert_eq!(
            point.intersection(&line_string),
            PointIntersection::Point(point.clone())
        );

        // End of LineString
        let point = Point::new(10.0, 0.0);

        assert_eq!(
            point.intersection(&line_string),
            PointIntersection::Point(point.clone())
        );

        // Not on LineString
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            point.intersection(&line_string),
            PointIntersection::None,
        );
    }

    #[test]
    fn with_polygon() {
        let exterior = line_string![(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)];
        let interiors = vec![
            line_string![(0.1, 0.1), (0.9, 0.9), (0.9, 0.1), (0.1, 0.1)],
        ];
        let polygon = Polygon::new(exterior, interiors);

        // Start of Polygon exterior
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            point.intersection(&polygon),
            PointIntersection::Point(point.clone())
        );

        // On Polygon exterior
        let point = Point::new(0.5, 0.0);

        assert_eq!(
            point.intersection(&polygon),
            PointIntersection::Point(point.clone())
        );

        // Start of Polygon interior
        let point = Point::new(0.1, 0.1);

        assert_eq!(
            point.intersection(&polygon),
            PointIntersection::Point(point.clone())
        );

        // On Polygon interior
        let point = Point::new(0.9, 0.4);

        assert_eq!(
            point.intersection(&polygon),
            PointIntersection::Point(point.clone())
        );

        // In Polygon interior
        let point = Point::new(0.05, 0.05);

        assert_eq!(
            point.intersection(&polygon),
            PointIntersection::Point(point.clone())
        );

        // Not in or on Polygon
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            point.intersection(&polygon),
            PointIntersection::None,
        );
    }

    #[test]
    fn with_multipoint() {
        let multi_point = multi_point!((0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0));

        // Point on MultiPoint
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            point.intersection(&multi_point),
            PointIntersection::Point(point.clone())
        );

        // Not in or on MultiPoint
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            point.intersection(&multi_point),
            PointIntersection::None,
        );
    }

    #[test]
    fn with_multilinestring() {
        let line_string1 = line_string![(0.0, 0.0), (10.0, 0.0)];
        let line_string2 = line_string![(100.0, 100.0), (200.0, 100.0)];
        let mls = MultiLineString(vec![line_string1, line_string2]);

        // Start of first LineString in MultiLineString
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            point.intersection(&mls),
            PointIntersection::Point(point.clone())
        );

        // On first LineString in MultiLineString
        let point = Point::new(5.0, 0.0);

        assert_eq!(
            point.intersection(&mls),
            PointIntersection::Point(point.clone())
        );

        // On second LineString in MultiLineString
        let point = Point::new(150.0, 100.0);

        assert_eq!(
            point.intersection(&mls),
            PointIntersection::Point(point.clone())
        );

        // Not on MultiLineString
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            point.intersection(&mls),
            PointIntersection::None,
        );
    }

    #[test]
    fn with_multipolygon() {
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
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        // On Polygon exterior
        let point = Point::new(0.5, 0.0);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        // Start of first Polygon interior
        let point = Point::new(0.1, 0.1);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        // On first Polygon interior
        let point = Point::new(0.9, 0.4);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        // In first Polygon interior
        let point = Point::new(0.05, 0.05);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        //-----------------------------------------------------------------------------------------
        // SECOND POLYGON
        //-----------------------------------------------------------------------------------------
        // Start of second Polygon exterior
        let point = Point::new(100.0, 100.0);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        // On Polygon exterior
        let point = Point::new(100.5, 100.0);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        // Start of second Polygon interior
        let point = Point::new(100.1, 100.1);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        // On second Polygon interior
        let point = Point::new(100.9, 100.4);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        // In second Polygon interior
        let point = Point::new(100.05, 100.05);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::Point(point.clone())
        );

        //-----------------------------------------------------------------------------------------
        // Not in or on Polygon
        //-----------------------------------------------------------------------------------------
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            point.intersection(&multi_polygon),
            PointIntersection::None,
        );
    }

    #[test]
    fn geometrycollection_with_point() {
        let mut geometries: Vec<Geometry<f32>> = vec![];

        geometries.push(Geometry::Point(Point::new(180.0, 180.0)));
        geometries.push(Geometry::Line(build_line![(179.0, 179.0), (178.0, 179.0)]));
        geometries.push(Geometry::LineString(line_string![(170.0, 170.0), (171.0, 170.0)]));

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

        geometries.push(Geometry::MultiPolygon(MultiPolygon(vec![polygon1, polygon2])));

        let geometry_collection = GeometryCollection(geometries);

        //-----------------------------------------------------------------------------------------
        // FIRST POLYGON
        //-----------------------------------------------------------------------------------------
        // Start of first Polygon exterior
        let point = Point::new(0.0, 0.0);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        // On Polygon exterior
        let point = Point::new(0.5, 0.0);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        // Start of first Polygon interior
        let point = Point::new(0.1, 0.1);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        // On first Polygon interior
        let point = Point::new(0.9, 0.4);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        // In first Polygon interior
        let point = Point::new(0.05, 0.05);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        //-----------------------------------------------------------------------------------------
        // SECOND POLYGON
        //-----------------------------------------------------------------------------------------
        // Start of second Polygon exterior
        let point = Point::new(100.0, 100.0);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        // On Polygon exterior
        let point = Point::new(100.5, 100.0);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        // Start of second Polygon interior
        let point = Point::new(100.1, 100.1);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        // On second Polygon interior
        let point = Point::new(100.9, 100.4);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        // In second Polygon interior
        let point = Point::new(100.05, 100.05);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::Point(point.clone())
        );

        //-----------------------------------------------------------------------------------------
        // Not in or on Polygon
        //-----------------------------------------------------------------------------------------
        let point = Point::new(10.0, 10.0);

        assert_eq!(
            point.intersection(&geometry_collection),
            PointIntersection::None,
        );
    }

    #[test]
    fn geometry_point_with_point() {
        let p1 = Point::new(180.0, 180.0);
        let p2 = p1.clone();
        let geometry_point = Geometry::Point(p1);

        assert_eq!(geometry_point.intersection(&p2), GeometryIntersection::Point(p1.clone()));

        // Not intersecting
        let p2 = Point::new(2.0, 2.0);

        assert_eq!(p2.intersection(&geometry_point), PointIntersection::None)
    }
}
