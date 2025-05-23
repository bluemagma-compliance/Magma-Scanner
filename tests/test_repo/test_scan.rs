
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
        