extern crate time;
extern crate sqlite3;

use time::Timespec;
use rusqlite::Connection;

use sqlite3::{
    DatabaseConnection,
    Query,
    ResultRow,
    ResultRowAccess,
    SqliteResult,
    StatementUpdate,
};

/*
fn main() {
    println!("Hello, world!");
}
*/

#[derive(Debug)]
struct Subscription {
    id: i32,
    title: String,
    last_updated: Timespec,
    data: Option<Vec<u8>>
}

pub fn main() {
    let conn = Connection::open_in_memory().unwrap();

    let me = Person {
        id: 0,
        title: "Steven".to_string(),
        last_updated: time::get_time(),
        data: None
    };

    /*
conn.execute("INSERT INTO person (name, time_created, data)
              VALUES ($1, $2, $3)",
             &[&me.name, &me.time_created, &me.data]).unwrap();
*/
    let mut stmt = conn.prepare("SELECT _id, title, last_updated, image FROM Subscription").unwrap();
    let mut person_iter = stmt.query_map(&[], |row| {
        Subscription {
            id: row.get(0),
            name: row.get(1),
            last_updated: row.get(2),
            data: row.get(3)
        }
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}
