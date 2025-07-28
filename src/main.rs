fn main() {
    enums();
    enums_with_data();
    manage_user();
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
    let shape = Shape::Circle(20.00);
    println!("The area of your circle is {}", shape.print_shape_area());

    let shape = Shape::Rectangle(20.00, 10.00);
    println!("The area of your rectangle is {}", shape.print_shape_area());

    let shape = Shape::Triangle(20.00, 10.00, 10.00);
    println!("The area of your triangle is {}", shape.print_shape_area())
}

// Classes (haha - structs really)

struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) -> String {
        format!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        )
    }

    fn have_birthday(&mut self) {
        self.age += 1;
        println!(
            "Happy birthday, {}! You are now {} years old.",
            self.name, self.age
        );
    }
}

fn manage_user() {
    let user = User::new("Johnpaul", 30);
    println!("{}", user.greet());

    let mut birthday_user = User::new("Johnpaul", 30);
    birthday_user.have_birthday();

    println!("{}", birthday_user.greet());
}
