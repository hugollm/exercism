#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    return String::from(input).chars().rev().collect();
}

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    return input.graphemes(true).rev().collect()
}
