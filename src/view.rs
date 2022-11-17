use crate::{Stats, Total};
use num_format::{Locale, ToFormattedString};

fn display_total(total: &Total, indent: i32) {
    let padding = (0..indent).map(|_| " ").collect::<String>();
    println!("{}Files: {:>25}", padding, total.files.to_formatted_string(&Locale::en));
    println!("{}Lines: {:>25}", padding, total.lines.to_formatted_string(&Locale::en));
    println!("{}Characters: {:>20}", padding, total.chars.to_formatted_string(&Locale::en));
    println!("{}Average line length: {:>11.1}", padding, total.avg_line_len());
    println!("{}Whitespace: {:>19}%", padding, total.per_whitespace());
    println!("{}Memory: {:>24}", padding, total.memory);
}

pub fn display_stats(stats: Stats) {
    println!("---- Total -----------------------");
    display_total(&stats.total, 1);
    println!("----------------------------------\n");
    
    println!("---- Source code -----------------");
    println!(" Languages:");

    // Calc total chars across all languages
    let mut total_chars: u64 = 0;
    for (_, total) in &stats.code.languages {
        total_chars += total.chars;
    }

    let mut count_vec: Vec<_> = stats.code.languages.iter().collect();
    count_vec.sort_by(|a, b| b.1.chars.cmp(&a.1.chars));

    for (extension, language) in count_vec {
        let per = (language.chars as f32 / total_chars as f32) * 100.0;
        println!(" .{}: {:.2}%", extension, per);
        display_total(language, 5);
    }
    println!("----------------------------------\n");

    println!("Binaries: {} ({})", stats.binary.files.to_formatted_string(&Locale::en), stats.binary.memory);
}
