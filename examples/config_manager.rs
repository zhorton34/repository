use repository::config::{ConfigContract, Repository};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut items = HashMap::new();
    let file = File::open("config.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("=").collect();
        items.insert(parts[0].to_owned(), parts[1].to_owned());
    }

    let mut config = Repository::new(items);
    config.set("foo", "baz");
    config.set("bar", "qux");

    println!("All config items:");
    for (key, value) in config.all() {
        println!("{}={}", key, value);
    }

    Ok(())
}
