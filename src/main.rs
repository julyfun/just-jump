use std::process::Command;
use toml::value::Table;

fn find_last_string(table: &toml::value::Table, args: Vec<String>) -> Option<String> {
    let mut current = table;
    // 这里 take 是 next() 总长度
    for key in args.iter().skip(1).take(args.len() - 2) {
        println!("key: {key}");
        match current.get(key) {
            Some(value) => {
                if let toml::Value::Table(t) = value {
                    current = t;
                } else {
                    return None;
                }
            }
            None => return None,
        }
    }
    println!("hello");
    if let Some(last_key) = args.last() {
        if let Some(value) = current.get(last_key) {
            if let toml::Value::String(s) = value {
                return Some(s.to_string());
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("file root: {}", std::env::current_dir().unwrap().display());
    let home_path = std::env::var("HOME").unwrap();
    let file_path = home_path + "/.config/just-jump/jump.toml";
    let file = match std::fs::read_to_string(&file_path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {}, file path: {}", e, file_path),
    };
    let table: Table = toml::from_str(&file)?;
    // dbg!(&table);
    let args: Vec<String> = std::env::args().collect();
    let link = find_last_string(&table, args).unwrap();
    println!("{link}");
    let output = Command::new("open")
        .arg("-a")
        .arg("Google Chrome")
        .arg(link)
        .output()
        .expect("failed to execute process");
    let std_err = String::from_utf8(output.stderr).unwrap();
    println!("{std_err}");
    Ok(())
}
