use gemini2html::parse_to_html;

#[test]
fn try_parse() {
    let header1 = "# Header 1";
    assert_eq!(parse_to_html(header1).unwrap(), vec!["<h1>Header 1</h1>"]);

    let header2 = "\
            ## Header 2\
        ";
    assert_eq!(parse_to_html(header2).unwrap(), vec!["<h2>Header 2</h2>"]);

    let header3 = "### Header 3";
    assert_eq!(parse_to_html(header3).unwrap(), vec!["<h3>Header 3</h3>"]);

    let link = "=> https://www.rust-lang.org/ Rust!";
    assert_eq!(
        parse_to_html(link).unwrap(),
        vec!["<a href=\"https://www.rust-lang.org/\">Rust!</a>"]
    );

    let link = "=> https://www.rust-lang.org/static/images/rust-logo-blk.svg Rust!";
    assert_eq!(
        parse_to_html(link).unwrap(),
        vec!["<img src=\"https://www.rust-lang.org/static/images/rust-logo-blk.svg\">Rust!</img>"]
    );

    let quote = "> Quote";
    assert_eq!(
        parse_to_html(quote).unwrap(),
        vec!["<blockquote><p>Quote</p></blockquote>"]
    );

    let code = "```rust\nprintln!(\"Hello world!\")\n```";
    assert_eq!(
        parse_to_html(code).unwrap(),
        vec!["<code> rust\nprintln!(\"Hello world!\")\n</code>"]
    );
}