fn main() {
    let s = String::from("Hello, world");

    println!("The number of ls is {}", count_l(&s));

    for c in s.chars() {
        println!("{}", c);
    }

    for c in s.bytes() {
        println!("{}", c);
    }

    for (i, c) in s.chars().enumerate() {
        println!("{} = {}", c, i);
    }
}

fn count_l(s: &str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res += 1;
        }
    }
    return res; 
}