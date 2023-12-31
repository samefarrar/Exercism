use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let graphemes = UnicodeSegmentation::graphemes(input, true).rev();
    let mut output = String::new();
    for grapheme in graphemes {
        output.push_str(grapheme);
    }
    return output;
}
