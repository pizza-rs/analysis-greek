//! Greek stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Greek stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "αλλα",
    "αν",
    "αντι",
    "απο",
    "αυτα",
    "αυτεσ",
    "αυτη",
    "αυτο",
    "αυτοι",
    "αυτοσ",
    "αυτουσ",
    "αυτων",
    "για",
    "δε",
    "δεν",
    "εαν",
    "ειμαι",
    "ειμαστε",
    "ειναι",
    "εισαι",
    "ειστε",
    "εκεινα",
    "εκεινεσ",
    "εκεινη",
    "εκεινο",
    "εκεινοι",
    "εκεινοσ",
    "εκεινουσ",
    "εκεινων",
    "ενω",
    "επι",
    "η",
    "θα",
    "ισωσ",
    "κ",
    "και",
    "κατα",
    "κι",
    "μα",
    "με",
    "μετα",
    "μη",
    "μην",
    "να",
    "ο",
    "οι",
    "ομωσ",
    "οπωσ",
    "οσο",
    "οτι",
    "παρα",
    "ποια",
    "ποιεσ",
    "ποιο",
    "ποιοι",
    "ποιοσ",
    "ποιουσ",
    "ποιων",
    "που",
    "προσ",
    "πωσ",
    "σε",
    "στη",
    "στην",
    "στο",
    "στον",
    "τα",
    "την",
    "τησ",
    "το",
    "τον",
    "τοτε",
    "του",
    "των",
    "ωσ",
    ];
    words.iter().copied().collect()
});

/// Removes Greek stop words from the token stream.
#[derive(Clone, Debug)]
pub struct GreekStopFilter {
    stop_words: HashSet<String>,
}

impl Default for GreekStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl GreekStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for GreekStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 75);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = GreekStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = GreekStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = GreekStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
