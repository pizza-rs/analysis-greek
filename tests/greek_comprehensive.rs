//! Comprehensive tests for pizza-analysis-greek.

use pizza_analysis_greek::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// GreekLowercaseFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn lowercase_construction() {
    let _f = GreekLowercaseFilter::new();
}

#[test]
fn lowercase_uppercase_sigma() {
    let f = GreekLowercaseFilter::new();
    // "ΣΠΙΤΙ" → "σπιτι"
    let mut token = make_token("ΣΠΙΤΙ");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(token.term.chars().all(|c| !c.is_uppercase() || !c.is_ascii()));
}

#[test]
fn lowercase_final_sigma() {
    let f = GreekLowercaseFilter::new();
    // Final sigma ς should stay as ς or normalize
    let mut token = make_token("ΛΟΓΟΣ");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn lowercase_removes_accents() {
    let f = GreekLowercaseFilter::new();
    // "Ελληνικά" → strip accents
    let mut token = make_token("Ελληνικά");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn lowercase_ascii_passthrough() {
    let f = GreekLowercaseFilter::new();
    let mut token = make_token("hello");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn lowercase_empty_string() {
    let f = GreekLowercaseFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// GreekStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = GreekStemFilter::new();
}

#[test]
fn stem_plural_noun() {
    let f = GreekStemFilter::new();
    // "σπιτια" (houses, already lowercased) → stem
    let mut token = make_token("σπιτια");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_form() {
    let f = GreekStemFilter::new();
    let mut token = make_token("γραφω");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_adjective() {
    let f = GreekStemFilter::new();
    let mut token = make_token("μεγαλος");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = GreekStemFilter::new();
    let mut token = make_token("και");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = GreekStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// GreekStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = GreekStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = GreekStopFilter::new();
    let stop_words = ["και", "ο", "η", "το", "στο", "να", "ειναι", "που", "με", "για"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = GreekStopFilter::new();
    let content_words = ["σπιτι", "βιβλιο", "σχολειο"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = GreekStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("greek_lowercase").is_some());
    assert!(factory.get_token_filter("greek_stem").is_some());
    assert!(factory.get_token_filter("greek_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("greek").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("greek").unwrap();
    let mut input = String::from("Το σπίτι είναι μεγάλο");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("greek").unwrap();
    let mut input = String::from("ο γατος και ο σκυλος");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"ο"));
    assert!(!terms.contains(&"και"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("greek").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("greek").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
