mod geometry;
mod geometry_collection;
mod intersection_enums;
mod line;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;

/// The trait that defines how intersecting geometries should behave.
///
pub trait Intersection<Rhs = Self> {
    type Output;

    fn intersection(&self, rhs: &Rhs) -> Self::Output;
}
