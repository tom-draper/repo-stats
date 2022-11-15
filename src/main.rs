extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    for f in WalkDir::new(".").into_iter().filter_map(|f| f.ok()) {
        if f.metadata().unwrap().is_file() {
            let path = f.path();
            let ext = path.extension();

            if ext.is_some() {
                println!("{:?} {:?}", path, ext.unwrap());
            }
        }
    }
}