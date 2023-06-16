#[derive(Debug)]
// Enum Definition
enum Color {
    Red,
    Green,
    Blue,
}

// Enums with Data
enum Shape { 
    Circle(f64), 
    Rectangle(f64, f64), 
    Triangle(f64, f64, f64),
}

// Enum Pattern Matching
fn print_area(shape: Shape) { 
    match shape {
        Shape::Circle(radius) => {
        let area = std::f64::consts::PI * radius * radius; 
        println!("Area of the circle: {}", area);
        }
        Shape::Rectangle(length, width) => {
        let area = length * width;
        println!("Area of the rectangle: {}", area); 
        }
        Shape::Triangle(side1, side2, side3) => {
        // Calculate area using Heron's formula
        let s = (side1 + side2 + side3) / 2.0;
        let area = (s * (s - side1) * (s - side2) * (s - side3)).sqrt(); 
        println!("Area of the triangle: {}", area);
        } 
    }
}

fn main() {
    // Enum Valuse
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;
    println!("{:?}", red);
    println!("{:?}", green);
    println!("{:?}", blue);

    print_area(Shape::Circle(2.0));
    print_area(Shape::Rectangle(2.0, 2.0));
    print_area(Shape::Triangle(2.0, 2.0, 2.0));
    
}
