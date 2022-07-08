use std::fs;
use std::fs::File;
use std::io::Write;
use gemini2html::parse_to_html;

#[test]
fn file_test() {
    let file = fs::read_to_string("test.gmi").unwrap();

    let gmi = parse_to_html(&file).unwrap().join("\n");

    let mut file = File::create("test.html").unwrap();

    file.write_all(gmi.as_bytes()).unwrap();
}