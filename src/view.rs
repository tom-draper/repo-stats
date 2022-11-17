use crate::Stats;
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    println!("---- Total -----------------------");
    println!(" Files: {:>25}", stats.total.files.to_formatted_string(&Locale::en));
    println!(" Lines: {:>25}", stats.total.lines.to_formatted_string(&Locale::en));
    println!(" Characters: {:>20}", stats.total.chars.to_formatted_string(&Locale::en));
    println!(" Average line length: {:>11.1}", stats.avg_total_line_len());
    println!(" Whitespace: {:>19}%", (stats.total.whitespace / stats.total.chars));
    println!(" Memory: {:>24}", stats.total.memory);
    println!("----------------------------------\n");

    println!("---- Source code -----------------");
    println!(" Languages:");
    let total: u64 = stats.code.extensions.values().sum();
    let mut count_vec: Vec<_> = stats.code.extensions.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (extension, size) in count_vec {
        let per = (*size as f32 / total as f32) * 100.0;
        println!("   .{}: {:.2}%", extension, per);
    }
    println!(" Files: {:>25}", stats.code.files.to_formatted_string(&Locale::en));
    println!(" Lines: {:>25}", stats.code.lines.to_formatted_string(&Locale::en));
    println!(" Characters: {:>20}", stats.code.chars.to_formatted_string(&Locale::en));
    println!(" Average line length: {:>11.1}", stats.avg_code_line_len());
    println!(" Whitespace: {:>19}%", (stats.code.whitespace / stats.code.chars));
    println!(" Memory: {:>24}", stats.code.memory);
    println!("----------------------------------\n");

    println!("Binaries: {} ({})", stats.binary.files.to_formatted_string(&Locale::en), stats.binary.memory);
}
