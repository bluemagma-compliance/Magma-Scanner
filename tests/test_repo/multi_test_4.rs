
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
