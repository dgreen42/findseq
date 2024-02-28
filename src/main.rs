use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;

fn main() {
    env::set_var("RUST_BACKTRACE", "0");

    let code = env::args().nth(1).expect("Please enter a search pattern");
    if code == "help" {
        println!("findseq help");
    } else {
        let path = env::args().nth(2).expect("Please enter a valid path");
        let option = env::args().nth(3).expect("Please enter an option");
        let read = read_lines(&path).expect("could not read file");
        let count = read
            .filter(|x| x.as_ref().expect("no").starts_with('>'))
            .count();
        if count > 1 {
            let hashed_fasta = read_multplie_fasta(path);
            parse_hash(hashed_fasta, option, code);
        } else {
            let hashed_fasta = read_single_fasta(path);
            println!("{:?}", hashed_fasta.values());
            parse_hash(hashed_fasta, option, code);
        };
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_hash(hash: HashMap<String, String>, option: String, code: String) {
    for (id, seq) in hash.iter() {
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

fn read_multplie_fasta<P>(filename: P) -> HashMap<String, String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not read file");
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
            curid = line[..].trim().to_string();
        } else {
            curseq.push_str(line.trim());
        }
    }
    fasta
}

fn read_single_fasta<P>(filename: P) -> HashMap<String, String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("oh");
    let buf = io::BufReader::new(file);
    let mut fasta = HashMap::new();
    let mut id = String::new();
    let mut seq = String::new();

    for line in buf.lines() {
        let line = line.expect("Bad Line");
        if line.starts_with('>') {
            id.push_str(&line);
        }
        seq.push_str(&line);
    }
    fasta.insert(id.clone(), seq.clone());
    fasta
}

fn search_verbose(line: String, pattern: String) {
    let line = line.to_lowercase();
    let pattern = pattern.to_lowercase();
    let bline = line.as_bytes();
    let bpattern = pattern.as_bytes();

    for i in 0..bline.len() - bpattern.len() - 1 {
        if bpattern == &bline[i..i + bpattern.len()] {
            let lflank = &bline[i - 5..i];
            let rflank = &bline[i + bpattern.len()..i + bpattern.len() + 5];
            println!(
                "There is a match at {}\nLeft Flank {}\nRight Flank {}\r\n\r\n",
                i,
                str::from_utf8(lflank).expect("Could not print left flank"),
                str::from_utf8(rflank).expect("Could not print right flank")
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
    println!("No occurances");
}

fn collapse_lines(curline: String) -> String {
    let mut fulline = String::new();
    for ch in curline.chars() {
        fulline.push(ch);
    }
    fulline
}
