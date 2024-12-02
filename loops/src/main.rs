fn main() {
    // loop can return value
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result: {result}");

    // loop label
    'loop1: loop {
        println!("loop1 start.");

        loop {
            println!("Inner loop start.");
            break 'loop1;
            // println!("Inner loop end.");
        }

        // println!("loop1 end.");
    }

    // while
    let mut stop_while = 3;
    while stop_while > 0 {
        println!("stop_while: {stop_while}");
        stop_while -= 1;
    }

    // for
    for n in 1..10 {
        print!("{n} ");
    }
}
