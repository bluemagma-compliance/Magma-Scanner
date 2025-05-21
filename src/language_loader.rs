use tree_sitter::Language;

pub fn get_language(language_name: &str) -> Option<Language> {
    match language_name {
        "rust" | "rs" => Some(tree_sitter_rust::language()),
        "javascript" | "js" => Some(tree_sitter_javascript::language()),
        "python" | "py" => Some(tree_sitter_python::language()),
        "go" => Some(tree_sitter_go::language()),
        "typescript" | "ts" => Some(tree_sitter_typescript::language_typescript()),
        "java" => Some(tree_sitter_java::language()),
        "cpp" | "c++" | "h" | "hpp" | "cc" => Some(tree_sitter_cpp::language()),
        "ruby" | "rb" => Some(tree_sitter_ruby::language()),
        "php" => Some(tree_sitter_php::language()),
        _ => None,
    }
}
