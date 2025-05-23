mod test_utils;

use test_utils::{
    create_test_scanner, ensure_test_repo, test_repo_path,
    RUST_SAMPLE, JS_SAMPLE, PYTHON_SAMPLE, GO_SAMPLE, TS_SAMPLE,
    JAVA_SAMPLE, CPP_SAMPLE, RUBY_SAMPLE, PHP_SAMPLE
};
use magma_scanner::types::{TreeSitterQuery, MatchResult};
use std::fs;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test file with the given content
    fn create_test_file(filename: &str, content: &str) -> String {
        ensure_test_repo();
        let file_path = test_repo_path().join(filename);
        fs::write(&file_path, content).expect("Failed to write test file");
        file_path.to_string_lossy().to_string()
    }

    // Helper function to display evidence collected for a query
    fn display_evidence(results: &[MatchResult], query_id: &str) {
        let query_results = results.iter().filter(|r| r.question_id == query_id).collect::<Vec<_>>();

        println!("\n=== Evidence for query '{}' ===", query_id);
        println!("Found {} matches:", query_results.len());

        for (i, result) in query_results.iter().enumerate() {
            println!("  {}. \"{}\" at {}:{}:{}",
                i + 1,
                result.text,
                result.file.split('\\').last().unwrap_or(&result.file),
                result.line,
                result.column
            );
        }
        println!("==============================\n");
    }

    // Helper function to group results by query ID
    fn group_results_by_query(results: &[MatchResult]) -> HashMap<String, Vec<&MatchResult>> {
        let mut grouped = HashMap::new();
        for result in results {
            grouped.entry(result.question_id.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }
        grouped
    }

    #[tokio::test]
    async fn test_rust_queries() {
        let scanner = create_test_scanner();
        let file_path = create_test_file("test_rust.rs", RUST_SAMPLE);

        // Test queries for Rust
        let queries = vec![
            // Find struct definitions
            TreeSitterQuery {
                question_id: "rust_struct".to_string(),
                file_type: ".rs".to_string(),
                query: "(struct_item name: (type_identifier) @struct_name)".to_string(),
                object_id: "test_object".to_string(),
                prompt: "Find struct definitions".to_string(),
                reasoning: "Testing struct detection".to_string(),
            },
            // Find function definitions
            TreeSitterQuery {
                question_id: "rust_function".to_string(),
                file_type: ".rs".to_string(),
                query: "(function_item name: (identifier) @function_name)".to_string(),
                object_id: "test_object".to_string(),
                prompt: "Find function definitions".to_string(),
                reasoning: "Testing function detection".to_string(),
            },
            // Find string literals
            TreeSitterQuery {
                question_id: "rust_strings".to_string(),
                file_type: ".rs".to_string(),
                query: "(string_literal) @string".to_string(),
                object_id: "test_object".to_string(),
                prompt: "Find string literals".to_string(),
                reasoning: "Testing string detection".to_string(),
            },
        ];

        // Run the scan
        let results = scanner.scan_files(vec![file_path], queries).await;

        // Verify results
        assert!(!results.is_empty());

        // Display evidence for each query
        println!("\n--- Rust Query Test Results ---");
        display_evidence(&results, "rust_struct");
        display_evidence(&results, "rust_function");
        display_evidence(&results, "rust_strings");

        // Check for struct detection
        let struct_results = results.iter().filter(|r| r.question_id == "rust_struct").collect::<Vec<_>>();
        assert!(!struct_results.is_empty());
        assert!(struct_results.iter().any(|r| r.text == "User"));

        // Verify struct evidence details
        let user_struct = struct_results.iter().find(|r| r.text == "User").unwrap();
        println!("User struct found at line {}, column {}", user_struct.line, user_struct.column);

        // Check for function detection
        let function_results = results.iter().filter(|r| r.question_id == "rust_function").collect::<Vec<_>>();
        assert!(!function_results.is_empty());
        assert!(function_results.iter().any(|r| r.text == "main"));
        assert!(function_results.iter().any(|r| r.text == "display"));

        // Verify function evidence details
        let main_fn = function_results.iter().find(|r| r.text == "main").unwrap();
        let display_fn = function_results.iter().find(|r| r.text == "display").unwrap();
        println!("main function found at line {}, column {}", main_fn.line, main_fn.column);
        println!("display function found at line {}, column {}", display_fn.line, display_fn.column);

        // Check for string detection
        let string_results = results.iter().filter(|r| r.question_id == "rust_strings").collect::<Vec<_>>();
        assert!(!string_results.is_empty());

        // Print summary of string literals found
        println!("Found {} string literals", string_results.len());
        println!("String literals: {}", string_results.iter()
            .map(|r| format!("\"{}\"", r.text))
            .collect::<Vec<_>>()
            .join(", "));
    }

    #[tokio::test]
    async fn test_javascript_queries() {
        let scanner = create_test_scanner();
        let file_path = create_test_file("test_js.js", JS_SAMPLE);

        // Test queries for JavaScript
        let queries = vec![
            // Find class definitions
            TreeSitterQuery {
                question_id: "js_class".to_string(),
                file_type: ".js".to_string(),
                query: "(class_declaration name: (identifier) @class_name)".to_string(),
                object_id: "test_object".to_string(),
                prompt: "Find class definitions".to_string(),
                reasoning: "Testing class detection".to_string(),
            },
            // Find method definitions
            TreeSitterQuery {
                question_id: "js_method".to_string(),
                file_type: ".js".to_string(),
                query: "(method_definition name: (property_identifier) @method_name)".to_string(),
                object_id: "test_object".to_string(),
                prompt: "Find method definitions".to_string(),
                reasoning: "Testing method detection".to_string(),
            },
        ];

        // Run the scan
        let results = scanner.scan_files(vec![file_path], queries).await;

        // Verify results
        assert!(!results.is_empty());

        // Display evidence for each query
        println!("\n--- JavaScript Query Test Results ---");
        display_evidence(&results, "js_class");
        display_evidence(&results, "js_method");

        // Check for class detection
        let class_results = results.iter().filter(|r| r.question_id == "js_class").collect::<Vec<_>>();
        assert!(!class_results.is_empty());
        assert!(class_results.iter().any(|r| r.text == "User"));

        // Verify class evidence details
        let user_class = class_results.iter().find(|r| r.text == "User").unwrap();
        println!("User class found at line {}, column {}", user_class.line, user_class.column);

        // Check for method detection
        let method_results = results.iter().filter(|r| r.question_id == "js_method").collect::<Vec<_>>();
        assert!(!method_results.is_empty());
        assert!(method_results.iter().any(|r| r.text == "display"));

        // Verify method evidence details
        let display_method = method_results.iter().find(|r| r.text == "display").unwrap();
        println!("display method found at line {}, column {}", display_method.line, display_method.column);

        // Print all methods found
        println!("All methods found: {}", method_results.iter()
            .map(|r| r.text.clone())
            .collect::<Vec<_>>()
            .join(", "));
    }

    #[tokio::test]
    async fn test_python_queries() {
        let scanner = create_test_scanner();
        let file_path = create_test_file("test_python.py", PYTHON_SAMPLE);

        // Test queries for Python
        let queries = vec![
            // Find class definitions
            TreeSitterQuery {
                question_id: "py_class".to_string(),
                file_type: ".py".to_string(),
                query: "(class_definition name: (identifier) @class_name)".to_string(),
                object_id: "test_object".to_string(),
                prompt: "Find class definitions".to_string(),
                reasoning: "Testing class detection".to_string(),
            },
            // Find function definitions
            TreeSitterQuery {
                question_id: "py_function".to_string(),
                file_type: ".py".to_string(),
                query: "(function_definition name: (identifier) @function_name)".to_string(),
                object_id: "test_object".to_string(),
                prompt: "Find function definitions".to_string(),
                reasoning: "Testing function detection".to_string(),
            },
        ];

        // Run the scan
        let results = scanner.scan_files(vec![file_path], queries).await;

        // Verify results
        assert!(!results.is_empty());

        // Display evidence for each query
        println!("\n--- Python Query Test Results ---");
        display_evidence(&results, "py_class");
        display_evidence(&results, "py_function");

        // Check for class detection
        let class_results = results.iter().filter(|r| r.question_id == "py_class").collect::<Vec<_>>();
        assert!(!class_results.is_empty());
        assert!(class_results.iter().any(|r| r.text == "User"));

        // Verify class evidence details
        let user_class = class_results.iter().find(|r| r.text == "User").unwrap();
        println!("User class found at line {}, column {}", user_class.line, user_class.column);

        // Check for function detection
        let function_results = results.iter().filter(|r| r.question_id == "py_function").collect::<Vec<_>>();
        assert!(!function_results.is_empty());
        assert!(function_results.iter().any(|r| r.text == "display"));

        // Verify function evidence details
        let display_fn = function_results.iter().find(|r| r.text == "display").unwrap();
        println!("display function found at line {}, column {}", display_fn.line, display_fn.column);

        // Print all functions found
        println!("All Python functions found: {}", function_results.iter()
            .map(|r| r.text.clone())
            .collect::<Vec<_>>()
            .join(", "));
    }

    // Add more language-specific tests as needed

    #[tokio::test]
    async fn test_complex_query() {
        let scanner = create_test_scanner();
        let file_path = create_test_file("test_complex.rs", r#"
            fn insecure_function() {
                let password = "hardcoded_password";
                let api_key = "1234567890abcdef";
                let token = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
            }

            fn secure_function() {
                let name = "John";
                let greeting = "Hello";
            }
        "#);

        // Complex query to find potential hardcoded secrets
        let query = TreeSitterQuery {
            question_id: "security_check".to_string(),
            file_type: ".rs".to_string(),
            query: r#"
                (assignment_expression
                  left: (identifier) @var_name
                  right: (string_literal) @string_value
                  (#match? @var_name "password|secret|key|token|credential")
                )

                (let_declaration
                  pattern: (identifier) @var_name
                  value: (string_literal) @string_value
                  (#match? @var_name "password|secret|key|token|credential")
                )
            "#.to_string(),
            object_id: "test_object".to_string(),
            prompt: "Find hardcoded secrets".to_string(),
            reasoning: "Testing complex query with predicates".to_string(),
        };

        // Run the scan
        let results = scanner.scan_files(vec![file_path], vec![query]).await;

        // Verify results
        assert!(!results.is_empty());

        // Display evidence for the complex query
        println!("\n--- Complex Query Test Results ---");
        display_evidence(&results, "security_check");

        // Group results by line number to show context
        let mut results_by_line: HashMap<usize, Vec<&MatchResult>> = HashMap::new();
        for result in &results {
            results_by_line.entry(result.line).or_insert_with(Vec::new).push(result);
        }

        // Print results with context
        println!("\nPotential security issues by line:");
        for (line, matches) in results_by_line.iter() {
            println!("Line {}: Found {} potential issues", line, matches.len());
            for m in matches {
                println!("  - Variable: '{}' (potential hardcoded secret)", m.text);
            }
        }

        // Should find the password, api_key, and token variables
        let var_names = results.iter().map(|r| r.text.clone()).collect::<Vec<_>>();
        assert!(var_names.contains(&"password".to_string()));
        assert!(var_names.contains(&"api_key".to_string()));
        assert!(var_names.contains(&"token".to_string()));

        // Print all found variables
        println!("\nAll sensitive variables found: {}", var_names.join(", "));

        // Should not find name or greeting
        assert!(!var_names.contains(&"name".to_string()));
        assert!(!var_names.contains(&"greeting".to_string()));

        // Print excluded variables
        println!("Correctly excluded non-sensitive variables: name, greeting");
    }
}
