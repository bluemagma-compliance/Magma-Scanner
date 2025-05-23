# Magma Scanner Tests

This directory contains unit tests for the Magma Scanner project, focusing on TreeSitter query functionality.

## Test Structure

- `test_utils/mod.rs`: Contains utility functions and sample code for different languages
- `scanner_tests.rs`: Tests for the Scanner functionality
- `query_tests.rs`: Tests for TreeSitter queries across different languages
- `performance_tests.rs`: Tests for measuring performance of various operations
- `integration_tests.rs`: Tests for API interactions (currently disabled)

## Test Repository

The tests create a test repository in `tests/test_repo` with sample files for each supported language. This repository is used to test TreeSitter queries against real code.

## Running Tests

To run all tests:

```bash
cargo test
```

To run a specific test:

```bash
cargo test test_rust_queries
```

To run a specific test file:

```bash
cargo test --test performance_tests
```

To run a specific test in a specific file:

```bash
cargo test --test performance_tests test_ast_caching_performance
```

## Direct Query Injection

The tests demonstrate how to directly inject TreeSitter queries without requiring API connectivity. This is useful for testing and development.

Example:

```rust
// Create a query to find struct definitions
let query = TreeSitterQuery {
    question_id: "test_question".to_string(),
    file_type: ".rs".to_string(),
    query: "(struct_item name: (type_identifier) @struct_name)".to_string(),
    object_id: "test_object".to_string(),
    prompt: "Find struct definitions".to_string(),
    reasoning: "Testing struct detection".to_string(),
};

// Run the scan
let results = scanner.scan_files(vec![file_path], vec![query]).await;
```

## Testing TreeSitter Queries

The test suite includes examples of TreeSitter queries for different languages:

- Rust: Finding structs, functions, and string literals
- JavaScript: Finding classes and methods
- Python: Finding classes and functions
- Complex queries with predicates

## Performance Tests

The performance tests measure various aspects of the scanner's performance:

- **File Parsing Performance**: Tests parsing files of different sizes
- **Query Performance**: Tests running different numbers of queries
- **AST Caching Performance**: Tests the effectiveness of AST caching
- **Multiple Files Performance**: Tests scanning multiple files

Example of running a performance test:

```bash
cargo test --test performance_tests test_ast_caching_performance
```

## Integration Tests

The integration tests verify the scanner's interaction with external APIs:

- **API Initialization**: Tests initializing a code scan report
- **Query Fetching**: Tests fetching available queries from the API
- **Evidence Posting**: Tests posting evidence to the API
- **Error Handling**: Tests handling API errors
- **Continuous Scanning**: Tests the full scanning workflow

Most of these tests are currently marked as `#[ignore]` because they require a mock server that matches the exact API endpoints. To run the non-ignored tests:

```bash
cargo test --test integration_tests
```

To run all integration tests, including the ignored ones:

```bash
cargo test --test integration_tests -- --include-ignored
```

## Adding New Tests

To add a new test for a language:

1. Add a sample file to the test repository
2. Create a test function that uses the sample file
3. Define TreeSitter queries to test against the sample file
4. Run the queries and verify the results

Example:

```rust
#[tokio::test]
async fn test_new_language() {
    let scanner = create_test_scanner();
    let file_path = create_test_file("test_file.ext", SAMPLE_CODE);

    let queries = vec![
        TreeSitterQuery {
            question_id: "test_id".to_string(),
            file_type: ".ext".to_string(),
            query: "(your_query_here)".to_string(),
            object_id: "test_object".to_string(),
            prompt: "Test prompt".to_string(),
            reasoning: "Test reasoning".to_string(),
        },
    ];

    let results = scanner.scan_files(vec![file_path], queries).await;

    // Verify results
    assert!(!results.is_empty());
    // Add more assertions as needed
}
```

To add a new performance test:

```rust
#[tokio::test]
async fn test_new_performance_aspect() {
    let scanner = create_test_scanner();

    // Setup test data
    let file_path = create_large_file(10); // 10x size

    // Measure execution time
    let (results, duration) = measure_async_execution_time(||
        scanner.some_operation(file_path)
    ).await;

    // Print performance results
    println!("Operation took: {:?}", duration);

    // Optional assertions if appropriate
    assert!(results.is_some());
}
```
