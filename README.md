<div align="center">

# 🇬🇷 pizza-analysis-greek

**Greek text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--greek-blue)](https://github.com/pizza-rs/analysis-greek)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Greek language analysis with accent-aware lowercasing, stemming, and stop words.
Handles Greek diacritics (tonos, dialytika) during case conversion.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `greek_lowercase` | Greek-aware lowercase (strips accents/tonos) |
| TokenFilter | `greek_stem` | Greek stemmer (suffix stripping) |
| TokenFilter | `greek_stop` | Greek stop words (75 entries) |
| Analyzer | `greek` | Full pipeline: greek_lowercase → stem → stop |

### Greek Lowercasing

Standard Unicode lowercasing doesn't handle Greek accents correctly. This filter:
- Converts Σ to σ (or ς in final position)
- Strips tonos accent (ά→α, έ→ε, ή→η, ί→ι, ό→ο, ύ→υ, ώ→ω)
- Handles dialytika (ϊ, ϋ)

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_greek::register_all(&mut factory);

let analyzer = factory.get_analyzer("greek").unwrap();
// "ΕΛΛΗΝΙΚΆ" → ["ελληνικ"]
```

## Installation

```toml
[dependencies]
pizza-analysis-greek = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["greek"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
