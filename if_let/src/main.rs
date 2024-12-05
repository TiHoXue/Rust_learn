fn main() {
    let some_ten = Some(10u8);
    if let Some(n) = some_ten {
        println!("There is {n}.");
    } else {
        println!("There is none.");
    }
}
