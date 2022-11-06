use std::io;
fn main() {
    let mut range_fibo = String::new();
    println!("Please give number of digits expected ");
    io::stdin()
    .read_line(&mut range_fibo)
    .expect("Failed to read line");
    let range_fibo: u32 = range_fibo.trim().parse().expect("PANIKK!");
    println!("{}", fibo_gen(range_fibo));
}

fn fibo_gen(x:u32)->u32 {
    match x {
        0 => return 1,
        1 => return 1,
        _ => {
            let s = fibo_gen(x-1)+ fibo_gen(x-2);
            print!("{s}..");
            return s;
        }
    }
}