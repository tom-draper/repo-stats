use crate::Stats;
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    print!("{} source files\n{} binaries\n{} lines\n{} characters\n", 

    stats.files.code.to_formatted_string(&Locale::en),
    stats.files.binaries.to_formatted_string(&Locale::en),
    stats.lines.to_formatted_string(&Locale::en),
    stats.chars.to_formatted_string(&Locale::en));
}