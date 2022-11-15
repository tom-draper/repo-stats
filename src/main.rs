extern crate walkdir;
use std::{collections::HashSet, fs, path::Path};
mod extensions;
mod view;
use walkdir::WalkDir;

#[derive(Debug)]
struct CodeCounts {
    total: u64,
    code: u64,
}

#[derive(Debug)]
struct FileCounts {
    total: u64,
    code: u64,
    binaries: u64,
    storage: u64,
}

#[derive(Debug)]
pub struct Stats {
    files: FileCounts,
    lines: CodeCounts,
    chars: CodeCounts,
}

impl Stats {
    fn avg_code_line_len(&self) -> u64 {
        self.chars.code / self.lines.code
    }
    fn avg_total_line_len(&self) -> u64 {
        self.chars.total / self.lines.total
    }
}

fn is_target_file(path: &Path, target_dir: &HashSet<&str>, ignore_dir: &HashSet<&str>) -> bool {
    let _path = path.to_str().unwrap();
    
    for target in target_dir {
        if !_path.contains(*target) {
            return false
        }
    }

    for ignore in ignore_dir {
        if _path.contains(*ignore) {
            return false
        }
    }

    return true
}

fn main() {
    let mut stats = Stats {
        files: FileCounts {
            total: 0,
            code: 0,
            binaries: 0,
            storage: 0,
        },
        lines: CodeCounts { total: 0, code: 0 },
        chars: CodeCounts { total: 0, code: 0 },
    };

    let ignore_dir = HashSet::from(["target", ".vscode"]);
    let target_dir = HashSet::from(["src"]);
    let source_code_exts = extensions::source_code_extensions();
    let binary_exts = extensions::binary_extensions();
    let storage_exts = extensions::storage_extensions();


    for f in WalkDir::new(".").into_iter().filter_map(|f| f.ok()) {
        if f.metadata().unwrap().is_file() {
            let path = f.path();

            stats.files.total += 1;

            println!("{:?}", path);
            if is_target_file(path, &target_dir, &ignore_dir) {
                // if path.extension().is_some() {
                let file_contents = fs::read_to_string(path);
                if file_contents.is_ok() {
                    let code = file_contents.unwrap();
                    if path.extension().is_some() {
                        let ext = path.extension().unwrap().to_str().unwrap();
                        if source_code_exts.contains(ext) {
                            stats.files.code += 1;
                            let lines = (code.matches("\n").count() + 1) as u64;
                            stats.lines.code += lines;
                            let length = code.len() as u64;
                            stats.chars.code += length;
                        } else if binary_exts.contains(ext) {
                            stats.files.binaries += 1;
                        } else if storage_exts.contains(ext) {
                            stats.files.storage += 1;
                        }
                    }
                    // Add one line for final line
                    let lines = (code.matches("\n").count() + 1) as u64;
                    stats.lines.total += lines;
                    let length = code.len() as u64;
                    stats.chars.total += length;
                }
            }
        }
    }

    view::display_stats(stats);
}
