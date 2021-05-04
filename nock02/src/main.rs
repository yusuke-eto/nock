fn main() {
    let str1 = "パトカー";
    let str2 = "タクシー";
    let mut result = "".to_string();
    for (i, j) in str1.chars().zip(str2.chars()) {
        result.push(i);
        result.push(j);
    }
    println!("{}", result);
}
