fn main() {
    let str = "パタトクカシーー";
    let mut result = "".to_string();
    for (i, val) in str.chars().enumerate() {
        if (i == 0) | (i == 2) | (i == 4) | (i == 6) {
            result.push(val);
        }
    }
    println!("{}", result);
}
