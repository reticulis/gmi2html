# gmi2html
Library for parsing Gemini Text to HTML format

Example usage:
```rust
use gemini2html::parse_to_html;

let header = "# Title\n ## Description";
let html: Vec<String> = parse_to_html(header).unwrap();

assert_eq!(html, vec!["<h1>Title</h1>", "<h2>Description</h2>"])
```

