use crate::Stats;
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    print!(
    "Source files: {}\nSource code lines: {}\nSource code characters: {}\nSource code average line length: {}\nTotal lines: {}\nTotal characters: {}\nTotal average line length: {}\nBinaries {}",
        stats.files.code.to_formatted_string(&Locale::en),
        stats.lines.code.to_formatted_string(&Locale::en),
        stats.chars.code.to_formatted_string(&Locale::en),
        stats.avg_code_line_len().to_formatted_string(&Locale::en),
        stats.lines.total.to_formatted_string(&Locale::en),
        stats.chars.total.to_formatted_string(&Locale::en),
        stats.avg_total_line_len().to_formatted_string(&Locale::en),
        stats.files.binaries.to_formatted_string(&Locale::en),
    );
}
