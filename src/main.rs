use kotch::generate_koch;
use svg::generate_svg;
use vector2d::Vector2D;
mod kotch;
mod svg;

fn main() {
    println!("Hello, world!");

    let line = generate_koch(Vector2D::new(0.0, 50.0), Vector2D::new(100.0, 50.0), 3);

    let svg = generate_svg(line);

    println!("{}", svg)
}
