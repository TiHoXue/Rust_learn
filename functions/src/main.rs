fn main() {
    println!("Hello, world!");

    another_function();

    print_labeled_measurement(10, 'm');

    let res = plus_one(1);
    println!("The value of res is: {res}");
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
