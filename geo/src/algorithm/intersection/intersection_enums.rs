use geo_types::Point;
use num_traits::Float;

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

/// Describes the possible outcomes of intersecting a MultiPoint with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum MultiPointIntersection<T: Float> {
    Point(Point<T>),
    None
}

/// Describes the possible outcomes of intersecting a MultiLineString with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum MultiLineStringIntersection<T: Float> {
    Point(Point<T>),
    None
}

/// Describes the possible outcomes of intersecting a MultiPolygon with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum MultiPolygonIntersection<T: Float> {
    Point(Point<T>),
    None
}

/// Describes the possible outcomes of intersecting a GeometryCollection with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum GeometryCollectionIntersection<T: Float> {
    Point(Point<T>),
    None
}

/// Describes the possible outcomes of intersecting a Geometry with another Geometry.
///
#[derive(Debug, PartialEq)]
pub enum GeometryIntersection<T: Float> {
    Point(Point<T>),
    None
}
