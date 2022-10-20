use csv::ReaderBuilder;
use std::{
    error::Error,
    fs::{read_dir, DirEntry},
};

fn read_column_from_path(path: &str, i: usize) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?
        .records()
        .filter_map(|v| Some(v.unwrap().get(i)?.to_string()))
        .collect::<Vec<_>>())
}

fn search_header_file(path: &str) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let entries = read_dir(path)?
        .into_iter()
        .filter_map(|v| {
            let v = v.ok()?;
            if v.file_name().to_string_lossy().contains("Header") {
                Some(v)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(entries)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let v = read_column_from_path("sample/sample1.csv", 0).unwrap();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6", "7", "8"]);
    }
}
