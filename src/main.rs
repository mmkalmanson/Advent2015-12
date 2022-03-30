use clap::Parser;
use regex::Regex;
use serde_json::{json, Value};

/// Advent 2015-10
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the part to solve
    #[clap(short, long)]
    part: u8,
}

fn add_numbers(input: &str) -> isize {
    let re = Regex::new(r"(-*\d+)").unwrap();
    re.find_iter(input)
        .map(|x| x.as_str().parse::<isize>().unwrap())
        .sum()
}

fn find_reds(input: &mut Value) -> &mut Value {
    match input {
        Value::Object(ref mut object) => {
            for (key, value) in object.into_iter() {
                if key == &"red".to_string() || value == &json!("red") {
                    object.clear();
                    break;
                }
                find_reds(value);
            }
        }
        Value::Array(ref mut array) => array.iter_mut().for_each(|x| {
            find_reds(x);
        }),
        _ => {}
    }
    input
}

fn main() {
    let args = Args::parse();
    let input = include_str!("../input.json");

    match args.part {
        1 => {
            let x = add_numbers(input);
            println!("{}", x);
        }
        2 => {
            let mut y = serde_json::from_str(input).unwrap();
            let x = add_numbers(&find_reds(&mut y).to_string());
            println!("{}", x);
        }
        _ => eprintln!("Invalid part"),
    }
}
