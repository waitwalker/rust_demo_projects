use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line)
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
// 将程序拆分为mian.rs和lib.rs，将业务逻辑放入lib.rs
// 当命令行解析逻辑较少时，将它放在main.rs也行
// 当命令行解析逻辑变复杂时，需要将它从main.rs提取到lib.rs
// 调用lib.rs中的run函数，处理run函数可能的错误

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
