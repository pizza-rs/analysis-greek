use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Greek stemmer based on the Georgios Ntais stemming algorithm.
/// Removes common Greek suffixes for nouns, adjectives, and verbs.
#[derive(Clone, Debug, Default)]
pub struct GreekStemFilter;

impl GreekStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for GreekStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.chars().count() < 4 {
            return (false, None);
        }

        let stemmed = stem_greek(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_greek(word: &str) -> String {
    let mut s = word.to_string();
    let len = s.chars().count();

    // Step 1: Remove common noun/adjective endings
    // Ordered longest-first for greedy matching
    let suffixes_3plus: &[&str] = &[
        "ματων", "ματα", "ματος", "ματοσ",
        "ησεων", "ησεισ", "ησεωσ",
        "ουδων", "ουδεσ",
        "ιστεσ", "ιστησ", "ιστων",
        "ισμων", "ισμοσ", "ισμοι", "ισμου",
        "ητεσ", "ητασ", "ητων",
        "οτητ", "οτητα", "οτητοσ", "οτητεσ",
        "ικων", "ικεσ", "ικοσ", "ικου", "ικοι",
        "εων", "εωσ",
        "ων", "εσ", "ησ", "οσ", "ου", "οι", "ασ", "υσ",
        "ηδεσ", "ηδων",
        "αδεσ", "αδων",
    ];

    for &suffix in suffixes_3plus {
        if s.ends_with(suffix) {
            let suffix_chars = suffix.chars().count();
            if len - suffix_chars >= 3 {
                let new_len = len - suffix_chars;
                s = s.chars().take(new_len).collect();
                return s;
            }
        }
    }

    // Step 2: Remove verb endings
    let verb_suffixes: &[&str] = &[
        "ουνται", "ονται", "ουμαι", "ειται",
        "ονταν", "ηθηκε", "ησουν",
        "αγαν", "ουσαν", "ουμε", "ειτε", "ουνε",
        "ησει", "ηστε", "ησαν", "ησεσ",
        "ουν", "ανε", "αει", "εισ",
        "ουσ", "αμε", "ατε",
        "ει", "ησ", "αν", "ων",
    ];

    let current_len = s.chars().count();
    for &suffix in verb_suffixes {
        if s.ends_with(suffix) {
            let suffix_chars = suffix.chars().count();
            if current_len - suffix_chars >= 3 {
                let new_len = current_len - suffix_chars;
                s = s.chars().take(new_len).collect();
                return s;
            }
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_noun() {
        let filter = GreekStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("ελληνικων"),
            start_offset: 0,
            end_offset: 18,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "ελληνικ");
    }

    #[test]
    fn test_short_word_unchanged() {
        let filter = GreekStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("και"),
            start_offset: 0,
            end_offset: 6,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "και");
    }
}
