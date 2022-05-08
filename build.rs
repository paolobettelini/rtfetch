use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    logo: String,
    info: Vec<String>
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=config.toml");

    let mut config: Config = toml::from_str(include_str!("config.toml")).unwrap();

    let result = get_result(&mut config);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("main.rs");
    generate_file(&dest_path, format!(r#"
        use sysinfo::{{System, SystemExt}};
    
        fn main() {{
            let mut sys = System::new_all();
            sys.refresh_all();

            let result = {};
            println!("{{}}", result);
        }}
    "#, result).as_bytes());
}

fn get_result(config: &mut Config) -> String {
    let logo = fs::read_to_string(format!("src/assets/{}.logo", config.logo)).expect("Unable to read logo");

    let mut art_lines = logo.split('\n').collect::<Vec<&str>>();

    let mut stats = Stats::new();

    for line in &config.info {
        stats.process_stats(&line);
    }

    //pad_left(&mut art_lines);

    // extend to same length if needed
    let diff = logo.len() as i32 - config.info.len() as i32;
    if diff > 0 {
        for _ in 0..diff {
            config.info.push("".to_string());
        }
    } else if diff < 0 {
        for _ in 0..diff {
            art_lines.push("");
        }
    }

    let info_iter = config.info.iter();
    let art_iter = art_lines.iter();

    let mut result = "".to_owned();

    for (art_line, info_line) in art_iter.zip(info_iter) {
        result.push_str(&format!("{}  {}\n", art_line, info_line));
    }

    format!("{{{}format!(color_print::cstr!(\"{}\"), {})}}",
        stats.get_declarations(),
        stats.remove_placeholders(&mut result),
        stats.get_format_params())
}

fn generate_file<P: AsRef<Path>>(path: P, text: &[u8]) {
    let mut f = File::create(path).unwrap();
    f.write_all(text).unwrap()
}

struct Stats {
    informations: Vec<Box<dyn Information>>,
}

impl Stats {

    fn new() -> Self {
        Self {
            informations: Vec::new(),
        }
    }

    fn process_stats(&mut self, line: &String) {
        if line.contains("%host_name%") {
            self.informations.push(Box::new(HostName{}));
        }

        if line.contains("%os_version%") {
            self.informations.push(Box::new(OsVersion{}));
        }

        if line.contains("%kernel_version%") {
            self.informations.push(Box::new(KernelVersion{}));
        }

        if line.contains("%total_memory%") {
            self.informations.push(Box::new(TotalMemory{}));
        }
    }

    fn get_declarations(&self) -> String {
        let mut variables = "".to_owned();
        
        for info in &self.informations {
            variables.push_str(info.variable_declaraton());
        }
        
        variables
    }

    fn remove_placeholders(&self, text: &mut String) -> String {
        for info in &self.informations {
            let mut replaced = text.replace(info.placeholder(), "{}");
            text.clear();
            text.push_str(&mut replaced);
        }
        text.to_string()
    }

    fn get_format_params(&self) -> String {
        let mut result = "".to_owned();

        let mut i = 0;
        for info in &self.informations {
            i += 1;
            let placeholder = info.placeholder();
            // %value% -> value
            result.push_str(&placeholder[1..placeholder.len() - 1]);
            if i != self.informations.len() {
                result.push_str(", ");
            }
        }

        result
    }

}

trait Information {
    
    fn variable_declaraton(&self) -> &'static str;
    
    fn placeholder(&self) -> &'static str;

}

struct HostName;

impl Information for HostName {

    fn variable_declaraton(&self) -> &'static str {
        "let host_name = sys.host_name().unwrap();"
    }
    
    fn placeholder(&self) -> &'static str {
        "%host_name%"
    }

}

struct TotalMemory;

impl Information for TotalMemory {

    fn variable_declaraton(&self) -> &'static str {
        "let total_memory = sys.total_memory();"
    }
    
    fn placeholder(&self) -> &'static str {
        "%total_memory%"
    }

}

struct KernelVersion;

impl Information for KernelVersion {

    fn variable_declaraton(&self) -> &'static str {
        "let kernel_version = sys.kernel_version().unwrap();"
    }
    
    fn placeholder(&self) -> &'static str {
        "%kernel_version%"
    }

}

struct OsVersion;

impl Information for OsVersion {

    fn variable_declaraton(&self) -> &'static str {
        "let os_version = sys.os_version().unwrap();"
    }
    
    fn placeholder(&self) -> &'static str {
        "%os_version%"
    }

}

struct Uptime;

impl Information for Uptime {

    fn variable_declaraton(&self) -> &'static str {
        "let uptime = sys.uptime();"
    }
    
    fn placeholder(&self) -> &'static str {
        "%uptime%"
    }

}