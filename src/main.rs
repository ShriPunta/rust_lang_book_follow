use std::io;
fn main() {
    let mut temp = String::new();
    println!("Please give the current temperature in Celsius ");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp: f32 = temp.trim().parse().expect("PANIKK!");
    
    println!("Please give the unit of output C,K,F ");

    let mut scale_name = String::new();
    io::stdin()
        .read_line(&mut scale_name)
        .expect("Failed to read line");
    let scale_name: char = scale_name.trim().parse().expect("PANIKK!");
    match scale_name {
        'C' => println!("{} deg C", ((temp - 32.0) / 9.0) * 5.0),
        'F' => println!("{} deg F", ((temp * 9.0) / 5.0) + 32.0),
        _ => println!("No any know")
    }
}
