use std::{io::{BufReader, BufRead}, fs::File};

fn main() {
    let file = File::open("input").expect("File not found");
    let lines: Vec<Vec<u8>> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect();
    println!("Solution to first problem: {}", standard(&lines));
    println!("Solution to second problem: {}", bonus(&lines));
}

fn bin_to_dec_closure(acc: i32, t:(usize, &u8)) -> i32 {
    let (i, val) = t;
    acc + (*val as i32) * 2i32.pow(i as u32)
}

fn bin_to_dec(bin: &Vec<u8>) -> i32 {
    bin.iter().rev().enumerate().fold(0, bin_to_dec_closure)
}

fn get_most_common_bit_at_pos(lines: &Vec<Vec<u8>>, pos: usize) -> u8 {
    let size = lines.len();
    let sum_ones = lines.into_iter().fold(0.0, |acc, e| acc + (e[pos] as f32));
    if sum_ones/ (size as f32) >= 0.5 {
        1u8
    } else {
        0u8
    }
}

fn standard(lines: &Vec<Vec<u8>>) -> i32 {
    let total = lines.len();
    let gamma_bin: Vec<u8> = lines
        .to_owned()
        .into_iter()
        .fold(vec![0i32; total], |acc, line| acc.into_iter().zip(line.into_iter()).map(|e| e.0 + (e.1 as i32)).collect())
        .into_iter()
        .map(|e| if (e as f32) / (total as f32) > 0.5 {1u8} else {0u8})
        .collect();
    let epsilon_bin: Vec<u8> = (&gamma_bin).to_vec().iter().map(|e| 1 - e).collect();
    let gamma_dec = bin_to_dec(&gamma_bin);
    let epsilon_dec = bin_to_dec(&epsilon_bin);
    println!("Gamma: {}", gamma_dec);
    println!("Epsilon: {}", epsilon_dec);
    gamma_dec * epsilon_dec
}

fn bonus_filter(lines: &Vec<Vec<u8>>, inverted: bool) -> Result<Vec<u8>, String> {
    let mut lines = lines.clone();
    let word_len = lines[0].len();
    for i in 0usize..word_len {
        let common_bit = get_most_common_bit_at_pos(&lines, i);
        let bit = if inverted {1 - common_bit} else {common_bit};
        lines = lines.into_iter().filter(|e| e[i] == bit).collect();
        if lines.len() == 1 {
            return Ok(lines[0].clone());
        }
    }
    Err(String::from("There are still elements in the list"))
    
}

fn bonus(lines: &Vec<Vec<u8>>) -> i32 {
    let ox = bin_to_dec(&bonus_filter(lines, false).unwrap());
    let co2 = bin_to_dec(&bonus_filter(lines, true).unwrap());
    ox * co2
}
