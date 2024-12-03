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

    // function with ownership transfer
    let s_print = String::from("Print me!");
    print_string_reference(&s_print);
    println!("After print_string(). {s_print}");

    // Reference
    let s = String::from("string");
    let s_len = calculate_length(&s);
    println!("Length of \"{s}\" is {s_len}");

    let mut number = 42;
    let r1 = &number;
    let r2 = &number;
    println!("r1 and r2: {r1} {r2}");

    let r3 = &mut number;
    *r3 = 777;
    println!("r3: {r3}");

    // slice
    let words = String::from("one two three four five");
    let first_word = first_words(&words);
    // words.clear();
    println!("First word: {first_word}");

    let s = String::from("abcdefghigklmn");
    let part = &s[0..4];
    println!("part: {part}");
}

fn print_string_reference(s: &String) {
    println!("{s}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_words(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
