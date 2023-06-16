// Define a trait named 'Drawable' with a single method 'draw'
trait Drawable {
    fn draw(&self);
}

// Implement the 'Drawable' traitfor the 'Circle' struct
struct Circle {
    radius: f64,
}
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

// Using Traits
fn draw_shape (shape: &dyn Drawable) {
    shape.draw();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    draw_shape(&circle);

}