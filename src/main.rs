use std::io;

mod ink_calculator;

fn main() {
    let mut calculator = ink_calculator::Calculator {
        wall_area: 0.0,
        ceiling_area: 0.0,
    };

    let height = 2.9;
    let width = read_width();
    let depth = read_depth();

    calculator.calculate_wall_area(height, width, depth);
    calculator.calculate_ceiling_area(width, depth);
    let ink_litring = calculator.calculate_required_literacy();

    println!("The area of walls is: {}", calculator.wall_area);
    println!("The area of ceiling is: {}", calculator.ceiling_area);
    println!("The required paint finish is: {}", ink_litring);
    println!("Finished!");
}

fn read_width() -> f64 {
    println!("How wide is the room?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_depth() -> f64 {
    println!("How deep is the room?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
