fn main() {
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
