// Main file to test stuff in

mod color;

fn main() {
    let red = color::Color::newBase(1.0, 0.0, 0.0, 1.0);

    println!("{:?}", red.r());
}
