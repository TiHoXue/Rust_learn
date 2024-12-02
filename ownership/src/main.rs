fn main() {
    // Ownership
    {
        let s = String::from("Hello");
        println!("{s}");
    }

    // move
    let hello = String::from("hello");
    // let hello_again = hello;
    let hello_again = hello.clone();
    println!("hello: {hello}, hello_again: {hello_again}");
}
