use std::fs::File;
use std::io::{BufReader, BufRead, ErrorKind, Error};

enum Command {
    Forward(i32),
    Down(i32)
}

impl Command {
    fn from_string(s: &String) -> Result<Command, Error> {
        if let Some((key, value)) = s.split_once(' '){
            if let Ok(value) = value.parse::<i32>(){
                match key {
                    "forward" => return Ok(Command::Forward(value)),
                    "down" => return Ok(Command::Down(value)),
                    "up" => return Ok(Command::Down(-value)),
                    _ => return Err(Error::new(ErrorKind::InvalidInput, "Command not valid"))
                }
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Can't convert value string to num"));
            }
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Can't split key value"));
        };
    }
}

fn standard() {
    let file = File::open("input").expect("File not found");
    let list:Vec<Command> = BufReader::new(file).lines().map(|l| Command::from_string(&(l.unwrap())).unwrap()).collect();
    let mut hor = 0;
    let mut ver = 0;
    for command in list {
        match command {
            Command::Forward(val) => hor += val,
            Command::Down(val) => ver += val,
        }
    }
    println!("({}, {}), product = {}", hor, ver, hor*ver);
}

fn bonus() {
    let file = File::open("input").expect("File not found");
    let list:Vec<Command> = BufReader::new(file).lines().map(|l| Command::from_string(&(l.unwrap())).unwrap()).collect();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in list {
        match command {
            Command::Forward(val) => {
                horizontal += val;
                depth += aim*val;
            }
            Command::Down(val) => aim += val,
        }
    }
    println!("({}, {}), aim: {}, product = {}", horizontal, depth, aim, horizontal*depth);
}

fn main() {
    standard();
    bonus();
 }
