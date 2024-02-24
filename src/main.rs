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
            let mut longseq = String::new();
            if let Ok(lines) = read_lines(&path) {
                for line in lines {
                    let line = line.expect("uh");
                    if line.starts_with('>') {
                        println!("{}", line)
                    } else {
                        if longseq.is_empty() {
                            longseq = collapse_lines(String::new(), line.clone());
                        }
                        longseq = collapse_lines(longseq, line.clone());
                    }
                }
                search_verbose(longseq, code.clone());
            }
        };

        if option == String::from("-m") {
            let mut longseq = String::new();
            if let Ok(lines) = read_lines(&path) {
                for line in lines {
                    let line = line.expect("uh");
                    if line.starts_with('>') {
                        println!("{}", line)
                    } else {
                        if longseq.is_empty() {
                            longseq = collapse_lines(String::new(), line.clone());
                        }
                        longseq = collapse_lines(longseq, line.clone());
                    }
                }
                search_minimal(longseq, code.clone());
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
        println!("There are {}", count);
    }
}

fn collapse_lines(mut fulline: String, curline: String) -> String {
    for ch in curline.chars() {
        fulline.push(ch);
    }
    fulline
}
