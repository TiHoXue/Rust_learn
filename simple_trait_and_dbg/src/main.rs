#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    const FACTOR: u32 = 3;
    let rect = Rectangle {
        width: dbg!(50 * FACTOR),
        height: dbg!(120 * FACTOR),
    };

    println!("rect is {rect:?}");
}
