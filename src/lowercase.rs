use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Greek-specific lowercase filter that also removes tonos (accent marks)
/// from Greek characters: ά→α, έ→ε, ή→η, ί→ι, ό→ο, ύ→υ, ώ→ω.
/// Also handles ΐ, ΰ, and maps ς→σ.
#[derive(Clone, Debug, Default)]
pub struct GreekLowercaseFilter;

impl GreekLowercaseFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for GreekLowercaseFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let mut result = String::with_capacity(text.len());
        let mut changed = false;

        for c in text.chars() {
            let lowered = match c {
                // Uppercase Greek → lowercase (standard)
                'Α'..='Ω' => {
                    changed = true;
                    char::from_u32(c as u32 + 32).unwrap_or(c)
                }
                // Accented uppercase
                'Ά' => { changed = true; 'α' }
                'Έ' => { changed = true; 'ε' }
                'Ή' => { changed = true; 'η' }
                'Ί' => { changed = true; 'ι' }
                'Ό' => { changed = true; 'ο' }
                'Ύ' => { changed = true; 'υ' }
                'Ώ' => { changed = true; 'ω' }
                'Ϊ' => { changed = true; 'ι' }
                'Ϋ' => { changed = true; 'υ' }
                // Accented lowercase → remove tonos
                'ά' => { changed = true; 'α' }
                'έ' => { changed = true; 'ε' }
                'ή' => { changed = true; 'η' }
                'ί' | 'ΐ' => { changed = true; 'ι' }
                'ό' => { changed = true; 'ο' }
                'ύ' | 'ΰ' => { changed = true; 'υ' }
                'ώ' => { changed = true; 'ω' }
                // Final sigma → regular sigma
                'ς' => { changed = true; 'σ' }
                // ASCII uppercase
                'A'..='Z' => {
                    changed = true;
                    char::from_u32(c as u32 + 32).unwrap_or(c)
                }
                _ => c,
            };
            result.push(lowered);
        }

        if changed {
            token.term = Cow::Owned(result);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greek_lowercase_with_tonos() {
        let filter = GreekLowercaseFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("Ελληνικά"),
            start_offset: 0,
            end_offset: 16,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "ελληνικα");
    }

    #[test]
    fn test_final_sigma() {
        let filter = GreekLowercaseFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("λόγος"),
            start_offset: 0,
            end_offset: 10,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "λογοσ");
    }
}
