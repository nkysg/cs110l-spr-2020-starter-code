use std::env;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::io::BufReader;
use std::fmt;

#[derive(Debug)]
struct MyError(String);

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

#[allow(dead_code)]

impl Error for MyError {}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        return Err(Box::new(MyError("Too few arguments.".into())));
    }
    let filename = &args[1];

    // Your code here :)
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    
    let lines_num = contents.lines().collect::<Vec<&str>>().len();
    let words_num = contents.split_whitespace().collect::<Vec<&str>>().len();
    let chars_num = contents.len();
    println!("{} {} {} {}", lines_num, words_num, chars_num, filename);

    Ok(())

}