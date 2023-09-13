use day3::modules::{input::file_in, items::find_duplicate_on_second_half};

fn main() {
    
    let file_contents = file_in();
    
    let result = file_contents
        .lines()
        .map(|line| {

            if !line.is_ascii() {
                panic!("Invalid characters at: {}", line);

            }

            let bytes = line.as_bytes();
            let dup = find_duplicate_on_second_half( bytes );

            if dup.is_none() {
                panic!("It seems there's o duplicates on the second half of: {}", line);
            }

            let char_value = *dup.unwrap();
            let priority = match char_value {
                b'A'..=b'Z' => char_value - b'A' + 27,
                b'a'..=b'z' => char_value - b'a' + 1,
                _ => 0
            };

            priority as u32

        })
        .sum::<u32>();

    println!("Priority is: {result}")



}
