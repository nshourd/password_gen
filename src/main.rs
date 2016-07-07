extern crate rand;

use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rand::Rng;


fn main() {
    let mut length = String::new();    
    
    println!("How long do you want your password to be?");
    
    //reads the user input and stores it in a mutable string length
    io::stdin().read_line(&mut length)
        .expect("failed to read line");

    //converts the length string into a number
    let length: u32 = length.trim().parse()
        .expect("Please type a number!");

    let possible_chars = "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz!*?_";
    let mut password = String::new();
   
    //in java for(i = 0; i <= length; i++), _ means that the iterator value is to be discarded
    for _ in 0..length {
        //setting the index to a random number between 0 and the length of the possible characters
        //string
        let index = rand::thread_rng().gen_range(0, possible_chars.len());

        //adds the character at the index position to password string
        password.push_str(&possible_chars[index..index + 1]);
    }
    
    let path = Path::new("password.txt");
    let display = path.display();

    //Open a file in write-only mode, returns 'io::Result<File>'
    let mut file = match File::create(&path) {
        Err(why) => { 
            panic!("couldn't create {}: {}", display, why.description())
        },
        Ok(file) => file,
    };

    //Write password string to file
    match file.write_all(password.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                    why.description())
        },
        Ok(_) => println!("Successfully wrote to {}", display), 
    }
    
    println!("It is recommended that you rename the file to identify what the password is for");
}