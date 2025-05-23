# Magma Scanner

![Magma Scanner](https://via.placeholder.com/150x150.png?text=Magma+Scanner)

A powerful code analysis tool that uses TreeSitter queries to scan codebases for patterns, vulnerabilities, and insights.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Magma Scanner is a high-performance code analysis tool built in Rust that leverages TreeSitter to parse and analyze code across multiple programming languages. It uses a query-based approach to identify patterns, potential issues, and gather insights from your codebase.

### Key Features

- **Multi-language Support**: Analyzes code in Rust, JavaScript, Python, Go, TypeScript, Java, C++, Ruby, and PHP
- **TreeSitter Powered**: Uses TreeSitter's precise parsing capabilities for accurate code analysis
- **AST Caching**: Optimizes performance by caching Abstract Syntax Trees (ASTs)
- **Continuous Scanning**: Polls for new queries and continuously scans your codebase
- **API Integration**: Connects to a central service for query management and result reporting
- **Git Integration**: Automatically detects repository information

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)
- Git (for repository information)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/magma-scanner.git
cd magma-scanner

# Build the project
cargo build --release

# The binary will be available at target/release/magma-scanner
```

## Usage

### Basic Usage

```bash
magma-scanner -a YOUR_API_KEY -o YOUR_ORGANIZATION_ID -t ./path/to/repo
```

### Command Line Options

```
USAGE:
    magma-scanner [OPTIONS] --api-key <API_KEY> --organization-id <ORGANIZATION_ID>

OPTIONS:
    -a, --api-key <API_KEY>                  API key for authentication
    -o, --organization-id <ORGANIZATION_ID>  Organization ID
    -t, --target <TARGET>                    Target directory to scan [default: .]
    -r, --report-id <REPORT_ID>              Report ID (optional)
    -p, --poll-interval <POLL_INTERVAL>      Polling interval in seconds [default: 5]
    -m, --max-polls <MAX_POLLS>              Maximum number of polling iterations [default: 20]
    -h, --help                               Print help information
    -V, --version                            Print version information
```

### Environment Variables

You can also configure Magma Scanner using environment variables by creating a `.env` file:

```
API_BASE_URL=http://localhost:8080/api/v1
API_KEY=your_api_key_here
ORGANIZATION_ID=your_organization_id_here
REPORT_ID=existing_report_id_if_continuing_a_scan
POLL_INTERVAL=5
MAX_POLLS=20
```

## How It Works

1. **Initialization**: Magma Scanner connects to the API service and initializes a code scan report
2. **File Discovery**: Scans the target directory for supported file types
3. **Query Fetching**: Retrieves TreeSitter queries from the API
4. **AST Parsing**: Parses each file into an Abstract Syntax Tree (AST)
5. **Query Execution**: Runs the queries against the ASTs to find matches
6. **Result Reporting**: Reports matches back to the API
7. **Continuous Scanning**: Polls for new queries and repeats the process

### TreeSitter Queries

Magma Scanner uses TreeSitter queries to analyze code. These queries are written in the TreeSitter query language and can identify specific patterns in the code.

Example query to find all function definitions in Rust:

```
(function_item
  name: (identifier) @function_name)
```

## Supported Languages

| Language   | Extensions                |
|------------|---------------------------|
| Rust       | .rs                       |
| JavaScript | .js                       |
| Python     | .py                       |
| Go         | .go                       |
| TypeScript | .ts                       |
| Java       | .java                     |
| C++        | .cpp, .h, .hpp, .cc       |
| Ruby       | .rb                       |
| PHP        | .php                      |

## Development

### Project Structure

```
magma-scanner/
├── src/
│   ├── bin/
│   │   └── magma-scanner.rs    # CLI entry point
│   ├── lib.rs                  # Library exports
│   ├── scanner.rs              # Core scanner implementation
│   ├── language_loader.rs      # Language support
│   └── types.rs                # Data structures
├── tests/
│   ├── scanner_tests.rs        # Scanner tests
│   ├── query_tests.rs          # Query tests
│   ├── performance_tests.rs    # Performance tests
│   ├── integration_tests.rs    # API tests
│   └── test_utils/             # Test utilities
└── Cargo.toml                  # Project configuration
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_rust_queries

# Run tests in a specific file
cargo test --test performance_tests

# Run a specific test in a specific file
cargo test --test performance_tests test_ast_caching_performance
```

### Performance Testing

The project includes performance tests to measure:

- File parsing performance
- Query execution performance
- AST caching effectiveness
- Multi-file scanning performance

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [TreeSitter](https://tree-sitter.github.io/tree-sitter/) for the powerful parsing capabilities
- All the language grammar contributors