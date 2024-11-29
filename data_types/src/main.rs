fn main() {
    let number_with_underscore = 1_000;
    let hex_number = 0xABCD;
    let oct_number = 0o737;
    let binary_number = 0b110101010;
    println!("1_000 is {number_with_underscore}");
    println!("0xABCD is {hex_number}");
    println!("0o737 is {oct_number}");
    println!("0b110101010 is {binary_number}");

    // tuple
    let tup: (i32, f64, char) = (7856, 32.844, '🔥');
    let f = tup.1;
    println!("tup's second element is {f}");
    let (x, y, z) = tup;
    println!("destructure tup: ({x}, {y}, {z})");

    // array
    let fibonaci = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let first_elem = fibonaci[0];
    println!("fibonaci's first elemnet: {first_elem}");

    let many_zeros: [u32; 100] = [0; 100];
    let get_panic = many_zeros[1000];
    println!("many_zeros[1000] = {get_panic}");
}
