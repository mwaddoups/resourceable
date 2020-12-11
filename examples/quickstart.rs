use async_trait::async_trait;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tide::log;
use std::sync::{Arc, Mutex};

use resourceable::{Resourceable, add_resource};

/*
   We'll use the Spaceship as an example resource.
   It should derive Serialize and Deserialize since the API emits JSON by default.
   For this example we derive Clone to allow us to fake the database more easily.
*/
#[derive(Serialize, Deserialize, Clone)]
struct Spaceship {
    id: Option<i32>,
    num_thrusters: u8,
    name: String,
}

/* 
   We'll make a simple thread-safe database (wrapping a Vec<Spaceship>)
   In the real world this would often be a database connection or pool
*/
type Database = Arc<Mutex<Vec<Spaceship>>>;

// We can now create a Resourceable trait instance for the Spaceship
#[async_trait]
impl Resourceable<Database, i32> for Spaceship {
    async fn read_by_id(db: &Database, id: i32) -> anyhow::Result<Spaceship> {
        let data = & *db.lock().unwrap();
        for spaceship in data {
            if spaceship.id == Some(id) { return Ok(spaceship.clone()); }
        }

        Err(anyhow!("No spaceship found!"))
    }

    async fn read_paged(db: &Database, _size: u32, _offset: u32) -> anyhow::Result<Vec<Spaceship>> {
        // This should return a page - but for this example, we'll ignore that
        let data = & *db.lock().unwrap();

        Ok(data.clone())
    }

    async fn create(db: &Database, new_spaceship: Spaceship) -> anyhow::Result<Spaceship> {
        let data = &mut *db.lock().unwrap();

        let max_id = data.iter().map(|x| x.id).max().unwrap().unwrap();
        let mut inserted_spaceship = new_spaceship.clone();
        inserted_spaceship.id = Some(max_id + 1);

        data.push(inserted_spaceship.clone());

        Ok(inserted_spaceship)
    }

    async fn update(db: &Database, id: i32, new_spaceship: Spaceship) -> anyhow::Result<Spaceship> {
        let data = &mut *db.lock().unwrap();

        let mut index_to_update: Option<usize> = None;

        for i in 0..data.len() {
            if data[i].id == Some(id) { 
                index_to_update = Some(i);
                break;
            }
        }

        match index_to_update {
            Some(index) => {
                data[index] = new_spaceship.clone();
                Ok(new_spaceship)
            },
            None => Err(anyhow!("No matching ID found to update.")),
        }
    }

    async fn delete(db: &Database, id: i32) -> anyhow::Result<Spaceship> {
        let data = &mut *db.lock().unwrap();

        let mut index_to_remove: Option<usize> = None;
        let mut removed_ship: Option<Spaceship> = None;
        for i in 0..data.len() {
            if data[i].id == Some(id) { 
                removed_ship = Some(data[i].clone());
                index_to_remove = Some(i);
                break;
            }
        }

        data.remove(index_to_remove.unwrap());
        Ok(removed_ship.unwrap())
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Setup the database
    let db: Database = Arc::new(Mutex::new(vec![
        Spaceship { id: Some(1), num_thrusters: 4, name: "SS Small".to_string() },
        Spaceship { id: Some(2), num_thrusters: 35, name: "Thrusty McThrust".to_string() },
    ]));

    // Pass the DB as the state
    let mut app = tide::with_state(db);

    // Enable logging
    log::start();
    app.with(tide::log::LogMiddleware::new());

    // Add the resource at the /spaceship endpoint
    add_resource!(app, "/spaceship", Spaceship);

    // Start the server
    app.listen("127.0.0.1:8082").await?;
    Ok(())
}
