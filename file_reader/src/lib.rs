use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read in a file and print its contents to the console.
/// # Examples:
/// ```
/// use file_reader::read_in;
/// read_in(Some(String::from("helloworld.txt")));
/// ```
/// # Panics:
/// If the file is not found, the program will panic.
/// If there is an error opening the file, the program will panic.
/// If there is an error reading a line, the program will panic.
/// # Arguments:
/// * `string_path` - The path to the file to read in. If no path is provided, the default path is "helloworld.txt".

pub fn read_in(string_path: Option<String>) {
    let string_path = string_path.unwrap_or_else(|| String::from("helloworld.txt"));

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", string_path);
    let file = File::open(string_path.clone());
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File {} not found: {}", string_path, error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_read_in_panic() {
        read_in(Some(String::from("nonexistent.txt")));
    }

    #[test]
    fn test_read_in_no_panic() {
        read_in(Some(String::from("helloworld.txt")));
        let s = None;
        read_in(s);
    }
}
