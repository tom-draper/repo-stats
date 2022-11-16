use crate::Stats;
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    println!("Total:");
    println!("Files: {}", stats.files.total.to_formatted_string(&Locale::en));
    println!("Lines: {}", stats.lines.total.to_formatted_string(&Locale::en));
    println!("Characters: {}", stats.chars.total.to_formatted_string(&Locale::en));
    println!("Average line length: {}", stats.avg_total_line_len().to_formatted_string(&Locale::en));
    println!("Whitespace: {}%", (stats.chars.total / stats.whitespace.total).to_formatted_string(&Locale::en));
    println!("Memory: {} KB\n", (stats.memory.total/1024).to_formatted_string(&Locale::en));

    println!("Source code:");
    println!("Files: {}", stats.files.code.to_formatted_string(&Locale::en));
    println!("Lines: {}", stats.lines.code.to_formatted_string(&Locale::en));
    println!("Characters: {}", stats.chars.code.to_formatted_string(&Locale::en));
    println!("Average line length: {}", stats.avg_code_line_len().to_formatted_string(&Locale::en));
    println!("Whitespace: {}%", (stats.chars.code / stats.whitespace.code).to_formatted_string(&Locale::en));
    println!("Memory: {} KB\n", (stats.memory.code/1024).to_formatted_string(&Locale::en));

    println!("Binaries: {}", stats.files.binaries.to_formatted_string(&Locale::en));
}
