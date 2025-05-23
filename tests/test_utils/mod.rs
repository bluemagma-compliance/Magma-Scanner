use magma_scanner::scanner::Scanner;
use magma_scanner::types::TreeSitterQuery;
use std::fs;
use std::path::{Path, PathBuf};

/// Create a test scanner instance that doesn't require API connectivity
pub fn create_test_scanner() -> Scanner {
    Scanner::new(
        "test_api_key".to_string(),
        "test_org_id".to_string(),
        "test_commit_hash".to_string(),
        Some("test_report_id".to_string()),
    )
}

/// Create a TreeSitterQuery for testing
pub fn create_test_query(language: &str, query_text: &str) -> TreeSitterQuery {
    TreeSitterQuery {
        question_id: "test_question_id".to_string(),
        file_type: format!(".{}", language),
        query: query_text.to_string(),
        object_id: "test_object_id".to_string(),
        prompt: "Test prompt".to_string(),
        reasoning: "Test reasoning".to_string(),
    }
}

/// Get the path to the test repository
pub fn test_repo_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("test_repo");
    path
}

/// Create the test repository if it doesn't exist
pub fn ensure_test_repo() {
    let repo_path = test_repo_path();
    if !repo_path.exists() {
        fs::create_dir_all(&repo_path).expect("Failed to create test repo directory");
        
        // Create sample files for each supported language
        create_sample_file(&repo_path, "sample.rs", RUST_SAMPLE);
        create_sample_file(&repo_path, "sample.js", JS_SAMPLE);
        create_sample_file(&repo_path, "sample.py", PYTHON_SAMPLE);
        create_sample_file(&repo_path, "sample.go", GO_SAMPLE);
        create_sample_file(&repo_path, "sample.ts", TS_SAMPLE);
        create_sample_file(&repo_path, "sample.java", JAVA_SAMPLE);
        create_sample_file(&repo_path, "sample.cpp", CPP_SAMPLE);
        create_sample_file(&repo_path, "sample.rb", RUBY_SAMPLE);
        create_sample_file(&repo_path, "sample.php", PHP_SAMPLE);
    }
}

fn create_sample_file(repo_path: &Path, filename: &str, content: &str) {
    let file_path = repo_path.join(filename);
    fs::write(file_path, content).expect(&format!("Failed to create {}", filename));
}

// Sample code for each supported language
pub const RUST_SAMPLE: &str = r#"
// Sample Rust file for testing TreeSitter queries
use std::collections::HashMap;

struct User {
    id: u64,
    name: String,
    email: String,
}

impl User {
    fn new(id: u64, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    fn display(&self) -> String {
        format!("User {} ({}): {}", self.id, self.name, self.email)
    }
}

fn main() {
    let mut users = HashMap::new();
    
    // Create some users
    let user1 = User::new(1, "Alice", "alice@example.com");
    let user2 = User::new(2, "Bob", "bob@example.com");
    
    // Add users to the map
    users.insert(user1.id, user1);
    users.insert(user2.id, user2);
    
    // Print users
    for (_, user) in &users {
        println!("{}", user.display());
    }
}
"#;

pub const JS_SAMPLE: &str = r#"
// Sample JavaScript file for testing TreeSitter queries
class User {
  constructor(id, name, email) {
    this.id = id;
    this.name = name;
    this.email = email;
  }
  
  display() {
    return `User ${this.id} (${this.name}): ${this.email}`;
  }
}

// Create a map of users
const users = new Map();

// Add some users
const user1 = new User(1, "Alice", "alice@example.com");
const user2 = new User(2, "Bob", "bob@example.com");

users.set(user1.id, user1);
users.set(user2.id, user2);

// Print users
for (const [_, user] of users) {
  console.log(user.display());
}
"#;

pub const PYTHON_SAMPLE: &str = r#"
# Sample Python file for testing TreeSitter queries
class User:
    def __init__(self, id, name, email):
        self.id = id
        self.name = name
        self.email = email
    
    def display(self):
        return f"User {self.id} ({self.name}): {self.email}"

# Create a dictionary of users
users = {}

# Add some users
user1 = User(1, "Alice", "alice@example.com")
user2 = User(2, "Bob", "bob@example.com")

users[user1.id] = user1
users[user2.id] = user2

# Print users
for user_id, user in users.items():
    print(user.display())
"#;

// Add more language samples as needed
pub const GO_SAMPLE: &str = r#"
// Sample Go file for testing TreeSitter queries
package main

import "fmt"

type User struct {
    ID    int
    Name  string
    Email string
}

func (u User) Display() string {
    return fmt.Sprintf("User %d (%s): %s", u.ID, u.Name, u.Email)
}

func main() {
    users := make(map[int]User)
    
    // Create some users
    user1 := User{ID: 1, Name: "Alice", Email: "alice@example.com"}
    user2 := User{ID: 2, Name: "Bob", Email: "bob@example.com"}
    
    // Add users to the map
    users[user1.ID] = user1
    users[user2.ID] = user2
    
    // Print users
    for _, user := range users {
        fmt.Println(user.Display())
    }
}
"#;

// Add more language samples as constants
pub const TS_SAMPLE: &str = r#"
// Sample TypeScript file for testing TreeSitter queries
interface User {
  id: number;
  name: string;
  email: string;
}

class UserImpl implements User {
  id: number;
  name: string;
  email: string;
  
  constructor(id: number, name: string, email: string) {
    this.id = id;
    this.name = name;
    this.email = email;
  }
  
  display(): string {
    return `User ${this.id} (${this.name}): ${this.email}`;
  }
}

// Create a map of users
const users = new Map<number, User>();

// Add some users
const user1 = new UserImpl(1, "Alice", "alice@example.com");
const user2 = new UserImpl(2, "Bob", "bob@example.com");

users.set(user1.id, user1);
users.set(user2.id, user2);

// Print users
for (const [_, user] of users) {
  console.log((user as UserImpl).display());
}
"#;

// Add more language samples as needed
pub const JAVA_SAMPLE: &str = r#"
// Sample Java file for testing TreeSitter queries
import java.util.HashMap;
import java.util.Map;

class User {
    private int id;
    private String name;
    private String email;
    
    public User(int id, String name, String email) {
        this.id = id;
        this.name = name;
        this.email = email;
    }
    
    public int getId() {
        return id;
    }
    
    public String display() {
        return "User " + id + " (" + name + "): " + email;
    }
}

public class Main {
    public static void main(String[] args) {
        Map<Integer, User> users = new HashMap<>();
        
        // Create some users
        User user1 = new User(1, "Alice", "alice@example.com");
        User user2 = new User(2, "Bob", "bob@example.com");
        
        // Add users to the map
        users.put(user1.getId(), user1);
        users.put(user2.getId(), user2);
        
        // Print users
        for (User user : users.values()) {
            System.out.println(user.display());
        }
    }
}
"#;

pub const CPP_SAMPLE: &str = r#"
// Sample C++ file for testing TreeSitter queries
#include <iostream>
#include <string>
#include <unordered_map>

class User {
private:
    int id;
    std::string name;
    std::string email;
    
public:
    User(int id, const std::string& name, const std::string& email)
        : id(id), name(name), email(email) {}
    
    int getId() const { return id; }
    
    std::string display() const {
        return "User " + std::to_string(id) + " (" + name + "): " + email;
    }
};

int main() {
    std::unordered_map<int, User> users;
    
    // Create some users
    User user1(1, "Alice", "alice@example.com");
    User user2(2, "Bob", "bob@example.com");
    
    // Add users to the map
    users[user1.getId()] = user1;
    users[user2.getId()] = user2;
    
    // Print users
    for (const auto& pair : users) {
        std::cout << pair.second.display() << std::endl;
    }
    
    return 0;
}
"#;

pub const RUBY_SAMPLE: &str = r#"
# Sample Ruby file for testing TreeSitter queries
class User
  attr_reader :id
  
  def initialize(id, name, email)
    @id = id
    @name = name
    @email = email
  end
  
  def display
    "User #{@id} (#{@name}): #{@email}"
  end
end

# Create a hash of users
users = {}

# Add some users
user1 = User.new(1, "Alice", "alice@example.com")
user2 = User.new(2, "Bob", "bob@example.com")

users[user1.id] = user1
users[user2.id] = user2

# Print users
users.each_value do |user|
  puts user.display
end
"#;

pub const PHP_SAMPLE: &str = r#"
<?php
// Sample PHP file for testing TreeSitter queries
class User {
    private $id;
    private $name;
    private $email;
    
    public function __construct($id, $name, $email) {
        $this->id = $id;
        $this->name = $name;
        $this->email = $email;
    }
    
    public function getId() {
        return $this->id;
    }
    
    public function display() {
        return "User " . $this->id . " (" . $this->name . "): " . $this->email;
    }
}

// Create an array of users
$users = [];

// Add some users
$user1 = new User(1, "Alice", "alice@example.com");
$user2 = new User(2, "Bob", "bob@example.com");

$users[$user1->getId()] = $user1;
$users[$user2->getId()] = $user2;

// Print users
foreach ($users as $user) {
    echo $user->display() . "\n";
}
?>"#;
