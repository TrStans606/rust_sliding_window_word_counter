
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::io;
fn main() {
	let mut three_letter_words:HashMap<String, u32> = HashMap::new();

	let mut file_path = String::new();
	println!("Enter the file path for the sequence you want scanned: ");
	io::stdin().read_line(&mut file_path).expect("Failed to read line");

	println!("The file is :{file_path}");

	let file_path:&str = file_path.trim();

	
	let salmon_genome = fs::read_to_string(file_path).expect("Make sure the sequence.fasta file is in assignment_1 not in target or src");

	let mut edited_salmon_genome = String::from("");

	let salmon_genome_lines = salmon_genome.lines();
	
	for line in salmon_genome_lines {
		let first_char = if let Some(x) = line.chars().next() {
			x
		} else {
			continue
		};
		if first_char != '>' {
			edited_salmon_genome.push_str(line);
		}
	}

	for window in edited_salmon_genome.chars().collect::<Vec<_>>().windows(3) {
		let triplet = window.iter().collect::<String>();
		let value = three_letter_words.entry(triplet).or_insert(0);
		*value += 1;
	}

	let mut results = match fs::File::create("results.txt") {
        Err(why) => panic!("couldn't create results files: {}",why),
        Ok(results) => results,
    };

	for (key, value) in three_letter_words.iter() {
		match writeln!(results,"{}: {}", key, value) {
			Ok(_) => continue,
			Err(e) => eprintln!("Write failed: {}", e),
		}
	}

	for (key, value) in three_letter_words.iter() {
		println!("{}: {}", key, value);
	}
}
