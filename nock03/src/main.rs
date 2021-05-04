fn main() {
    let str = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let splitted_str = str.replace(",", "").replace(".", "");
    let words: Vec<&str> = splitted_str.split_whitespace().collect();
    let mut result = Vec::<usize>::new();
    for word in words {
        result.push(word.len());
    }
    println!("{:?}", result);
}
