mod test_utils;

use test_utils::{create_test_scanner, ensure_test_repo, test_repo_path, RUST_SAMPLE};
use magma_scanner::types::TreeSitterQuery;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a temporary file with content
    fn create_temp_file(filename: &str, content: &str) -> String {
        ensure_test_repo();
        let file_path = test_repo_path().join(filename);
        fs::write(&file_path, content).expect("Failed to write test file");
        file_path.to_string_lossy().to_string()
    }

    #[test]
    fn test_get_language_for_file() {
        let scanner = create_test_scanner();

        // Test with various file extensions
        assert_eq!(scanner.get_language_for_file("test.rs"), Some("rust"));
        assert_eq!(scanner.get_language_for_file("test.js"), Some("javascript"));
        assert_eq!(scanner.get_language_for_file("test.py"), Some("python"));
        assert_eq!(scanner.get_language_for_file("test.go"), Some("go"));
        assert_eq!(scanner.get_language_for_file("test.ts"), Some("typescript"));
        assert_eq!(scanner.get_language_for_file("test.java"), Some("java"));
        assert_eq!(scanner.get_language_for_file("test.cpp"), Some("cpp"));
        assert_eq!(scanner.get_language_for_file("test.h"), Some("cpp"));
        assert_eq!(scanner.get_language_for_file("test.rb"), Some("ruby"));
        assert_eq!(scanner.get_language_for_file("test.php"), Some("php"));

        // Test with unsupported extension
        assert_eq!(scanner.get_language_for_file("test_repo/test_python.py"), Some("python"));
        assert_eq!(scanner.get_language_for_file("test_repo/test_js.js"), Some("javascript"));
        assert_eq!(scanner.get_language_for_file("test_repo/test_rust.rs"), Some("rust"));
        assert_eq!(scanner.get_language_for_file("test.xyz"), None);
    }

    #[test]
    fn test_parse_file() {
        let scanner = create_test_scanner();
        let file_path = create_temp_file("test_parse.rs", RUST_SAMPLE);

        // Test parsing a Rust file
        let result = scanner.parse_file(&file_path, "rust");
        assert!(result.is_some());

        let (tree, source) = result.unwrap();
        assert_eq!(source, RUST_SAMPLE);
        assert!(tree.root_node().child_count() > 0);
    }

    #[tokio::test]
    async fn test_scan_files_with_direct_query_injection() {
        let scanner = create_test_scanner();
        ensure_test_repo();

        // Create a test file
        let file_path = create_temp_file("test_scan.rs", r#"
            struct User {
                id: u64,
                name: String,
                email: String,
            }

            fn main() {
                let user = User {
                    id: 1,
                    name: "Alice".to_string(),
                    email: "alice@example.com".to_string(),
                };
            }
        "#);

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

        // Verify results
        assert!(!results.is_empty());
        assert_eq!(results[0].text, "User");
    }

    #[test]
    fn test_run_query_on_tree_direct_injection() {
        let scanner = create_test_scanner();
        let file_path = create_temp_file("test_query.rs", r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }

            fn subtract(a: i32, b: i32) -> i32 {
                a - b
            }
        "#);

        // Parse the file
        let (tree, source) = scanner.parse_file(&file_path, "rust").unwrap();

        // Create a query to find function definitions
        let query_text = "(function_item name: (identifier) @function_name)";

        // Run the query directly
        let captures = scanner.run_query_on_tree(&tree, &source, query_text, "rust");

        // Verify results
        assert_eq!(captures.len(), 2);
        assert!(captures.iter().any(|c| c.value == "add"));
        assert!(captures.iter().any(|c| c.value == "subtract"));
    }
}
