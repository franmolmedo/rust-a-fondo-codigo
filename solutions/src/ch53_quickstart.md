# Quickstart verificable

```rust
use course_solutions::organization::c53::{ConfigBuilder, Source};

let config = ConfigBuilder::default()
    .source(Source::Defaults)
    .value("port", "8080")
    .build()?;

assert_eq!(config.get("port")?, "8080");
# Ok::<(), Box<dyn std::error::Error>>(())
```
