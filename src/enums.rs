enum Direction {
    Left,
    Right,
}

pub fn which_way(){
    let go = Direction::Left;
    match go {
        Direction::Left => println!("left"),
        Direction::Right => println!("right")
    }
}
