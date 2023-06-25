use std::time::Duration;

use crate::reporter::Report;
use crate::util::bytes_to_size;

pub fn pretty_print(report: &Report, elapsed: Duration) {
    let Report { sections, summary } = report;

    println!("{:>12.4} secs", elapsed.as_secs_f64());
    println!("┌───────────────────────────────────────────────────────────────────────────────────────┐");
    println!(
        "│ {:<25}{:>12}{:>12}{:>12}{:>12}{:>12} │",
        "Language", "files", "size", "blank", "comment", "code",
    );
    println!("├───────────────────────────────────────────────────────────────────────────────────────┤");

    for detail in sections {
        println!(
            "│ {:<25}{:>12}{:>12}{:>12}{:>12}{:>12} │",
            detail.language,
            detail.files,
            bytes_to_size(detail.bytes as f64),
            detail.blank,
            detail.comment,
            detail.code,
        );
    }

    println!("├───────────────────────────────────────────────────────────────────────────────────────┤");
    println!(
        "│ {:<25}{:>12}{:>12}{:>12}{:>12}{:>12} │",
        "Sum",
        summary.files,
        bytes_to_size(summary.bytes as f64),
        summary.blank,
        summary.comment,
        summary.code,
    );
    println!("└───────────────────────────────────────────────────────────────────────────────────────┘");
}

/// Pretty print to markdown file
use std::fs::File;
use std::io::Write;

pub fn pretty_print_to_markdown(report: &Report, elapsed: Duration) {
    let Report { sections, summary } = report;

    println!("{:>12.4} secs", elapsed.as_secs_f64());

    let mut markdown_output = String::new();
    // 输出标题
    markdown_output.push_str("# Code Line Counter\n\n");
    markdown_output.push_str("## Summary\n\n");
    // 输出花费时间
    markdown_output.push_str(&format!("Time Elapsed: {:>12.4} secs\n\n", elapsed.as_secs_f64()));
    markdown_output.push_str("Language | Files | Size | Blank | Comment | Code\n");
    markdown_output.push_str("| --- | --- | --- | --- | --- | --- |\n");

    for detail in sections {
        markdown_output.push_str(&format!(
            "| {:<25} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} |\n",
            detail.language,
            detail.files,
            bytes_to_size(detail.bytes as f64),
            detail.blank,
            detail.comment,
            detail.code,
        ));
    }

    markdown_output.push_str("| --- | --- | --- | --- | --- | --- |\n");
    markdown_output.push_str(&format!(
        "| {:<25} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} |\n",
        "Sum",
        summary.files,
        bytes_to_size(summary.bytes as f64),
        summary.blank,
        summary.comment,
        summary.code,
    ));

    // Save markdown output to file
    // 创建一个output文件夹，如果不存在的话
    // 加入错误处理
    std::fs::create_dir_all("output").expect("create output directory failed");
    let mut file = File::create("output/report.md").expect("create report.md failed");
    file.write_all(markdown_output.as_bytes())
        .expect("write report.md failed");
}
