extern crate walkdir;
use std::fs;
mod view;
use walkdir::WalkDir;

#[derive(Debug)]
struct FileCounts {
    code: u64,
    binaries: u64,
}

#[derive(Debug)]
pub struct Stats {
    files: FileCounts,
    lines: u64,
    chars: u64,
}

impl Stats {
    fn avg_line_len(&self) -> u64 {
        self.chars / self.lines
    }
}

fn main() {
    let mut stats = Stats {
        files: FileCounts {
            code: 0,
            binaries: 0,
        },
        lines: 0,
        chars: 0,
    };

    for f in WalkDir::new(".").into_iter().filter_map(|f| f.ok()) {
        if f.metadata().unwrap().is_file() {
            let path = f.path();
            let ext = path.extension();

            if ext.is_some() {
                let file_contents = fs::read_to_string(path);
                match file_contents {
                    Ok(code) => {
                        println!("{:?} {:?}\n{:?}", path, ext.unwrap(), code);
                        // Add one line for final line
                        let lines = (code.matches("\n").count() + 1) as u64;
                        let length = code.len() as u64;
                        stats.lines += lines;
                        stats.chars += length;
                    }
                    Err(e) => {
                        println!("{:?}", e); //handled error
                    }
                }
                stats.files.code += 1;
            } else {
                stats.files.binaries += 1;
            }
        }
    }

    view::display_stats(stats);
}
