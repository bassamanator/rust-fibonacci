use std::io;

fn main() {
    println!("Which Fibonacci number would you like to see to?");

    print!("[Enter index:] "); // User's input should be typed at the end of this prompt (not on the next line)

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
        Err(err) => {
            println!("Halt! Not a number.");
            return;
        }
    };
    let n = n + 1;
    println!("...");

    let mut ans: Vec<u32> = Vec::new();

    for i in 0..n {
        let fi = fib(i);
        ans.push(fi);
        println!("Fibonacci[{i}] = {}", fi);
    }
    println!();

    let ans: String = ans
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    println!("{}", ans);
}

fn fib(i: u32) -> u32 {
    if i <= 1 {
        return i;
    }
    return fib(i - 1) + fib(i - 2);
}
