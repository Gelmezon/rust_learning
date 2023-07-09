fn main() {
    let dire = Direction::South;

    match dire {
        Direction::East => println!("East"),
        Direction::West | Direction::North => println!("West AND North"),
        _ => println!("South"),
    }
}

enum Direction{
    East,
    West,
    North,
    South,
}

