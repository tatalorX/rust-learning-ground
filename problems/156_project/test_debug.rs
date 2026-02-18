fn count_content(content: &str) -> (usize, usize, usize, usize) {
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    let bytes = content.len();
    (lines, words, chars, bytes)
}

fn main() {
    let content = "Hello world\nLine two\n";
    let (lines, words, chars, bytes) = count_content(content);
    println!("Content: {:?}", content);
    println!("lines: {}", lines);
    println!("words: {}", words);
    println!("chars: {}", chars);
    println!("bytes: {}", bytes);
}
