use vector2d::Vector2D;

pub fn generate_svg(points: Vec<Vector2D<f64>>) -> String {
    let mut out_string: String =
        "<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">\n".to_string();

    // Loop over each SVG line. Fence post problem
    for i in 0..points.len() - 1 {
        out_string.push_str(svg_line(points[i], points[i + 1]).as_str());
    }

    // Close the SVG tag
    out_string.push_str("</svg>");

    return out_string.to_string();
}

fn svg_line(p1: Vector2D<f64>, p2: Vector2D<f64>) -> String {
    return format!(
        "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" vector-effect=\"non-scaling-stroke\"/>\n",
        p1.x, p1.y, p2.x, p2.y
    );
}
