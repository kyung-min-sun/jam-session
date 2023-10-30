fn main() {
    let x = 1 + 1;
    print!("{x}");
    println!("Hello, world!");
    let help = yo();
    println!("{help} this is pretty cool omg");
    println!("testing");
    let value = function_that_returns_things();
    println!("{value}")
}

fn function_that_returns_things() -> i32 {
    return 2;
}

fn yo() -> &'static str {
    return "help";
}
