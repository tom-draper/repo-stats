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
}

#[derive(Debug)]
pub struct Stats {
    files: FileCounts,
    memory: FileCounts,
    lines: CodeCounts,
    chars: CodeCounts,
    whitespace: CodeCounts,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            files: FileCounts {
                total: 0,
                code: 0,
                binaries: 0,
            },
            memory: FileCounts {
                total: 0,
                code: 0,
                binaries: 0,
            },
            lines: CodeCounts { total: 0, code: 0 },
            chars: CodeCounts { total: 0, code: 0 },
            whitespace: CodeCounts { total: 0, code: 0 },
        }
    }
}

impl Stats {
    fn avg_total_line_len(&self) -> u64 {
        if self.lines.total == 0 {
            return 0;
        }
        self.chars.total / self.lines.total
    }

    fn avg_code_line_len(&self) -> u64 {
        if self.lines.code == 0 {
            return 0;
        }
        self.chars.code / self.lines.code
    }
}

fn is_target_file(path: &Path, target_dir: &HashSet<&str>, ignore_dir: &HashSet<&str>) -> bool {
    let _path = path.to_str().unwrap();

    if !target_dir.is_empty() {
        for target in target_dir {
            if !_path.contains(*target) {
                return false;
            }
        }
    }

    if !ignore_dir.is_empty() {
        for ignore in ignore_dir {
            if _path.contains(*ignore) {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let mut stats = Stats::default();

    let ignore_dir = HashSet::from([]);
    let target_dir = HashSet::from([]);

    let source_code_exts = extensions::source_code_extensions();
    let binary_exts = extensions::binary_extensions();

    for f in WalkDir::new(".").into_iter().filter_map(|f| f.ok()) {
        let metadata = f.metadata().unwrap();
        if metadata.is_file() {
            let path = f.path();
            stats.files.total += 1;
            stats.memory.total += metadata.len();

            // println!("{:?}", path);
            if is_target_file(path, &target_dir, &ignore_dir) {
                let file_contents = fs::read_to_string(path);
                if file_contents.is_ok() {
                    let code = file_contents.unwrap();
                    // Add one line for final line
                    let lines = (code.matches("\n").count() + 1) as u64;
                    let whitespace = code.matches(' ').count() as u64;
                    let length = code.len() as u64;
                    stats.lines.total += lines;
                    stats.chars.total += length;
                    stats.whitespace.total += length;
                    if path.extension().is_some() {
                        let ext = path.extension().unwrap().to_str().unwrap();
                        if source_code_exts.contains(ext) {
                            stats.files.code += 1;
                            stats.lines.code += lines;
                            stats.chars.code += length;
                            stats.whitespace.code += whitespace;
                            stats.memory.code += metadata.len();
                        } else if binary_exts.contains(ext) {
                            stats.files.binaries += 1;
                            stats.memory.binaries += metadata.len();
                        }
                    }
                }
            }
        }
    }

    view::display_stats(stats);
}
