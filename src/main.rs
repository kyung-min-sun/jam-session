fn main() {
    let x = 1 + 1;
    print!("{x}");
    println!("Hello, world!");
    let help = yo();
    println!("{help} this is pretty cool omg");
    println!("testing")
}

fn yo() -> &'static str {
    return "help";
}
