use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use crate::lowercase::GreekLowercaseFilter;
use crate::stem::GreekStemFilter;
use crate::stop::GreekStopFilter;

/// Register all Greek analysis components.
///
/// Registers:
/// - `"greek"` analyzer (greek_lowercase → stop → stem)
/// - `"greek_lowercase"` token filter
/// - `"greek_stem"` token filter
/// - `"greek_stop"` token filter
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("greek_lowercase", Box::new(GreekLowercaseFilter::new()));
    factory.register_token_filter("greek_stem", Box::new(GreekStemFilter::new()));
    factory.register_token_filter("greek_stop", Box::new(GreekStopFilter::new()));

    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(GreekLowercaseFilter::new()),
        Box::new(GreekStopFilter::new()),
        Box::new(GreekStemFilter::new()),
    ];

    factory.register_analyzer(
        "greek",
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("greek_lowercase").is_some());
        assert!(factory.get_token_filter("greek_stem").is_some());
        assert!(factory.get_token_filter("greek_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("greek").is_some());
    }
}
