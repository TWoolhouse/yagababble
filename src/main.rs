use yagababble::*;

use unicode_segmentation::UnicodeSegmentation;

fn my_big_text() -> String {
    include_str!("alice.txt")
        .replace("\r\n", "\n")
        .replace("\n\r", "\n")
        .replace("\n", " ")
}

fn main() {
    let text = my_big_text();

    let it = text
        .unicode_sentences()
        .map(|sentence| sentence.unicode_words());

    let mut table = MarkovTable::new();
    for sentence in it {
        table.extend(sentence);
    }

    let chain = table.chain();
    for word in chain {
        print!("{} ", word);
    }
}
