// build.rs
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::{Captures, Regex};

// static mut SOURCE_PATH: &'static str = if cfg!(feature = "unicode9") {
// "res/WordBreakTest9.txt"
// } else {
// "res/WordBreakTest.txt"
// };


fn open_samples() -> String {
    let SOURCE_PATH = if cfg!(feature = "unicode9") {
        "res/WordBreakTest9.txt"
    } else {
        "res/WordBreakTest.txt"
    };

    let path = Path::new(unsafe { SOURCE_PATH });
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {}
    }
    s
}


lazy_static!{
    pub static ref CODEPOINT_RE: Regex = {
        Regex::new(r"(÷|×|[0-9A-F]+)").unwrap()
    };
}

fn write_generated_tests_file(test_code: String) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen_tests.rs");
    let mut f = File::create(&dest_path).unwrap();

    write!(f,
           "

#[cfg(test)]
mod gen_tests {{
    use unicode_segmentation::UnicodeSegmentation;
    \
            {}
}}
",
           test_code);
    // f.write_all(file_contents.into_bytes()).unwrap()
}

fn generate_expectatation(tokens: String) -> String {
    let mut items: Vec<String> = Vec::new();
    let mut should_break = false;
    let mut current = String::from("\"");
    for token in tokens.split_whitespace() {
        match token {
            "÷" => {
                if current.len() > 1 {
                    current.push_str("\"");
                    items.push(current);
                    current = String::from("\"");
                }
            }

            "×" => {}
            x => current.push_str(x),
        }
    }
    format!("let b: &[_] = &[{}];", items.join(", "))
}

fn generate_test_case(ucd_test: &str, num: i32) -> Option<String> {
    if let Some(idx) = ucd_test.find("#") {
        let (test_desc, comment) = ucd_test.split_at(idx);
        let tokens: String = CODEPOINT_RE.replace_all(test_desc, |c: &Captures| {
            match c.at(1) {
                Some(x) if x.len() >= 4 => format!("\\u{{{}}}", x.trim()),
                Some(y) => y.trim().to_string(),
                None => panic!("match contained no.. match?"),
            }
        });

        let test_text = tokens.split_whitespace()
            .filter(|t| t.len() > 3)
            .collect::<String>();

        let test_expectation = generate_expectatation(tokens);

        let generated_test = format!("
    #[test]
    fn generated_test_{}() {{
        // {}
        let s = \"{}\";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        {}
        assert_eq!(w, b);
    }}
    ",
                                     num,
                                     comment,
                                     test_text,
                                     test_expectation);
        Some(generated_test)
    } else {
        None
    }
}

fn main() {
    let unicode_tests = open_samples();
    let lines = unicode_tests.lines()
        .filter_map(|l| {
            match l {
                x if x.starts_with("#") => None,
                x => Some(x),
            }
        })
        .enumerate()
        .map(|(i, line)| generate_test_case(line, i as i32).unwrap_or("".to_string()))
        .collect::<Vec<_>>()
        .join("\n");

    write_generated_tests_file(lines);
}
