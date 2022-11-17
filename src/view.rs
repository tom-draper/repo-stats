use std::collections::HashMap;

use crate::{Stats, extensions};
use num_format::{Locale, ToFormattedString};

pub fn display_stats(stats: Stats) {
    println!("---- Total -----------------------");
    println!(" Files: {:>25}", stats.total.files.to_formatted_string(&Locale::en));
    println!(" Lines: {:>25}", stats.total.lines.to_formatted_string(&Locale::en));
    println!(" Characters: {:>20}", stats.total.chars.to_formatted_string(&Locale::en));
    println!(" Average line length: {:>11.1}", stats.total.avg_line_len());
    println!(" Whitespace: {:>19}%", stats.total.per_whitespace());
    println!(" Memory: {:>24}", stats.total.memory);
    println!("----------------------------------\n");

    println!("---- Source code -----------------");
    println!(" Languages:");

    let mut total_chars: u64 = 0;
    for (_, total) in &stats.code.languages {
        total_chars += total.chars;
    }

    let mut count_vec: Vec<_> = stats.code.languages.iter().collect();
    count_vec.sort_by(|a, b| b.1.chars.cmp(&a.1.chars));

    for (extension, language) in count_vec {
        let per = (language.chars as f32 / total_chars as f32) * 100.0;
        println!(" .{}: {:.2}%", extension, per);
        println!("     Files: {:>25}", language.files.to_formatted_string(&Locale::en));
        println!("     Lines: {:>25}", language.lines.to_formatted_string(&Locale::en));
        println!("     Characters: {:>20}", language.chars.to_formatted_string(&Locale::en));
        println!("     Average line length: {:>11.1}", language.avg_line_len());
        println!("     Whitespace: {:>19}%", language.per_whitespace());
        println!("     Memory: {:>24}", language.memory);
    }
    println!("----------------------------------\n");

    println!("Binaries: {} ({})", stats.binary.files.to_formatted_string(&Locale::en), stats.binary.memory);
}
