fn main() {
    control_flow();
    type_loop();
    type_match()
}

fn type_match() {
    let number = 5;
    match number {
        1 => println!("{}", number),
        2 => println!("{}", number),
        _ => println!("Something else"),
    }

    let new_number = 10;

    match new_number {
        1 => println!("one"),
        2 | 3| 4| 5 => println!("2 | 3| 4| 5"),
        6..20 => println!("others 6..20"),
        _ => println!("default")
    }
}

fn type_loop() {
    let mut counter = 0;

    loop {
        counter += 1;
        if counter == 10 {
            println!("loop {}", counter);
            break;
        }
    }

    let numbers = [10, 20, 30, 40, 50, 60];

    for element in numbers {
        println!("value for loop {}", element)
    }

    for element in (1..4).rev() {
        println!("{}!", element)
    }
}

fn control_flow() {
    let x = 3;
    let y = 10;

    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if x > 3 && y < 15 {
        println!("x is greater than 3 and y is less than 15");
    }

    if x == 5 || y == 20 {
        println!("Either x is 5 or y is 20");
    }

    let x = true;
    if x {
        println!("number was three");
    }

    if y != 0 {
        println!("number was something other than zero");
    }

    let contition = true;
    let number = if contition { 5 } else { 7 };
    println!("{}!", number)
}