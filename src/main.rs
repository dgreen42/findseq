use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    
    let code = env::args().nth(1).expect("Please enter a search pattern");
    let path = env::args().nth(2).expect("Please enter a valid path");
    let option = env::args().nth(3).expect("Please enter an option");

    if code == String::from("help") {
	println!("findseq help");
    } else {
	if option == String::from("-v") {
	    if let Ok(lines) = read_lines(&path) {
		for line in lines {
		    search_verbose(line.expect("Could not read line"), code.clone());
		}
	    }
	}
	if option == String::from("-m") {
	    if let Ok(lines) = read_lines(&path) {
		for line in lines {
		    search_minimal(line.expect("Could not read line"), code.clone());
		}
	    }
	}	
    }
}

fn read_lines <P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn search_verbose(line: String, pattern: String) {
    let line = line.to_lowercase();
    let pattern = pattern.to_lowercase();
    let bline = line.as_bytes();
    let bpattern = pattern.as_bytes();

    for i in 0..bline.len() - bpattern.len() {
	if bpattern == &bline[i..i + bpattern.len()] {
	    println!("There is a match at character {}", i)
	}
    }
}

fn search_minimal(line: String, pattern: String) {
    let line = line.to_lowercase();
    let pattern = pattern.to_lowercase();
    let bline = line.as_bytes();
    let bpattern = pattern.as_bytes();

    for i in 0..bline.len() - bpattern.len() {
	if bpattern == &bline[i..i + bpattern.len()] {
	    println!("There is a match at character {}", i)
	}
    }
}
