enum Message {
    Start,
    Write(String),
    Calculate { number_1: i32, number_2: i32 },
    Stop(i32, i32),
}

impl Message {
    fn call(&self) {
        // method body
    }
}

fn main() {}
