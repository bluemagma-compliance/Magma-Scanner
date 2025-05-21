use magma_scanner::scanner::Scanner;
use std::{path::Path, process::Command};
use glob::glob;
use std::error::Error;
use clap::{Parser, Subcommand};
use dotenv::dotenv;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Target directory to scan
    #[arg(short, long, default_value = ".")]
    target: String,

    /// API key for authentication
    #[arg(short, long)]
    api_key: String,

    /// Organization ID
    #[arg(short, long)]
    organization_id: String,

    /// Report ID (optional)
    #[arg(short, long)]
    report_id: Option<String>,

    /// Polling interval in seconds
    #[arg(short, long, default_value_t = 5)]
    poll_interval: u64,

    /// Maximum number of polling iterations
    #[arg(short, long, default_value_t = 20)]
    max_polls: usize,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a repository
    Scan {
        /// Target directory to scan
        #[arg(short, long, default_value = ".")]
        target: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file if it exists
    dotenv().ok();

    let cli = Cli::parse();

    let target_dir = cli.target;
    let api_key = cli.api_key;
    let organization_id = cli.organization_id;
    let report_id = cli.report_id;
    let poll_interval = cli.poll_interval;
    let max_polls = cli.max_polls;

    println!("Target Directory: {}", target_dir);
    println!("API Key: {}", api_key);
    println!("Organization ID: {}", organization_id);
    if let Some(report_id) = &report_id {
        println!("Report ID: {}", report_id);
    }

    // Get git information
    let commit_hash = get_git_commit_hash()?;
    let branch_name = get_git_branch_name()?;
    let repo_name = get_git_repo_name()?;

    println!("\n📦 Repository: {}", repo_name);
    println!("🔗 Commit Hash: {}", commit_hash);
    println!("🌿 Branch Name: {}", branch_name);

    // Find all supported files
    let files = find_files(&target_dir)?;
    println!("\n🔍 Scanning {} files", files.len());

    // Get file extensions for API
    let file_extensions: Vec<String> = files.iter()
        .filter_map(|file| {
            Path::new(file)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_string())
        })
        .collect();

    // Create scanner
    let mut scanner = Scanner::new(
        api_key,
        organization_id,
        commit_hash.clone(),
        report_id,
    );

    // Initialize scan if needed
    let report_id = scanner.initialize_code_scan(file_extensions, &commit_hash, &branch_name, &repo_name).await?;
    println!("Using report ID: {}", report_id);

    // Start continuous scanning
    scanner.start_continuous_scan(files, poll_interval, max_polls).await?;

    Ok(())
}

/// Find all supported files in the target directory
fn find_files(target_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Extensions for supported languages
    let extensions = [
        "rs", "js", "py", "go", "ts", "java", "cpp", "h", "hpp", "cc", "rb", "php"
    ];

    let pattern = format!("{}/**/*.{{{}}}", target_dir, extensions.join(","));
    let ignore_dirs = ["**/node_modules/**", "**/target/**", "**/dist/**", "**/build/**"];

    let mut files = Vec::new();

    for entry in glob(&pattern)? {
        match entry {
            Ok(path) => {
                let path_str = path.to_string_lossy().to_string();

                // Skip ignored directories
                if ignore_dirs.iter().any(|ignore| {
                    let glob_pattern = glob::Pattern::new(ignore).unwrap();
                    glob_pattern.matches(&path_str)
                }) {
                    continue;
                }

                files.push(path_str);
            },
            Err(e) => eprintln!("Error matching glob pattern: {}", e),
        }
    }

    Ok(files)
}

/// Get the current git commit hash
fn get_git_commit_hash() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()?;

    if output.status.success() {
        let hash = String::from_utf8(output.stdout)?;
        Ok(hash.trim().to_string())
    } else {
        Err("Failed to get git commit hash".into())
    }
}

/// Get the current git branch name
fn get_git_branch_name() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if output.status.success() {
        let branch = String::from_utf8(output.stdout)?;
        Ok(branch.trim().to_string())
    } else {
        Err("Failed to get git branch name".into())
    }
}

/// Get the git repository name
fn get_git_repo_name() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if output.status.success() {
        let repo_path = String::from_utf8(output.stdout)?;
        let repo_name = Path::new(repo_path.trim())
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or("Failed to extract repository name")?;

        Ok(repo_name.to_string())
    } else {
        Err("Failed to get git repository name".into())
    }
}
