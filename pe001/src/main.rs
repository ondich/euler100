// Project Euler, Problem 1
// Multiples of 3 and 5

fn main() {
    let mut sum = 0;
    for k in 1..1000 {
        if k % 3 == 0 || k % 5 == 0 {
            sum = sum + k
        }
    }
    println!("Answer: {}", sum)
}
