
use crate::Stats;
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    println!("---- Total ------------------------");
    println!(" Files: {:>26}", stats.files.total.to_formatted_string(&Locale::en));
    println!(" Lines: {:>26}", stats.lines.total.to_formatted_string(&Locale::en));
    println!(" Characters: {:>21}", stats.chars.total.to_formatted_string(&Locale::en));
    println!(" Average line length: {:>12.1}", stats.avg_total_line_len());
    println!(" Whitespace: {:>20}%", (stats.whitespace.total / stats.chars.total));
    println!(" Memory: {:>25}", stats.memory.total);
    println!("-----------------------------------\n");

    println!("---- Source code ------------------");
    println!(" Languages:");
    let total: u64 = stats.extensions.values().sum();
    for (extension, size) in &stats.extensions {
        let per = (*size as f32 / total as f32) * 100.0;
        println!("   {}: {:.2}%", extension, per);
    }
    println!(" Files: {:>26}", stats.files.code.to_formatted_string(&Locale::en));
    println!(" Lines: {:>26}", stats.lines.code.to_formatted_string(&Locale::en));
    println!(" Characters: {:>21}", stats.chars.code.to_formatted_string(&Locale::en));
    println!(" Average line length: {:>12.1}", stats.avg_code_line_len());
    println!(" Whitespace: {:>20}%", (stats.whitespace.code / stats.chars.code));
    println!(" Memory: {:>25}", stats.memory.code);
    println!("-----------------------------------\n");

    println!("Binaries: {} ({})", stats.files.binaries.to_formatted_string(&Locale::en), stats.memory.binaries);
}
