fn main() {
    // Variable and mutabable variable
    println!("Variable and mutabable variable");
    let x = 5;
    println!("x = {x}");
    // x = 6; compile-error
    let mut y = 42;
    println!("y = {y}");
    y = 100; // Yes
    println!("y = {y} now");

    println!();

    // constant
    println!("Constant value");
    const A_DAY_IN_SECONDS: u32 = 60 * 60 * 24;
    println!("There are {A_DAY_IN_SECONDS} seconds of a day.");

    println!();

    // Shadowing
    println!("Shadowing variable");
    let shadow_variable = 1;
    println!("shadow_variable: {shadow_variable}");
    let shadow_variable = 2;
    println!("shadow_variable: {shadow_variable}");
    let shadow_variable = "Happy!";
    println!("shadow_variable: {shadow_variable}");

    let shadow_variable = 1;
    println!("shadow_variable in the outer scope is {shadow_variable}");
    {
        let shadow_variable = shadow_variable * 10;
        println!("shadow_variable in the inner scope is {shadow_variable}");
    }
    println!("shadow_variable in the outer scope is {shadow_variable}");
}
