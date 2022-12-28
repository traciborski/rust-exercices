fn main() {
    match 'x' {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        _ => println!("Something else"),
    }

    let pair: (i32, i32) = (2, -2);
    match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x.abs() == y.abs() => println!("abs"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }
}
