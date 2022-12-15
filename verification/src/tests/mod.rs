use alloc::{format, vec::Vec};
use std::path::PathBuf;

use walkdir::WalkDir;

mod types;
mod utilities;

// Since some checks are too slow, we only check few examples.
// TODO Randomize the choices.
pub(crate) const CHECKS_COUNT: usize = 8;

pub(crate) mod test_data {
    pub(crate) const ROOT: &str = "../tests/data";
    pub(crate) const COUNT: usize = 64;
}

pub(crate) mod check_entry {
    use walkdir::DirEntry;

    pub(crate) fn is_json(entry: &DirEntry) -> bool {
        entry
            .path()
            .extension()
            .map(|s| s.to_ascii_lowercase() == "json")
            .unwrap_or(false)
    }

    pub(crate) fn if_starts_with(prefix: &str) -> impl Fn(&DirEntry) -> bool + '_ {
        move |entry: &DirEntry| {
            entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with(prefix))
                .unwrap_or(false)
        }
    }
}

pub(crate) fn find_json_files(in_dir: &str, filename_prefix: &str) -> Vec<PathBuf> {
    let files_dir = format!("{}/{}", test_data::ROOT, in_dir);
    let json_files = WalkDir::new(files_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(check_entry::is_json)
        .filter(check_entry::if_starts_with(filename_prefix))
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    assert_eq!(json_files.len(), test_data::COUNT);
    json_files
}
