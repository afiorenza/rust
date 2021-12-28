fn main() {
    let mut index: u32 = 0;

    loop {
        println!("{0} -> {1}", index, fibonacci(index));
        index = index + 1;
    }

    fn fibonacci(n: u32) -> u32 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
}
