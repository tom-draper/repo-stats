use crate::Stats;
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    print!(
        "Source files: {}\nBinaries: {}\nLines: {}\nCharacters: {}\nAverage line length: {}",
        stats.files.code.to_formatted_string(&Locale::en),
        stats.files.binaries.to_formatted_string(&Locale::en),
        stats.lines.to_formatted_string(&Locale::en),
        stats.chars.to_formatted_string(&Locale::en),
        stats.avg_line_len().to_formatted_string(&Locale::en),
    );
}
