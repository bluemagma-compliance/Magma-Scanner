use magma_scanner::scanner::Scanner;
use std::{path::Path, process::Command, env, ffi::OsStr};
use std::error::Error;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use walkdir::WalkDir;

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
    api_key: Option<String>,

    /// Organization ID
    #[arg(short, long)]
    organization_id: Option<String>,

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

    // Use command line args if provided, otherwise fall back to environment variables
    let api_key = cli.api_key
        .or_else(|| env::var("API_KEY").ok())
        .expect("API key must be provided via --api-key argument or API_KEY environment variable");

    let organization_id = cli.organization_id
        .or_else(|| env::var("ORGANIZATION_ID").ok())
        .expect("Organization ID must be provided via --organization-id argument or ORGANIZATION_ID environment variable");

    let report_id = cli.report_id
        .or_else(|| env::var("REPORT_ID").ok());

    let poll_interval = cli.poll_interval;
    let max_polls = cli.max_polls;

    println!("Target Directory: {}", target_dir);
    println!("API Key: {}", api_key);
    println!("Organization ID: {}", organization_id);
    if let Some(report_id) = &report_id {
        println!("Report ID: {}", report_id);
    }

    // Get git information
    let commit_hash = get_git_commit_hash().unwrap_or_else(|_| "unknown".to_string());
    let branch_name = get_git_branch_name().unwrap_or_else(|_| "unknown".to_string());
    let repo_url = get_git_repo_url().unwrap_or_else(|_| "unknown".to_string());

    println!("\n📦 Repository: {}", repo_url);
    println!("🔗 Commit Hash: {}", commit_hash);
    println!("🌿 Branch URL: {}", branch_name);

    // Find all supported files
    let files = find_files(&target_dir)?;
    println!("\n🔍 Scanning {} files", files.len());

    // Get distinct file extensions for API
    let file_extensions: Vec<String> = files.iter()
        .filter_map(|file| {
            Path::new(file)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_string())
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    // Create scanner
    let mut scanner = Scanner::new(
        api_key,
        organization_id,
        commit_hash.clone(),
        report_id,
    );

    // Initialize scan if needed
    let report_id = scanner.initialize_code_scan(file_extensions, &commit_hash, &branch_name, &repo_url).await?;
    println!("Using report ID: {}", report_id);

    // Start continuous scanning
    scanner.start_continuous_scan(files, poll_interval, max_polls).await?;

    Ok(())
}

/// Find all supported files in the target directory and all subdirectories
fn find_files(target_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Extensions for supported languages
    let extensions = [
        "rs", "js", "py", "go", "ts", "java", "cpp", "h", "hpp", "cc", "rb", "php"
    ];

    // Directories to ignore
    let ignore_dirs = ["node_modules", "target", "dist", "build"];

    println!("Searching for files in directory and subdirectories: {}", target_dir);

    let mut files = Vec::new();

    // Ensure the target directory exists
    let target_path = Path::new(target_dir);
    if !target_path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Target directory not found: {}", target_dir)
        )));
    }

    // Use WalkDir to recursively walk the directory tree
    // This will automatically walk through all subdirectories
    let walker = WalkDir::new(target_dir)
        .follow_links(true)  // Follow symbolic links
        .into_iter();

    // Process each entry
    for entry_result in walker {
        // Handle any errors during directory traversal
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error accessing path: {}", e);
                continue;
            }
        };

        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();

        // Skip directories we want to ignore
        if path.is_dir() {
            let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
            if ignore_dirs.iter().any(|&ignore| dir_name == ignore) {
                println!("Skipping directory: {}", path.display());
                // This will skip the directory and all its contents
                continue;
            }
        }
        // Only process files
        else if path.is_file() {
            // Check if the file has one of our supported extensions
            if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                if extensions.contains(&ext) {
                    println!("Found file: {}", path_str);
                    files.push(path_str);
                }
            }
        }
    }

    println!("Found {} files", files.len());

    // If no files were found, print a warning
    if files.is_empty() {
        println!("Warning: No files with supported extensions found in {}", target_dir);
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

/// Get the git repository URL
fn get_git_repo_url() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()?;

    if output.status.success() {
            let url = String::from_utf8(output.stdout)?;
            Ok(url.trim().to_string())
    } else {
        Err("Failed to get git repository URL".into())
    }
}
