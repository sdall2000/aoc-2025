use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Read a text file containing one number per line and return them as a `Vec<i32>`.
///
/// Empty lines are ignored. Parsing errors return an `Err`.
pub fn read_numbers_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut values = Vec::new();

    for line_res in reader.lines() {
        let line = line_res?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let num: i32 = trimmed.parse()?;
        values.push(num);
    }

    Ok(values)
}

/// Read a text file and return each non-empty line as a `Vec<String>`.
///
/// Empty lines are ignored. IO errors return an `Err`.
pub fn read_lines_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line_res in reader.lines() {
        let line = line_res?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        lines.push(trimmed.to_string());
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use std::env;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn read_numbers_from_file_parses_numbers() {
        let tmp_dir = env::temp_dir();
        let path = tmp_dir.join("utils_test_numbers.txt");
        let contents = "10\n20\n30\n\n40\n";
        write(&path, contents).expect("failed to write temp file");

        let numbers = read_numbers_from_file(&path).expect("read failed");
        assert_eq!(numbers, vec![10, 20, 30, 40]);
    }

    #[test]
    fn read_lines_from_file_parses_lines() {
        let tmp_dir = env::temp_dir();
        let path = tmp_dir.join("utils_test_lines.txt");
        let contents = "alpha\nbeta\n\ngamma\n";
        write(&path, contents).expect("failed to write temp file");

        let lines = read_lines_from_file(&path).expect("read failed");
        assert_eq!(lines, vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()]);
    }
}
