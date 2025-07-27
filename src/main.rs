fn main() {
    enums();
    enums_with_data()
}

fn enums() {
    let direction = Direction::East;
    println!("This direction is: {}", direction.say_direction());

    let direction = Direction::West;
    println!("This direction is: {}", direction.say_direction());

    let direction = Direction::North;
    println!("This direction is: {}", direction.say_direction());

    let direction = Direction::South;
    println!("This direction is: {}", direction.say_direction());
}

enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn say_direction(&self) -> &str {
        match self {
            Direction::North => "Up",
            Direction::East => "Right",
            Direction::West => "Left",
            Direction::South => "Down",
        }
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn print_shape_area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => radius * 3.14 * radius,
            Shape::Rectangle(length, bredth) => length * bredth,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                return s;
            }
        }
    }
}

fn enums_with_data() {
    let circle = Shape::Circle(20.00);
    println!("The area of your circle is {}", circle.print_shape_area());

    let rectangle = Shape::Rectangle(20.00, 10.00);
    println!(
        "The area of your rectangle is {}",
        rectangle.print_shape_area()
    );

    let triangle = Shape::Triangle(20.00, 10.00, 10.00);
    println!(
        "The area of your triangle is {}",
        triangle.print_shape_area()
    )
}
