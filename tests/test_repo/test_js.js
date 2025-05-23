
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
