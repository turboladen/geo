use algorithm::intersects::Intersects;
use num_traits::Float;
use geo_types::{Line, LineString, Point, Polygon};

/// Describes the possible outcomes of intersecting a Point with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum PointIntersection<T: Float> {
    Point(Point<T>),
    None
}

/// Describes the possible outcomes of intersecting a Line with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum LineIntersection<T: Float> {
    Point(Point<T>),
    None
}

/// Describes the possible outcomes of intersecting a LineString with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum LineStringIntersection<T: Float> {
    Point(Point<T>),
    None
}

/// Describes the possible outcomes of intersecting a Polygon with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum PolygonIntersection<T: Float> {
    Point(Point<T>),
    None
}

/// The trait that defines how intersecting geometries should behave.
///
pub trait Intersection<Rhs = Self> {
    type Output;

    fn intersection(&self, rhs: &Rhs) -> Self::Output;
}

impl<T> Intersection for Point<T> where T: Float {
    type Output = PointIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if self.eq(rhs) {
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
    use geo_types::{Coordinate, Line, Point};
    use super::*;

    #[test]
    fn point_with_point() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = p1.clone();

        assert_eq!(p1.intersection(&p2), PointIntersection::Point(p1.clone()));

        // Not intersecting
        let p2 = Point::new(2.0, 2.0);

        assert_eq!(p1.intersection(&p2), PointIntersection::None)
    }

    #[test]
    fn line_with_point() {
        let line = Line::new(
            Coordinate { x: 0.0, y: 0.0 },
            Coordinate { x: 10.0, y: 0.0 },
        );

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

    #[test]
    fn linestring_with_point() {
        let line_string: LineString<f32> = vec![(0.0, 0.0), (10.0, 0.0)].into();

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

    #[test]
    fn polygon_with_point() {
        let exterior = LineString(vec![
                                  Coordinate { x: 0.0, y: 0.0 },
                                  Coordinate { x: 1.0, y: 1.0 },
                                  Coordinate { x: 1.0, y: 0.0 },
                                  Coordinate { x: 0.0, y: 0.0 },
        ]);
        let interiors = vec![LineString(vec![
                                        Coordinate { x: 0.1, y: 0.1 },
                                        Coordinate { x: 0.9, y: 0.9 },
                                        Coordinate { x: 0.9, y: 0.1 },
                                        Coordinate { x: 0.1, y: 0.1 },
        ])];
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
