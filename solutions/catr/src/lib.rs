use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

// Using box to store values in Heap, because the system doesn't know how much memory it might need. So the stack is not suitable here.
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("GTazh")
        .about("Rust cat")
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number nonblank lines")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank")
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .get_matches();

    let file = matches.values_of_lossy("file").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank = matches.is_present("number_nonblank");

    Ok(Config {
        files: file,
        number_lines: number_lines,
        number_nonblank_lines: number_nonblank,
    })
}

fn print(reader: &mut Box<dyn BufRead>, number_lines: bool, number_nonblank_lines: bool) {
    let mut line_num = 1;

    for line in reader.lines() {
        let text = line.unwrap();

        if number_lines {
            print!("     {}\t{}\n", line_num, text);
            line_num += 1;
        } else if number_nonblank_lines {
            if text != "" {
                print!("     {}\t{}\n", line_num, text);
                line_num += 1;
            } else {
                println!("{}", text)
            }
        } else {
            println!("{}", text);
        }
    } 
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => print(&mut open(&filename).unwrap(), config.number_lines, config.number_nonblank_lines)
        }
    }
    Ok(())
}
