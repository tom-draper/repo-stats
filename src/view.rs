use crate::Stats;
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    println!("Total files: {}", stats.files.total.to_formatted_string(&Locale::en));
    println!("Total lines: {}", stats.lines.total.to_formatted_string(&Locale::en));
    println!("Total characters: {}", stats.chars.total.to_formatted_string(&Locale::en));
    println!("Total average line length: {}\n", stats.avg_total_line_len().to_formatted_string(&Locale::en));

    println!("Source code files: {}", stats.files.code.to_formatted_string(&Locale::en));
    println!("Source code lines: {}", stats.lines.code.to_formatted_string(&Locale::en));
    println!("Source code characters: {}", stats.chars.code.to_formatted_string(&Locale::en));
    println!("Source code average line length: {}\n", stats.avg_code_line_len().to_formatted_string(&Locale::en));

    println!("Storage files: {}", stats.files.storage.to_formatted_string(&Locale::en));
    println!("Storage lines: {}", stats.lines.storage.to_formatted_string(&Locale::en));
    println!("Storage characters: {}", stats.chars.storage.to_formatted_string(&Locale::en));
    println!("Storage average line length: {}\n", stats.avg_storage_line_len().to_formatted_string(&Locale::en));

    println!("Binaries {}", stats.files.binaries.to_formatted_string(&Locale::en));
}
