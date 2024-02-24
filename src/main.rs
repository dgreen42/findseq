use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let code = env::args().nth(1).expect("Please enter a search pattern");
    if code == String::from("help") {
        println!("findseq help");
    } else {
        let path = env::args().nth(2).expect("Please enter a valid path");
        let option = env::args().nth(3).expect("Please enter an option");

        if option == String::from("-v") {
            let mut linum = 0;
            if let Ok(lines) = read_lines(&path) {
                for line in lines {
                    linum += 1;
                    let line = line.expect("uh");
                    if line.starts_with('>') {
                        println!("{}", line)
                    } else {
                        search_verbose(line, code.clone(), linum);
                    }
                }
            }
        }
        if option == String::from("-m") {
            let mut linum = 0;
            if let Ok(lines) = read_lines(&path) {
                for line in lines {
                    linum += 1;
                    let line = line.expect("uh");
                    if line.starts_with('>') {
                        println!("{}", line)
                    } else {
                        search_minimal(line, code.clone(), linum);
                    }
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn search_verbose(line: String, pattern: String, linum: i32) {
    let line = line.to_lowercase();
    let pattern = pattern.to_lowercase();
    let bline = line.as_bytes();
    let bpattern = pattern.as_bytes();

    for i in 0..bline.len() - bpattern.len() {
        if bpattern == &bline[i..i + bpattern.len()] {
            if i < 5 {
                let leftflank = &bline[0..i];
                let rightflank = &bline[i + bpattern.len()..i + bpattern.len() + 5];
                println!("There is a match at character {} in line {}\nFlanked on left by {}\nFlanked on right by {}",
			 i,
			 linum,
			 str::from_utf8(leftflank).expect("Something went wrong with the left flank sequence"),
			 str::from_utf8(rightflank).expect("Something went wrong with the right flank sequence")
		)
            }
            if i + bpattern.len() + 5 > i + bpattern.len() {
                let leftflank = &bline[i - 5..i];
                let rightflank = &bline[i + bpattern.len()..i + bpattern.len()];
                println!("There is a match at character  {} in line {}\nFlanked on left by {}\nFlanked on right by {}",
			 i,
			 linum,
			 str::from_utf8(leftflank).expect("Something went wrong with the left flank sequence"),
			 str::from_utf8(rightflank).expect("Something went wrong with the right flank sequence")
		)
            }
        }
    }
}

fn search_minimal(line: String, pattern: String, linum: i32) {
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
        println!("There are {} occurances in line {}", count, linum);
    }
}
