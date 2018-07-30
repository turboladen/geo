use algorithm::intersects::Intersects;
use num_traits::Float;
use geo_types::{Line, LineString, Point};

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
        if self.lines().any(|line| line.intersects(rhs)) {
            LineStringIntersection::Point(rhs.clone())
        } else {
            LineStringIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{Coordinate, Line, Point};
    use super::*;

    #[test]
    fn point_with_intersecting_point() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = p1.clone();

        assert_eq!(p1.intersection(&p2), PointIntersection::Point(p1.clone()));
    }

    #[test]
    fn point_with_no_intersecting_point() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 2.0);

        assert_eq!(p1.intersection(&p2), PointIntersection::None)
    }

    #[test]
    fn line_with_intersecting_point() {
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
    fn linestring_with_intersecting_point() {
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
}
