extern crate bytesize;
extern crate walkdir;
use bytesize::ByteSize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};
mod extensions;
mod remote;
mod view;
use clap::Parser;
use indicatif::ProgressBar;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Stats {
    total: Total,
    code: Code,
    binary: Binary,
}

#[derive(Debug)]
struct Total {
    files: u64,
    memory: ByteSize,
    lines: u64,
    chars: u64,
    whitespace: u64,
    crlf: u64,
}

#[derive(Debug)]
struct Code {
    total: Total,
    languages: HashMap<String, Total>,
}

impl Default for Total {
    fn default() -> Self {
        Self {
            files: 0,
            memory: ByteSize(0),
            lines: 0,
            chars: 0,
            whitespace: 0,
            crlf: 0,
        }
    }
}

impl Total {
    fn inc(
        &mut self,
        files: u64,
        memory: ByteSize,
        lines: u64,
        chars: u64,
        whitespace: u64,
        crlf: u64,
    ) {
        self.files += files;
        self.memory += memory;
        self.lines += lines;
        self.chars += chars;
        self.whitespace += whitespace;
        self.crlf += crlf;
    }
}

#[derive(Debug)]
struct Binary {
    files: u64,
    memory: ByteSize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            total: Total::default(),
            code: Code {
                total: Total::default(),
                languages: HashMap::new(),
            },
            binary: Binary {
                files: 0,
                memory: ByteSize(0),
            },
        }
    }
}

impl Total {
    fn avg_line_len(&self) -> f32 {
        if self.lines == 0 {
            return 0.0;
        }
        (self.chars / self.lines) as f32
    }

    fn percent_whitespace(&self) -> f32 {
        if self.chars == 0 {
            return 0.0;
        }
        (self.whitespace / self.chars) as f32 * 100.0
    }

    fn percent_clrf(&self) -> f32 {
        if self.files == 0 {
            return 0.0;
        }
        (self.crlf / self.files) as f32 * 100.0
    }
}

fn is_target_file(path: &Path, target_dir: &HashSet<&str>, ignore_dir: &HashSet<&str>) -> bool {
    let _path = path.to_str().unwrap();

    // False if not target
    if !target_dir.is_empty() {
        for target in target_dir {
            if !_path.contains(*target) {
                return false;
            }
        }
    }

    // False if ignored
    if !ignore_dir.is_empty() {
        for ignore in ignore_dir {
            if _path.contains(*ignore) {
                return false;
            }
        }
    }

    return true;
}

fn repo_stats(path: &str, target_dir: HashSet<&str>, ignore_dir: HashSet<&str>) -> Stats {
    let mut stats = Stats::default();
    let extensions = extensions::Extensions::default();

    let num_files = WalkDir::new(path).into_iter().count() as u64;
    let progress_bar = ProgressBar::new(num_files);

    for f in WalkDir::new(path).into_iter().filter_map(|f| f.ok()) {
        let metadata = f.metadata().unwrap();
        if !metadata.is_file() {
            continue;
        }
        let path = f.path();
        stats.total.files += 1;
        let filesize = ByteSize(metadata.len());
        stats.total.memory += filesize;

        if !is_target_file(path, &target_dir, &ignore_dir) {
            continue;
        }

        if path.extension().is_none() {
            continue;
        }

        let file_contents = fs::read_to_string(path);
        let readable = file_contents.is_ok();

        let ext = path.extension().unwrap().to_str().unwrap();
        if readable && extensions.is_source_code(ext) {
            let code = file_contents.unwrap();
            // Add one line for final line
            let lines = (code.matches("\n").count() + 1) as u64;
            let crlf = (code.matches("\r\n").count() > 0) as u64;
            let whitespace = code.matches(' ').count() as u64;
            let length = code.len() as u64;
            stats
                .total
                .inc(0, ByteSize(0), lines, length, whitespace, crlf);
            stats
                .code
                .total
                .inc(1, filesize, lines, length, whitespace, crlf);

            let language = stats.code.languages.entry(ext.to_owned()).or_default();
            language.inc(1, filesize, lines, length, whitespace, crlf);
        } else if extensions.is_binary(ext) {
            stats.binary.files += 1;
            stats.binary.memory += filesize;
        }
        progress_bar.inc(1);
    }
    stats
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value("."))]
    path: String,

    #[arg(short, long, default_value(""))]
    repo: String,

    #[arg(short, long, default_value(""))]
    ignore: String,

    #[arg(short, long, default_value(""))]
    target: String,
}

fn main() {
    let args = Args::parse();

    let mut path = args.path;

    // Build any ignore and target dirs from args
    let ignore_dir: HashSet<&str>;
    if args.ignore != "" {
        ignore_dir = args.ignore.split(",").collect();
    } else {
        ignore_dir = HashSet::from([])
    }
    let mut target_dir: HashSet<&str>;
    if args.target != "" {
        target_dir = args.target.split(",").collect();
    } else {
        target_dir = HashSet::from([])
    }

    // Build repo info if repo arg has been specified
    let mut repo_name = args.repo.as_str();
    if repo_name.contains("/") {
        repo_name = args.repo.split("/").collect::<Vec<&str>>()[1];
    } else if repo_name != "" {
        println!("Error: Repo name invalid\nRequired pattern: <user>/<repo>");
        return;
    }
    let repo_dir = format!("temp-{}", repo_name);
    // If repo has been specified, use as target dir instead
    if repo_name != "" {
        remote::clone_repo(&args.repo);
        target_dir = HashSet::from([repo_dir.as_str()]);
        path = String::from(".");
    }

    let stats = repo_stats(&path, target_dir, ignore_dir);

    // Clean up any temp files created
    if repo_name != "" {
        fs::remove_dir_all(&repo_dir).unwrap();
    }

    view::display_stats(stats);
}

