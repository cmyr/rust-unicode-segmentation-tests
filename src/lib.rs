extern crate unicode_segmentation;

include!(concat!(env!("OUT_DIR"), "/gen_tests.rs"));
#[cfg(test)]
mod tests {
    use unicode_segmentation::UnicodeSegmentation;

    #[test]
    fn skin_tone_modifier() {
        // http://unicode.org/reports/tr29/#WB14
        let s = "😌👎🏼";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        let b: &[_] = &["😌", "👎🏼"];
        assert_eq!(w, b);
    }

    #[test]
    fn regional_indicator_symbols() {
        // http://unicode.org/reports/tr29/#WB15
        let s = "🇨🇦🇨🇭🇿🇲🇿 hi";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        let b: &[_] = &["🇨🇦", "🇨🇭", "🇿🇲", "🇿", "hi"];
        assert_eq!(w, b);
    }

    #[test]
    fn emoji_zwj_sequence() {
        // http://unicode.org/reports/tr29/#WB3c
        let s = "\u{1f468}\u{200d}\u{1f468}\u{200d}\u{1f466}";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        let b: &[_] = &["\u{1f468}\u{200d}\u{1f468}\u{200d}\u{1f466}"];
        assert_eq!(w, b);
    }
}
