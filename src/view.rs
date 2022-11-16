
use crate::Stats;
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    println!("---- Total -----------------------");
    println!(" Files: {:>25}", stats.files.total.to_formatted_string(&Locale::en));
    println!(" Lines: {:>25}", stats.lines.total.to_formatted_string(&Locale::en));
    println!(" Characters: {:>20}", stats.chars.total.to_formatted_string(&Locale::en));
    println!(" Average line length: {:>11.1}", stats.avg_total_line_len());
    println!(" Whitespace: {:>19}%", (stats.whitespace.total / stats.chars.total));
    println!(" Memory: {:>24}", stats.memory.total);
    println!("----------------------------------\n");

    println!("---- Source code -----------------");
    println!(" Languages:");
    let total: u64 = stats.extensions.values().sum();
    let mut count_vec: Vec<_> = stats.extensions.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (extension, size) in count_vec {
        let per = (*size as f32 / total as f32) * 100.0;
        println!("   .{}: {:.2}%", extension, per);
    }
    println!(" Files: {:>25}", stats.files.code.to_formatted_string(&Locale::en));
    println!(" Lines: {:>25}", stats.lines.code.to_formatted_string(&Locale::en));
    println!(" Characters: {:>20}", stats.chars.code.to_formatted_string(&Locale::en));
    println!(" Average line length: {:>11.1}", stats.avg_code_line_len());
    println!(" Whitespace: {:>19}%", (stats.whitespace.code / stats.chars.code));
    println!(" Memory: {:>24}", stats.memory.code);
    println!("----------------------------------\n");

    println!("Binaries: {} ({})", stats.files.binaries.to_formatted_string(&Locale::en), stats.memory.binaries);
}
