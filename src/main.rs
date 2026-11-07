use std::env;
use rand::RngExt;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn generate_sltext(text: &String, percent: &f32, file_name: &String) -> String {
    let mut letters: Vec<char> = text.chars().collect();

    if letters.is_empty() {
        return Default::default();
    }

    let file = File::open(file_name).expect("Cannot open file");
    let reader = BufReader::new(file);

    let dictionary: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    if dictionary.is_empty() {
        println!("Dictionary is empty!");
        return Default::default();
    }

    let mut index_slt: Vec<usize> = Vec::new();

    let num_letters2 = (letters.len() as f32 * percent) as usize;

    let mut rng = rand::rng();

    for _ in 0..num_letters2 {
        let random_index: usize = loop {
            let index = rng.random_range(0..letters.len());

            if !index_slt.contains(&index) {
                break index;
            }
        };

        index_slt.push(random_index);

        for line in &dictionary {
            let line_vec: Vec<char> = line.chars().collect();
            if letters[random_index] == line_vec[0] {
                letters[random_index] = line_vec[1];
                //println!("{} {}", letters[random_index], line_vec[1]);
                break;
            }
        }
    }
    let text_returning: String = letters.iter().collect();
    return text_returning;
}
fn main() {
   let args: Vec<String> = env::args().collect();

    if args.len() < 4 || args.len() > 4 {
        println!("usage: program <text> <number>");
        return;
    }

    let text = &args[1];

    let mut nump = args[2].parse::<f32>().expect("enter correct value");

    let text_file = &args[3];

    if nump>100.0||nump<0.0 { 
        println!("enter correct percent value! from 0 to 100!m")
    }
    nump=nump/100.0;
    println!("{}",generate_sltext(&text,&nump,&text_file));
}
