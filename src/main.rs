use std::{fs::File, io::{ErrorKind, Read}};
use std::io::{self};
// Standard library enum: Option
fn main() {
    let greeting_file_result = File::open("Jello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound{
            File::create("Jello.txt").unwrap_or_else(|err| {
                panic!("Problem creating a file: {:?}", err)
            })
        }else{
            panic!("Problem opening the file: {:?}", err)
        }
    });

}

fn read_username_from_file() -> Result<String, io::Error>{
    //let uname_file_res = File::open("hello.txt");
    let mut uname_file = File::open("hello.txt")?;

    //"?" works like if null return operator

    // let mut uname_file = match uname_file_res{
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut uname = String::new();
    uname_file.read_to_string(&mut uname)?;
    Ok(uname)
    // match uname_file.read_to_string(&mut uname){
    //     Ok(_) => Ok(uname),
    //     Err(e) => Err(e),
    // }
}
