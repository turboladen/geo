use num_traits::Float;
use geo_types::Point;

/// Describes the possible outcomes of intersecting a Point with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum PointIntersection<T: Float> {
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

    fn intersection(&self, other_point: &Point<T>) -> PointIntersection<T> {
        if self.eq(other_point) {
            PointIntersection::Point(self.clone())
        } else {
            PointIntersection::None
        }
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{Point};
    use super::{Intersection, PointIntersection};

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
}
