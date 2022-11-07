use std::io;
fn main() {
    let mut enter_string = String::new();
    println!("Please give input ");
    io::stdin()
    .read_line(&mut enter_string)
    .expect("Failed to read line");

    println!("You entered: {enter_string}");
    println!("First word: {}",first_word(&enter_string));
}
// return the index of the first word ending
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[0..i];
        }
    }

    &s[..]
}
