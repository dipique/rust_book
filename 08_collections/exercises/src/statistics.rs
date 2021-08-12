use std::collections::HashMap;

pub fn sum(numbers: &[f64]) -> f64 {
    numbers.iter().fold(0.0, |acc, &v| acc + v)
}

pub fn mean(numbers: &[f64]) -> f64 {
    if numbers.len() == 0 {
        return 0f64;
    }
    sum(numbers) / numbers.len() as f64
}

enum Parity {
    Even,
    Odd
}

// this method needs to take a Vec because we use Vec capabilities
pub fn median(numbers: &Vec<f64>) -> f64 {
    let len = numbers.len();
    if len == 0 {
        return 0f64;
    }
    let tgt_idx = (len as f64 / 2.0).ceil() as usize - 1;
    let parity = if len % 2 == 0 { Parity::Even } else { Parity::Odd };
    
    let mut cpy = numbers.clone(); // sorts are in-place so the input has to be cloned
    cpy.sort_by(|a, b| a.partial_cmp(b).unwrap()); // floats have to be sorted in a special way

    match parity {
        Parity::Odd => cpy[tgt_idx],
        Parity::Even => mean(&cpy[tgt_idx..(tgt_idx + 1)])
    }
}

pub fn max(numbers: &[f64]) -> f64 {
    numbers.iter().fold(0.0, |acc, &v| if v > acc { v } else { acc })
}

// this uses a slice, which also accepts a vector
pub fn mode(numbers: &[f64]) -> String {
    //let map = HashMap::new();
    //this ends up being a heavy replacement for a simple loop
    let hmap = numbers.iter().fold(HashMap::new(), |mut hm, &v| {
        let cnt = hm.entry(v.to_string()).or_insert(0);
        *cnt += 1;
        hm
    });

    let mut ret_val:i32 = 0;
    let mut ret_key = "";
    for (key, val) in &hmap {
        if val > &ret_val {
            ret_val = *val;
            ret_key = key;
        }
    }

    // to return an f64, we would need to store the integral
    // and fractional parts separately in a struct for the
    // hashmap and then re-combine them at the end.
    ret_key.to_string()
}