fn main() {

    /*
     let x = 42;
    let y = 3.14;
    let z = true;
    let ch = 'A';
    */

    type_variable();
    another_function();
    functijon_param("test");
    let sum = sum(10, 15);
    println!("sum {}", sum)
}

fn another_function() {
    println!("Another function.");
}

fn functijon_param(a: &str) {
    println!("function param {}", a)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn type_variable() {
    let x = 5; // immutable variable
    println!("value x is: {}", x);

    let mut y = 5;
    println!("value y is {}", y);
    y = 50;
    println!("value y is {}", y);

    let x = 1;
    println!("value y is {}", x);

    let a: i32 = 10;
    let b = 20;
    println!("Integer  a {} b {}", a, b);

    let a: f32 = 2.0;
    let b = 3.0;
    println!("float a {} b {}", a, b);

    let a: bool = true;
    let b = false;
    println!("bool a {} b {}", a, b);

    let a: char = 'a';
    let b = 'b';
    println!("char a {} b {}", a, b);
}
