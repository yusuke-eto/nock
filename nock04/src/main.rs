use std::collections::HashMap;

fn main() {
    let str = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let str = str.replace(".", "").replace(",", "");
    let words: Vec<&str> = str.split_whitespace().collect();
    let mut map: HashMap<&str, usize> = HashMap::new();

    for (index, value) in words.iter().enumerate() {
        match index {
            1 | 5 | 6 | 7 | 8 | 9 | 15 | 16 | 19 => {
                map.insert(&value[0..2], index);
            }
            _ => {
                map.insert(&value[0..1], index);
            }
        }
    }

    println!("{:?}", map);
}
