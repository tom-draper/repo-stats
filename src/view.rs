use crate::{Stats, Total};
use colored::Colorize;
use num_format::{Locale, ToFormattedString};

fn display_total(total: &Total, indent: i32) {
    let width = 28 - indent as usize;
    let padding = (0..indent).map(|_| " ").collect::<String>();
    println!(
        "{}Files: {:>width$}",
        padding,
        format!("{}", total.files.to_formatted_string(&Locale::en)).yellow(),
        width = width
    );
    println!(
        "{}Lines: {:>width$}",
        padding,
        format!("{}", total.lines.to_formatted_string(&Locale::en)).yellow(),
        width = width
    );
    println!(
        "{}Characters: {:>width$}",
        padding,
        format!("{}", total.chars.to_formatted_string(&Locale::en)).yellow(),
        width = width - 5
    );
    println!(
        "{}Average line length: {:>width$}",
        padding,
        format!("{:.1}", total.avg_line_len()).yellow(),
        width = width - 14
    );
    println!(
        "{}Whitespace: {:>width$}",
        padding,
        format!("{:.2}%", total.per_whitespace()).yellow(),
        width = width - 5
    );
    println!(
        "{}Memory: {:>width$}",
        padding,
        format!("{}", total.memory).yellow(),
        width = width - 1
    );
    println!(
        "{}Carridge returns: {:>width$}",
        padding,
        format!("{}", total.cr).yellow(),
        width = width - 11
    );
}

pub fn display_stats(stats: Stats) {
    if stats.total.files > 0 {
        println!("---- Total -------------------------");
        display_total(&stats.total, 1);
        println!("------------------------------------\n");
    } else {
        println!("No files found.")
    }

    if stats.code.total.files > 0 {
        println!("---- Source code -------------------");
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
            println!(" .{}: {:.2}%", format!("{}", extension).yellow(), per);
            display_total(language, 5);
        }
        println!("------------------------------------\n");
    }

    if stats.binary.files > 0 {
        println!("---- Binaries ----------------------");
        println!(
            " Files: {}",
            stats.binary.files.to_formatted_string(&Locale::en)
        );
        println!(" Memory: {}", stats.binary.memory);
        println!("------------------------------------\n");
    }
}
