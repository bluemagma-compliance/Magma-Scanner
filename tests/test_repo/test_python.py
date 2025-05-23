
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
