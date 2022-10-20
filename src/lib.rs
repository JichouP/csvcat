use csv::ReaderBuilder;
use std::{
    error::Error,
    fs::{read_dir, DirEntry},
};

const HEADER_FILE_SUFFIX: &str = "_Header.txt";

fn read_column_from_path(path: &str, i: usize) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?
        .records()
        .filter_map(|v| Some(v.unwrap().get(i)?.to_string()))
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod read_column_from_path {
    use super::*;
    #[test]
    fn read_sample1() {
        let v = read_column_from_path("sample/sample1.csv", 0).unwrap();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6", "7", "8"]);
    }

    #[test]
    fn read_sample2() {
        let v = read_column_from_path("sample/sample2.csv", 0).unwrap();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6", "7", "8"]);
    }

    #[test]
    #[should_panic]
    fn read_non_existing_file() {
        let v = read_column_from_path("sample/sample3.csv", 0).unwrap();
        assert_eq!(v, vec!["1", "2", "3", "4", "5", "6", "7", "8"]);
    }
}

fn search_header_file(dir_path: &str) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let entries = read_dir(dir_path)?
        .into_iter()
        .filter_map(|v| {
            let v = v.ok()?;
            if v.file_name().to_string_lossy().ends_with("_Header.txt") {
                Some(v)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(entries)
}

#[cfg(test)]
mod search_header_file {
    use super::*;
    #[test]
    fn search_sample_header_file() {
        let v = search_header_file("sample").unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].file_name(), "sample_Header.txt");
    }

    #[test]
    fn search_non_existing_file() {
        let v = search_header_file(".").unwrap();
        assert_eq!(v.len(), 0);
    }

    #[test]
    #[should_panic]
    fn search_non_existing_directory() {
        let v = search_header_file("non_existing_dir").unwrap();
    }
}

fn get_prefix(dir_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let header_file_names = search_header_file(dir_path)?
        .into_iter()
        .map(|v| v.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    let prefixes = header_file_names
        .into_iter()
        .map(|v| v.replace(HEADER_FILE_SUFFIX, ""))
        .collect::<Vec<_>>();

    Ok(prefixes)
}

#[cfg(test)]
mod get_prefix {
    use super::*;
    #[test]
    fn get_sample_file_prefix() {
        let prefixes = get_prefix("sample").unwrap();
        assert_eq!(prefixes.len(), 1);
        assert_eq!(prefixes[0], "sample");
    }

    #[test]
    fn get_non_existing_file_prefix() {
        let prefixes = get_prefix(".").unwrap();
        assert_eq!(prefixes.len(), 0);
    }

    #[test]
    #[should_panic]
    fn get_non_existing_dir_prefix() {
        let prefixes = get_prefix("non_existing_dir").unwrap();
    }
}

fn search_files_by_prefix(dir_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let entries = read_dir(dir_path)
        .unwrap()
        .map(|v| v.unwrap())
        .into_iter()
        .map(|v| v.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    let prefixes = get_prefix(dir_path)?;
    let matched_entries = prefixes
        .into_iter()
        .map(move |prefix| {
            entries
                .to_vec()
                .into_iter()
                .filter(move |file_name| {
                    let is_header = file_name.ends_with("_Header.txt");
                    let is_matched = file_name.starts_with(&prefix);

                    if is_header || !is_matched {
                        return false;
                    }
                    true
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(matched_entries)
}
