use base64::encode;
use hex::decode;
use std::collections::HashMap;

fn convert_hex_base64(hex: &String) {
    let string_bytes = &hex.clone().into_bytes();

    let hex_decode = decode(hex).expect("expected to decode hex");

    println!("hex decode to bytes {:?}", hex_decode);

    println!("base64 encode {:?}", encode(hex_decode));
}

fn custom_hex_decode(hex: &String) {
    let mut hex_values = HashMap::new();

    hex_values.insert("a", 10);
    hex_values.insert("b", 11);
    hex_values.insert("c", 12);
    hex_values.insert("d", 13);
    hex_values.insert("e", 14);
    hex_values.insert("f", 15);

    let mut char_bytes: Vec<i32> = vec![];

    for i in 0..hex.len() {
        if i % 2 != 0 {
            println!("skipped {}", i);
            continue;
        }
        // loop over string by 2 pairs (x, y)
        let left = &hex[i..i + 1];
        let right = &hex[i + 1..i + 2];

        println!("left {} right {}", left, right);

        let left_value = match hex_values.get(left) {
            Some(m) => m.to_owned(),
            None => left.parse::<i32>().unwrap().to_owned(),
        };
        let right_value = match hex_values.get(right) {
            Some(m) => m.to_owned(),
            None => right.parse::<i32>().unwrap().to_owned(),
        };
        // (N X 16 ** 1) + (N X 16**0)
        let value = left_value * 16_i32.pow(1) + right_value;
        char_bytes.push(value);
    }

    println!("this is char_bytes {:?}", char_bytes);
}

fn main() {
    let hex = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    custom_hex_decode(&hex);
    convert_hex_base64(&hex);
}
