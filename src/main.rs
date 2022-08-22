use std::{fs, io, time};
use rand::{thread_rng, Rng};

fn main() {
    let contents = fs::read_to_string("1-1000.txt")
        .expect("could not read the file");
    
    let word_list: Vec<&str> = contents.split('\n').collect();

    let mut current_words: Vec<&str> = Vec::new();

    for _i in 0..50 {
    
        let index = thread_rng().gen_range(0..1000);
        current_words.push(word_list[index]);

    }

    let mut another_index = 0;

    loop {
        
        let displayed_word = current_words[another_index];

        println!("{displayed_word}");
        
        let mut input = String::new();
    
        io::stdin().read_line(&mut input)
            .expect("Could not read user input");

        let real_input: Vec<&str> = input.split('\r').collect();

        if real_input[0] == displayed_word {
            another_index += 1;
        } else {
            println!("So Wrong!");
        }
    }

}
