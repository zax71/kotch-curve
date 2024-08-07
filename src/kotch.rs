use vector2d::Vector2D;

/// Used to multiply a Vector2D by an f64.
fn multiply(float: f64, vector: Vector2D<f64>) -> Vector2D<f64> {
    return Vector2D::new(vector.x * float, vector.y * float);
}

/// Adds an equlatteral triangle between point 1 and 2 of an array that has length 4 to make it have
/// length 5
fn draw_triangle(points: &mut Vec<Vector2D<f64>>) -> &mut Vec<Vector2D<f64>> {
    assert_eq!(points.len(), 4);

    let point_1 = points[1];
    let point_2 = points[2];
    let midpoint = (point_1 + point_2) / 2.0;
    let between = point_1 - point_2;
    let direction = Vector2D::new(between.y, -between.x);
    let point_3 = midpoint + multiply((3_f64.sqrt()) / 2.0, direction);

    points.insert(2, point_3);
    return points;
}

/// A recursive function to generate the points of a Kotch curve given two starting points. Returns
/// a Vec of Vector2D to be plotted
pub fn generate_koch(p1: Vector2D<f64>, p2: Vector2D<f64>, depth: u16) -> Vec<Vector2D<f64>> {
    if depth == 0 {
        return None;
    }

    // Split the line and generate the triangle
    let mut line = split_into(p1, p2, 3);

    draw_triangle(&mut line);

    // -1 as this is the sections not the points
    for i in 0..line.len() - 1 {
        // Be careful if point 1 is
        // duplicated. Probably not an issue but good to look out for
        line.insert(i, generate_koch(line[i], line[i + 1], depth - 1))
    }
    return line;
}

/// Splits a set of two points in to a specified number of sections
fn split_into(p1: Vector2D<f64>, p2: Vector2D<f64>, sections: i32) -> Vec<Vector2D<f64>> {
    let mut out_vec = Vec::new();

    // Add the first point
    out_vec.push(p1);

    // Calculate the distance between the points and then divide that by the number of sections to
    // see how much we need to move by, as a vector, for each section
    let change_amount = (p2 - p1) / (sections as f64);

    // Loop through each section and add a point the change amount off of the previous
    for _ in 0..sections {
        println!("splitting");
        out_vec.push(out_vec.last().unwrap() + &change_amount);
    }

    return out_vec;
}
