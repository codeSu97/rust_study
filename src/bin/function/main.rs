fn main() {
    println!("{}", plus_or_minus(6));
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

// fn dead_end() -> ! {
//     panic!("panic====")
// }

// fn loop_end() -> ! {
//     loop {}
// }
