use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    logo: String,
    setup: Vec<String>,
    info: Vec<String>,
}

fn main() { 
    println!("cargo:rerun-if-changed=../Fetch.toml");

    let mut config: Config = toml::from_str(include_str!("../Fetch.toml")).unwrap();

    let code = generate_code(&mut config);

    let out_dir = env::var("OUT_DIR").unwrap();
    
    let dest_path = Path::new(&out_dir).join("result.rs");
    generate_file(&dest_path, code.as_bytes());
}

fn generate_code(config: &mut Config) -> String {
    let info_code = generate_string_to_print(config);

    let mut result = String::new();
    
    // wrap everything in an expression
    // because you can only include! a single expression
    result.push('{');
    result.push('\n');

    for string in &config.setup {
        result.push_str("    ");
        result.push_str(&string);
        result.push('\n');
    }

    result.push_str(&format!("    let result = {};\n", info_code));
    result.push_str("    println!(\"{}\", result);\n");

    result.push('}');
    result.push('\n');

    result
}

fn generate_string_to_print(config: &mut Config) -> String {
    let logo = fs::read_to_string(format!("src/assets/{}", config.logo)).expect("Unable to read logo");

    let mut art_lines = logo.split('\n').collect::<Vec<&str>>();

    let mut variables = Vec::new();

    for line in &config.info {
        let mut counter = 0;
        let mut variable = "".to_owned();
        for c in line.chars() {
            if c == '{' {
                counter += 1;
                if counter == 1 {
                    continue;
                }
            } else if c == '}' {
                counter -= 1;

                if counter == 0 {
                    if variable.len() != 0 {
                        variables.push(variable);
                        variable = "".to_owned();
                    }
                }
            }

            if counter > 0 {
                variable.push(c);
            }
        }
    }

    // extend to same length if needed
    let diff = art_lines.len() as i32 - config.info.len() as i32;
    if diff > 0 {
        for _ in 0..diff {
            config.info.push("".to_string());
        }
    } else if diff < 0 {
        for _ in 0..-diff {
            art_lines.push("");
        }
    }

    let info_iter = config.info.iter();
    let art_iter = art_lines.iter();

    let mut result = "".to_owned();

    for (art_line, info_line) in art_iter.zip(info_iter) {
        result.push_str(&format!("{}  {}\n", art_line, info_line));
    }

    format!("format!(color_print::cstr!(\"{}\"), {})",
        strip_variables(&mut result, &variables),
        as_params(&variables))
}

fn strip_variables(text: &mut String, variables: &Vec<String>) -> String {
    for variable in variables {
        *text = text.replace(variable, "");
    }
    text.to_string()
}

fn as_params(lines: &Vec<String>) -> String {
    let mut result = "".to_owned();
    let mut i = 0;

    for line in lines {
        i += 1;
        result.push_str(&line);
        if i != lines.len() {
            result.push_str(", ");
        }
    }
    result
}

fn generate_file<P: AsRef<Path>>(path: P, text: &[u8]) {
    let mut f = File::create(path).unwrap();
    f.write_all(text).unwrap()
}