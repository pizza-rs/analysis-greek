# pizza-analysis-greek

Greek language analysis with Greek-specific lowercasing, stemming, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `greek_lowercase` | Token Filter | Greek-specific lowercasing with accent/diacritic removal |
| `greek_stem` | Token Filter | Greek light stemmer |
| `greek_stop` | Token Filter | Greek stop words filter (75 words) |
| `greek` | Analyzer | Full pipeline: greek_lowercase → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "greek"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["greek_lowercase", "greek_stem", "greek_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
