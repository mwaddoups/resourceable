# Resourceable

A simple proof-of-concept crate for quickly writing REST APIs for resources that can be described as structs. This is built on top of [http-rs/tide](https://github.com/http-rs/tide).

# Motivation

Consider the example Spaceship. We want to write the minimal code to provide the API

- `GET /spaceship?size=x&offset=y` - Provide a page of spaceships starting at y of size x.
- `GET /spaceship/:id` - Provide a single spaceship
- `POST /spaceship` - Create a new spaceship
- `PUT /spaceship/:id` - Replace an existing spaceship at :id
- `DELETE /spaceship/:id` - Delete the spaceship

Here is how this looks here

```rust
use resourceable::{Resourceable, add_resource};

#[derive(Serialize, Deserialize)]
struct Spaceship {
    id: Option<i32>, // Optional to represent both inserted and non-inserted records
    num_thrusters: u8,
    name: String,
}

type Database = // your choice, e.g. PgPool from sqlx

// Type parameters are <State, IdentityType>, matching the state in tide
impl Resourceable<Database, i32> for Spaceship {
    async fn read_by_id(db: &Database, id: i32) -> anyhow::Result<Spaceship> {
        // Return a Spaceship with the given id from the database
    }

    async fn read_paged(db: &Database, size: u32, offset: u32) -> anyhow::Result<Spaceship> {
        // Return a page of spaceships of size with offset.
    }

    async fn create(db: &Database, new_spaceship: Spaceship) -> anyhow::Result<Spaceship> {
        // Add the new spaceship, and return the new resource.
    }

    async fn update(db: &Database, id: i32, new_spaceship: Spaceship) -> anyhow::Result<Spaceship> {
        // Replace the spaceship at id with a new spaceship
    }

    async fn remove(db: &Database, id: i32, new_spaceship: Spaceship) -> anyhow::Result<Spaceship> {
        // Remove the spaceship at id
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Setup the database
    let db = // ....

    // Pass the DB as the state
    let mut app = tide::with_state(db);

    // Add the resource at the /spaceship endpoint
    add_resource!(app, "/spaceship", Spaceship);

    // Start the server
    app.listen("127.0.0.1:8082").await?;
    Ok(())
}
```

# Examples

- [examples/quickstart.rs](examples/quickstart.rs) - A quick example with in-memory database.
- [examples/sqlx-postgres](examples/sqlx-postgres) - A full example showing an implementation with SQLx.

# TODO

- Write some tests.
