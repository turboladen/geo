// Apparently line![] is already taken?
macro_rules! build_line {
    (
        ($x1:expr, $y1:expr), ($x2:expr, $y2:expr)
    ) => {
        Line::new(
            Coordinate { x: $x1, y: $y1 },
            Coordinate { x: $x2, y: $y2 }
        )
    };
}

macro_rules! line_string {
    (
        $(
            ($x:expr, $y:expr)
        ),* $(,)*
    ) => {
        {
            let coordinates = vec![
                $(
                    Coordinate { x: $x, y: $y },
                    )*
            ];

            LineString(coordinates)
        }
    };
}

macro_rules! multi_point {
    (
        $(
            ($x:expr, $y:expr)
        ),* $(,)*
    ) => {
        {
            let points = vec![
                $(
                    Point::new($x, $y),
                    )*
            ];

            MultiPoint(points)
        }
    };
}
