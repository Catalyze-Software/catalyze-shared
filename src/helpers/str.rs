use unicode_segmentation::UnicodeSegmentation;

pub fn str_len(value: &str) -> usize {
    value.graphemes(true).count()
}
