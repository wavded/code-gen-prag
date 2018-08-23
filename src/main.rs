#[feature(nll)]
extern crate failure;

use failure::Error;

fn generate_comment<'a>(text: &'a str, lang: &'static str) -> String {
    match lang {
        "c" => format!("/* {} */", text),
        "javascript" => format!("// {}", text),
        _ => "".to_string(),
    }
}

fn generate_model_start(name: &'static str, lang: &'static str) -> String {
    match lang {
        "c" => "typedef struct {".to_string(),
        "javascript" => format!("class {} {{", name),
        _ => "".to_string(),
    }
}

fn generate_model_end(name: &'static str, lang: &'static str) -> String {
    match lang {
        "c" => format!("}} {}", name),
        "javascript" => "}".to_string(),
        _ => "".to_string(),
    }
}

fn parse<'a>(file: &'static str) -> Result<String, Error> {
    use std::fs::File;
    use std::io::Read;

    let mut b = String::new();
    File::open(file)?.read_to_string(&mut b)?;

    let mut r = String::new();
    for line in b.lines() {
        // for part in line.split_whitespace() {}

        r.push_str(match line.chars().next().unwrap() {
            '#' => &format!("{}", generate_comment(line[2..], "javascript")),
            'M' => line,
            'F' => line,
            'E' => line,
            _ => line,
        })
    }
    Ok(r)
}

fn main() {
    match parse("product.txt") {
        Ok(x) => println!("x = {:?}", x),
        Err(e) => println!("e = {:?}", e),
    }
}
