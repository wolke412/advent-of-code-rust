use core::panic;
use std::fs;


fn main() {

    let file_contents = fs::read_to_string("static/values.txt").expect("Error reading file.");
    let mut greater = 0;

    file_contents
        .lines()
        .into_iter( )
        .fold(0, | acc, curr | {

            if curr.is_empty() {

                if acc > greater {
                    greater = acc;
                }

                return 0;
            }

            if let Ok(i) = curr.parse::<i32>() {
                return acc + i;
            } 

            panic!("The file contains non-numeric data.");

        });  

    println!("Greater is: {greater}");

}
