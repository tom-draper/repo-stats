use crate::Stats;
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    println!("Total");
    println!("Files: {}", stats.files.total.to_formatted_string(&Locale::en));
    println!("Lines: {}", stats.lines.total.to_formatted_string(&Locale::en));
    println!("Characters: {}", stats.chars.total.to_formatted_string(&Locale::en));
    println!("Average line length: {}", stats.avg_total_line_len().to_formatted_string(&Locale::en));
    println!("Memory: {} KB\n", (stats.memory.total/1024).to_formatted_string(&Locale::en));

    println!("Source code");
    println!("Files: {}", stats.files.code.to_formatted_string(&Locale::en));
    println!("Lines: {}", stats.lines.code.to_formatted_string(&Locale::en));
    println!("Characters: {}", stats.chars.code.to_formatted_string(&Locale::en));
    println!("Average line length: {}", stats.avg_code_line_len().to_formatted_string(&Locale::en));
    println!("Memory: {} KB\n", (stats.memory.code/1024).to_formatted_string(&Locale::en));

    println!("Storage");
    println!("Files: {}", stats.files.storage.to_formatted_string(&Locale::en));
    println!("Lines: {}", stats.lines.storage.to_formatted_string(&Locale::en));
    println!("Characters: {}", stats.chars.storage.to_formatted_string(&Locale::en));
    println!("Average line length: {}", stats.avg_storage_line_len().to_formatted_string(&Locale::en));
    println!("Memory: {} KB\n", (stats.memory.storage/1024).to_formatted_string(&Locale::en));

    println!("Binaries: {}", stats.files.binaries.to_formatted_string(&Locale::en));
}
