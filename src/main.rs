use std::fs::File;
use std::io::Read;
use std::io::{BufWriter, Write};
use std::fs::create_dir_all;

extern crate pulldown_cmark;
use pulldown_cmark::{html, Parser};

/// Get the markdown of our article file.
fn get_data(filename: &str) -> String {
    let mut data = String::new();
    let mut f = File::open(&filename).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    return data;
}

/// In goes markdown text; out comes HTML text.
fn mark_to_html(markdown: &str) -> String {
    let parser = Parser::new(&markdown);
    let mut buffer = String::new();
    html::push_html(&mut buffer, parser);
    buffer
}


/// In goes our article html; out goes a html file.
fn write_html(html: &str) {
    // If if doesn't exist, create a `public` directory.
    create_dir_all("public");

    let f = File::create("public/index.html").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(html.as_bytes()).expect("Unable to write data");
}

fn main() {
    // Get the article data and turn it into html.
    let data = get_data("article.md");
    let html = mark_to_html(&data);
    write_html(&html);

    println!("✨ Success creating articles!");
}
