extern crate rusqlite;
extern crate time;

use std::path::Path;
use rusqlite::Connection;

mod types;
mod library;

pub fn main() {
    //let conn = Connection::open_in_memory().unwrap();

    let path = Path::new("/home/bottiger/Rust/SoundWaves-Core/test_data/podcast.db");
    let conno = Connection::open(path);

    match conno {
            Ok(ref v) => println!("Database opened: {:?}", v),
            Err(ref e) => println!("Database could not be opened: {:?}", e),
    }

    let conn = conno.unwrap();


    /*
conn.execute("INSERT INTO person (name, time_created, data)
              VALUES ($1, $2, $3)",
             &[&me.name, &me.time_created, &me.data]).unwrap();
*/
    let mut stmt = conn.prepare("SELECT _id, title FROM subscriptions").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        types::Subscription {
            id: row.get(0),
            title: row.get(1),
            episodes: Vec::new()
 //         last_updated: row.get(2),
 //         data: row.get(3)
        }
    }).unwrap();

    for person in person_iter {
        let sub = person.unwrap();
        let count = sub.episode_count();
        println!("Found person {:?}, episodes: {:?}", sub, count);
    }
}
