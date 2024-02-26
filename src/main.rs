use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let code = env::args().nth(1).expect("Please enter a search pattern");
    if code == "help" {
        println!("findseq help");
    } else {
        let path = env::args().nth(2).expect("Please enter a valid path");
        let option = env::args().nth(3).expect("Please enter an option");
	let hashed_fasta = read_lines(path);
	
	for (id, seq) in hashed_fasta.iter() {
	    println!("ID: {}", id);
	    let fullseq = collapse_lines(seq.to_string());
	    if option == "-m" {
		search_minimal(fullseq.clone(), code.clone());
	    }
	    if option == "-v" {
		search_verbose(fullseq.clone(), code.clone());
	    }
	}
    }
}

fn read_lines<P>(filename: P) -> HashMap<String, String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("oh");
    let buf = io::BufReader::new(file);
    let mut fasta = HashMap::new();
    let mut curid = String::new();
    let mut curseq = String::new();

    for line in buf.lines() {
        let line = line.expect("Bad Line");
        if line.starts_with('>') {
            if !curid.is_empty() {
                fasta.insert(curid.clone(), curseq.clone());
                curseq.clear();
            }
            curid = line[1..].trim().to_string();
        } else {
            curseq.push_str(&line.trim())
        }
    }
    fasta
}

fn search_verbose(line: String, pattern: String) {
    let line = line.to_lowercase();
    let pattern = pattern.to_lowercase();
    let bline = line.as_bytes();
    let bpattern = pattern.as_bytes();

    for i in 0..bline.len() - bpattern.len() {
        if bpattern == &bline[i..i + bpattern.len()] {
            let lflank = &bline[i - 5..i];
            let rflank = &bline[i + bpattern.len()..i + bpattern.len() + 5];
            println!(
                "There is a match at {}\nLeft Flank {}\nRight Flank {}",
                i,
                str::from_utf8(lflank).expect("Uh"),
                str::from_utf8(rflank).expect("Oh")
            )
        }
    }
}

fn search_minimal(line: String, pattern: String) {
    let line = line.to_lowercase();
    let pattern = pattern.to_lowercase();
    let bline = line.as_bytes();
    let bpattern = pattern.as_bytes();
    let mut count = 0;

    for i in 0..bline.len() - bpattern.len() {
        if bpattern == &bline[i..i + bpattern.len()] {
            count += 1;
        }
    }
    if count >= 1 {
        println!("There are {} occurances", count);
    }
}

fn collapse_lines(curline: String) -> String {
    let mut fulline = String::new();
    for ch in curline.chars() {
        fulline.push(ch);
    }
    fulline
}
