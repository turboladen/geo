use num_traits::Float;
use geo_types::{Line, Point};

/// Describes the possible outcomes of intersecting a Point with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum PointIntersection<T: Float> {
    Point(Point<T>),
    None
}

/// Describes the possible outcomes of intersecting a Point with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum LineIntersection<T: Float> {
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

impl<T> Intersection<Point<T>> for Line<T> where T: Float {
    type Output = LineIntersection<T>;

    fn intersection(&self, rhs: &Point<T>) -> Self::Output {
        if [self.start, self.end].contains(&rhs.0) {
            LineIntersection::Point(rhs.clone())
        } else {
            LineIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{Coordinate, Line, Point};
    use super::{Intersection, LineIntersection, PointIntersection};

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
            Coordinate { x: 1.0, y: 1.0 },
            Coordinate { x: 2.0, y: 2.0 },
        );

        let point = Point::new(1.0, 1.0);

        assert_eq!(
            line.intersection(&point),
            LineIntersection::Point(point.clone())
        );

        let point = Point::new(2.0, 2.0);

        assert_eq!(
            line.intersection(&point),
            LineIntersection::Point(point.clone())
        );
    }

    #[test]
    fn line_with_no_intersecting_point() {
        let line = Line::new(
            Coordinate { x: 10.0, y: 10.0 },
            Coordinate { x: 20.0, y: 10.0 },
        );

        let point = Point::new(1.0, 1.0);

        assert_eq!(
            line.intersection(&point),
            LineIntersection::None
        );
    }
}
