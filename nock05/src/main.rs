fn main() {
    let str = "I am an NLPer";
    let words = str.split_whitespace().collect();
    println!("{:?}", create_ngram(&words, 2));
}

fn create_ngram<'a>(words: &Vec<&'a str>, size: usize) -> Vec<Vec<&'a str>> {
    let mut result: Vec<Vec<&str>> = Vec::new();
    for i in 0..(words.len() - size + 1) {
        result.push(words[i..(i + size)].to_vec());
    }
    return result;
}
