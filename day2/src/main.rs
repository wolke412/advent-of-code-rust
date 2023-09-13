use std::io::Lines;

use day2::modules::{input::file_in, pairs::{self, Pair}};

fn main() {
    
    let file_contents = file_in();
    
    let result = file_contents
        .lines()
        .map(|line| {
            let chars: Vec<char> = line
                .chars()
                .filter(|ch| {
                    ch.is_alphabetic()
                })
                .collect();

            if chars.len() != 2 {
                panic!("Error in file at line {}", line);
            }
            
            let p = Pair::new( chars[0], chars[1] );


            p.result()
        })
        .sum::<u8>();

    println!("Resultado final: {result}");



}
