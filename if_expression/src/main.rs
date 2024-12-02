fn main() {
    let x = 100;

    if x < 100 {
        println!("Small number.");
    } else if x == 100 {
        println!("100!");
    } else {
        println!("Big number.");
    }

    let condition = true;
    let y = if condition { 1 } else { 2 };
    println!("The value of y is {y}");
}
