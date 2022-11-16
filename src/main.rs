extern crate walkdir;
extern crate bytesize;
use bytesize::ByteSize;
use std::{collections::{HashSet, HashMap}, fs, path::Path};
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
struct FileSizeCounts {
    total: ByteSize,
    code: ByteSize,
    binaries: ByteSize,
}

#[derive(Debug)]
pub struct Stats {
    files: FileCounts,
    memory: FileSizeCounts,
    lines: CodeCounts,
    chars: CodeCounts,
    whitespace: CodeCounts,
    extensions: HashMap<String, u64>
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            files: FileCounts {
                total: 0,
                code: 0,
                binaries: 0,
            },
            memory: FileSizeCounts {
                total: ByteSize(0),
                code: ByteSize(0),
                binaries: ByteSize(0),
            },
            lines: CodeCounts { total: 0, code: 0 },
            chars: CodeCounts { total: 0, code: 0 },
            whitespace: CodeCounts { total: 0, code: 0 },
            extensions: HashMap::new()
        }
    }
}

impl Stats {
    fn avg_total_line_len(&self) -> f32 {
        if self.lines.total == 0 {
            return 0.0;
        }
        self.chars.total as f32 / self.lines.total as f32
    }

    fn avg_code_line_len(&self) -> f32 {
        if self.lines.code == 0 {
            return 0.0;
        }
        self.chars.code as f32 / self.lines.code as f32
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

    let extensions = extensions::Extensions::default();
    let target_dir = HashSet::from([]);
    let ignore_dir = HashSet::from(["target"]);

    for f in WalkDir::new(".").into_iter().filter_map(|f| f.ok()) {
        let metadata = f.metadata().unwrap();
        if metadata.is_file() {
            let path = f.path();
            stats.files.total += 1;
            let filesize = ByteSize(metadata.len());
            stats.memory.total += filesize;

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
                        if extensions.is_source_code(ext) {
                            stats.files.code += 1;
                            stats.lines.code += lines;
                            stats.chars.code += length;
                            stats.whitespace.code += whitespace;
                            stats.memory.code += filesize;
                            stats.extensions.entry(ext.to_owned()).and_modify(|ext| *ext += length).or_insert(0);
                        } else if extensions.is_binary(ext) {
                            stats.files.binaries += 1;
                            stats.memory.binaries += filesize;
                        }
                    }
                }
            }
        }
    }

    view::display_stats(stats);
}
