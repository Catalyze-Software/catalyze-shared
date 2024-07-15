use unicode_segmentation::UnicodeSegmentation;

pub fn str_len(value: &str) -> usize {
    value.graphemes(true).count()
}

pub fn eq_str<S: Into<String>>(a: S, b: S) -> bool {
    a.into().to_lowercase().trim() == b.into().to_lowercase().trim()
}
