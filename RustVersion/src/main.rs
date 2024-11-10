use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use clap::Parser;

#[derive(Parser)]
#[command(name = "code-combiner")]
#[command(about = "Combine all code files into a single text file")]
struct Args {
    #[arg(long, default_value = "src")]
    src: String,

    #[arg(long, default_value = "combined_code.txt")]
    output: String,
}

fn combine_code_files(root_dir: &str, output_file: &str) -> io::Result<()> {
    let code_extensions: HashSet<_> = vec![
        "java", "py", "cpp", "h", "cs", "js", "ts", "txt"
    ].into_iter().collect();

    let mut output = File::create(output_file)?;
    let root_path = PathBuf::from(root_dir);

    for entry in walkdir::WalkDir::new(root_dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !e.path().to_string_lossy().contains(".idea")) {

        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        if let Some(extension) = entry.path().extension() {
            if code_extensions.contains(extension.to_string_lossy().as_ref()) {
                let relative_path = entry.path().strip_prefix(&root_path)
                    .unwrap_or(entry.path())
                    .to_string_lossy();

                let separator = "=".repeat(50);
                writeln!(output, "\n{}", separator)?;
                writeln!(output, "File: {}", relative_path)?;
                writeln!(output, "{}\n", separator)?;

                match fs::read_to_string(entry.path()) {
                    Ok(content) => {
                        writeln!(output, "{}\n", content)?;
                        println!("Added: {}", relative_path);
                    },
                    Err(e) => println!("Error reading {}: {}", relative_path, e),
                }
            }
        }
    }

    println!("\nAll code files have been combined into {}", output_file);
    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = combine_code_files(&args.src, &args.output) {
        eprintln!("Error: {}", e);
    }
}