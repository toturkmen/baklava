fn main() {
    for y in 0..=10 {
        print!("{}", " ".repeat(10 - y));
        println!("{}", "*".repeat(y * 2 + 1));
    }
    for y in (0..10).rev() {
        print!("{}", " ".repeat(10 - y));
        println!("{}", "*".repeat(y * 2 + 1));
    }
}
