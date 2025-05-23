mod test_utils;

use test_utils::{create_test_scanner, ensure_test_repo, test_repo_path, RUST_SAMPLE};
use magma_scanner::types::TreeSitterQuery;
use std::fs;
use std::time::{Duration, Instant};

// Helper function to create a large file for performance testing
fn create_large_file(size_multiplier: usize) -> String {
    ensure_test_repo();
    let file_path = test_repo_path().join("large_test.rs");

    // Create a large file by repeating the sample multiple times
    let content = RUST_SAMPLE.repeat(size_multiplier);
    fs::write(&file_path, &content).expect("Failed to write large test file");

    file_path.to_string_lossy().to_string()
}

// Helper function to create many queries for performance testing
fn create_many_queries(count: usize) -> Vec<TreeSitterQuery> {
    let base_queries = vec![
        "(struct_item name: (type_identifier) @struct_name)",
        "(function_item name: (identifier) @function_name)",
        "(string_literal) @string",
        "(integer_literal) @integer",
        "(call_expression function: (identifier) @function_call)",
        "(binary_expression) @binary_op",
        "(parameter) @param",
        "(field_declaration) @field",
        "(comment) @comment",
        "(macro_invocation) @macro",
    ];

    let mut queries = Vec::with_capacity(count);

    for i in 0..count {
        let query_index = i % base_queries.len();
        queries.push(TreeSitterQuery {
            question_id: format!("perf_query_{}", i),
            file_type: ".rs".to_string(),
            query: base_queries[query_index].to_string(),
            object_id: format!("perf_object_{}", i),
            prompt: format!("Performance test query {}", i),
            reasoning: "Testing query performance".to_string(),
        });
    }

    queries
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to measure execution time
    fn measure_execution_time<F, T>(f: F) -> (T, Duration)
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    // Helper function to measure async execution time
    async fn measure_async_execution_time<F, Fut, T>(f: F) -> (T, Duration)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let start = Instant::now();
        let result = f().await;
        let duration = start.elapsed();
        (result, duration)
    }

    #[tokio::test]
    async fn test_parse_performance_with_large_file() {
        let scanner = create_test_scanner();

        // Create files of different sizes
        let small_file = create_large_file(1);  // 1x size
        let medium_file = create_large_file(10); // 10x size
        let large_file = create_large_file(50); // 50x size

        // Measure parsing time for each file
        let (small_result, small_duration) = measure_execution_time(|| {
            scanner.parse_file(&small_file, "rust")
        });

        let (medium_result, medium_duration) = measure_execution_time(|| {
            scanner.parse_file(&medium_file, "rust")
        });

        let (large_result, large_duration) = measure_execution_time(|| {
            scanner.parse_file(&large_file, "rust")
        });

        // Verify all files were parsed successfully
        assert!(small_result.is_some());
        assert!(medium_result.is_some());
        assert!(large_result.is_some());

        // Print performance results
        println!("Small file parsing time: {:?}", small_duration);
        println!("Medium file parsing time: {:?}", medium_duration);
        println!("Large file parsing time: {:?}", large_duration);

        // Just print the results - don't assert on timing as it can be unpredictable
        // The first parse might be slower due to JIT compilation, caching, etc.
        println!("Small file size: {} bytes", RUST_SAMPLE.repeat(1).len());
        println!("Medium file size: {} bytes", RUST_SAMPLE.repeat(10).len());
        println!("Large file size: {} bytes", RUST_SAMPLE.repeat(50).len());
    }

    #[tokio::test]
    async fn test_query_performance_with_many_queries() {
        let scanner = create_test_scanner();
        let file_path = create_large_file(5); // 5x size

        // Create different numbers of queries
        let few_queries = create_many_queries(5);
        let some_queries = create_many_queries(20);
        let many_queries = create_many_queries(50);

        // Measure scan time for different numbers of queries
        let (_, few_duration) = measure_async_execution_time(||
            scanner.scan_files(vec![file_path.clone()], few_queries.clone())
        ).await;

        let (_, some_duration) = measure_async_execution_time(||
            scanner.scan_files(vec![file_path.clone()], some_queries.clone())
        ).await;

        let (_, many_duration) = measure_async_execution_time(||
            scanner.scan_files(vec![file_path.clone()], many_queries.clone())
        ).await;

        // Print performance results
        println!("Scan with 5 queries: {:?}", few_duration);
        println!("Scan with 20 queries: {:?}", some_duration);
        println!("Scan with 50 queries: {:?}", many_duration);

        // Verify that scan time scales with number of queries
        // This is a rough check - actual scaling might not be perfectly linear
        assert!(some_duration > few_duration);
        assert!(many_duration > some_duration);
    }

    #[tokio::test]
    async fn test_ast_caching_performance() {
        let scanner = create_test_scanner();
        let file_path = create_large_file(10); // 10x size

        // Create some queries
        let queries = create_many_queries(10);

        // First scan - should parse and cache the AST
        let (_, first_scan_duration) = measure_async_execution_time(||
            scanner.scan_files(vec![file_path.clone()], queries.clone())
        ).await;

        // Second scan - should use the cached AST
        let (_, second_scan_duration) = measure_async_execution_time(||
            scanner.scan_files(vec![file_path.clone()], queries.clone())
        ).await;

        // Print performance results
        println!("First scan (parsing + querying): {:?}", first_scan_duration);
        println!("Second scan (cached, querying only): {:?}", second_scan_duration);

        // Verify that the second scan is faster due to caching
        assert!(second_scan_duration < first_scan_duration);
    }

    #[tokio::test]
    async fn test_performance_with_multiple_files() {
        let scanner = create_test_scanner();
        ensure_test_repo();

        // Create multiple files
        let files = (0..10).map(|i| {
            let file_path = test_repo_path().join(format!("multi_test_{}.rs", i));
            fs::write(&file_path, RUST_SAMPLE).expect("Failed to write test file");
            file_path.to_string_lossy().to_string()
        }).collect::<Vec<_>>();

        // Create some queries
        let queries = create_many_queries(5);

        // Measure scan time for multiple files
        let (results, duration) = measure_async_execution_time(||
            scanner.scan_files(files.clone(), queries.clone())
        ).await;

        // Print performance results
        println!("Scan of 10 files with 5 queries: {:?}", duration);
        println!("Found {} matches", results.len());

        // Verify that we got results from multiple files
        let unique_files = results.iter()
            .map(|r| &r.file)
            .collect::<std::collections::HashSet<_>>();

        println!("Results from {} unique files", unique_files.len());
        assert!(unique_files.len() > 1);
    }
}
