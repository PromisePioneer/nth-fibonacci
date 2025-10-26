fn main() {
    let n = 1000;
    println!("Fibbonaci from {n} is {}", fibonacci(n));
}

fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 0;

    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }

    b

}
