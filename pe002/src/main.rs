// Project Euler, Problem 2
// Even Fibonacci numbers

fn main() {
    let mut sum = 0;
    let mut f1 = 1;
    let mut f2 = 2;
    while f2 < 4000000 {
        if f2 % 2 == 0 {
            sum = sum + f2;
        }
        let tmp = f2;
        f2 = f2 + f1;
        f1 = tmp;
    }
    println!("Answer: {}", sum)
}
