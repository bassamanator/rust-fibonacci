use std::io::{self, Write};

fn main() {
    println!("Which Fibonacci number would you like to see to?");

    print!("[Enter index:] ");
    io::stdout().flush().expect("Erroring flushing");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => {
            if num > 0 {
                num
            } else {
                println!("Halt! Not a positive integer value.");
                return;
            }
        }
        Err(_) => {
            println!("Halt! Not a number.");
            return;
        }
    };
    let n = n + 1;

    let mut ans: Vec<u32> = Vec::new();

    for i in 0..n {
        let fi = fib(i);
        ans.push(fi);
    }

    println!("\n{:?}", ans);
}

fn fib(i: u32) -> u32 {
    if i <= 1 {
        return i;
    }
    return fib(i - 1) + fib(i - 2);
}
