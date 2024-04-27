use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(num) if num > 0 => Ok(num),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);
    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
    .version("0.1.0")
    .author("GTazh")
    .about("Rust head")
    .arg(
        Arg::with_name("lines")
        .short("n")
        .long("lines")
        .value_name("LINES")
        .help("Number of lines")
        .default_value("10"),
    )
    .arg(
        Arg::with_name("bytes")
        .short("c")
        .long("bytes")
        .value_name("BYTES")
        .takes_value(true)
        .conflicts_with("lines")
        .help("Number of bytes"),
    )
    .arg(
        Arg::with_name("files")
        .value_name("FILE")
        .help("Input file(s)")
        .multiple(true)
        .default_value("-"),
    )
    .get_matches();

    // println!("{:#?}", matches);
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                for (line_num, line_res) in file.lines().enumerate() {
                    if line_num == 0 {
                        println!("{}==> {} <==", if file_num != 0 {"\n"} else { "" }, &filename);
                    }

                    let line = line_res?;
                    if config.lines {
                        if line_num < config.lines {
                            println!("{}", line);
                        }
                    } else if config.bytes.is_some() {
                        if config.bytes.is_some() {
                            print!("");
                        }
                    }


                }
            }
        }
        file_num += 1;
    }
    Ok(())
}