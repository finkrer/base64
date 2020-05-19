use std::io;
use std::char;

fn main() {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
    let result = to_base_64(&input.trim());
    print!("{}", result);
}

fn to_base_64(string: &str) -> String {
    let mut result = String::new();
    let bits: Vec<bool> = string.bytes()
        .map(|c| u8_to_bits(c))
        .flatten()
        .collect();

    let mut buf = [false; 6];
    let mut i = 0;
    for bit in bits {
        buf[i] = bit;
        i += 1;
        if i == 6 {
            i = 0;
            result.push(get_base64_char_from_buf(&buf));
            buf = [false; 6];
        }
    }
    if i != 0 {
        result.push(get_base64_char_from_buf(&buf));
    }
    while result.len() % 4 != 0 {
        result.push('=');
    }
    result
}

fn get_base64_char_from_buf(buf: &[bool]) -> char {
    let code = get_buf_contents(buf);
    let code = get_base64_code_from_u32(code);
    char::from_u32(code).unwrap()
}

fn u8_to_bits(code: u8) -> Vec<bool> {
    let mut result = Vec::new();
    for i in 0..8 {
        let val = (code >> (7-i)) & 1 == 1;
        result.push(val);
    }
    result
}

fn get_buf_contents(buf: &[bool]) -> u32 {
    let mut result = 0;
    for i in 0..6 {
        if buf[i] {
            result += 1 << (5 - i);
        }
    }
    result
}

fn get_base64_code_from_u32(code: u32) -> u32 {
    match code {
        0..=25 => code + 65,
        26..=51 => code + 71,
        52..=61 => code - 4,
        62 => 43,
        63 => 47,
        _ => panic!("Tried to convert number >63")
    }
}
