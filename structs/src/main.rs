struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("John Smith"),
        email: String::from("john-smith@mail.com"),
        sign_in_count: 0,
    };
    user1.email = String::from("john-smith@gmail.com");
    let user1_username = &user1.username;
    let user1_email = &user1.email;
    println!("user1's name: {user1_username}, user1's email: {user1_email}");

    let user2 = build_user(String::from("Jack"), String::from("jack@me.cn"));
    let user2_username = &user2.username;
    let user2_email = &user2.email;
    println!("user2's name: {user2_username}, user2's email: {user2_email}");

    let user3 = User {
        email: String::from("jack@you.com"),
        ..user2
    };
    let user3_active = &user3.active;
    let user3_username = &user3.username;
    let user3_email = &user3.email;
    let user3_sign_in_count = &user3.sign_in_count;
    println!("user3's name: {user3_username}, user3's email: {user3_email}, user3's active: {user3_active}, user3's sign in count: {user3_sign_in_count}");

    // tuple struct
    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0);
    let x = origin.0;
    let y = origin.1;
    let z = origin.2;
    // let (x, y, z) = origin;
    println!("origin point: ({x}, {y}, {z})");

    // unit-like structs
    struct Unitlike;
    let instance = Unitlike;

}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
