use std::env;
use std::fs;

fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <substring> <directory>", args[0]);
        return;
    }

    let query = &args[1];
    let directory = &args[2];

    match search_files(directory, query) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }
}

fn search_files(directory: &str, query: &str) -> Result<String, String> {
    let entries = fs::read_dir(directory).map_err(|e| format!("Error reading directory: {}", e))?;

    let mut count = 0;
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.contains(query) {
                    count += 1;
                    println!("{}. {}", count, file_name);
                }
            }
        }
    }
    Ok(format!("Occurrences of '{}': {}", query, count))
}
