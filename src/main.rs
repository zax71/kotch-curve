use kotch::generate_koch;
use vector2d::Vector2D;
mod kotch;

fn main() {
    println!("Hello, world!");

    let line = generate_koch(Vector2D::new(5.0, 5.0), Vector2D::new(6.0, 3.0), 1);

    println!("{:#?}", line)
}
