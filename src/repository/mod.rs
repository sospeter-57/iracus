use rusqlite::{Connection, params};

use crate::models::compartment::Label;
use crate::{utils::get_default_db_path};
use crate::models::user::User;

use std::{path::Path};

pub struct Repository {
    connection: Connection
}

impl Repository {
    pub fn new() -> Repository {
        let path: &Path = get_default_db_path();
        let repository = Repository { connection: Connection::open(path).unwrap() };
        repository.connection.execute(
            "CREATE TABLE IF NOT EXISTS secrets(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            label TEXT NOT NULL,
            secret TEXT NOT NULL)",
            (),
        ).unwrap();

        // // at this point theres a problem
        // // to migrate will give us a key value pair of all labels and their secrets
        // // but the current implementation assumes each user has got
        // // one compartment which stores one secret and it's label
        // // a different possibility of an impolementation might be different users logged in,
        // // so migration allocates each user each key value pair (they added in the past)
        // // so to evade all that, we're going to query the database just the first entry to
        // // fill our one user who is logged in
        // let row: (String, String) = repository.connection.query_row(
        //     "SELECT label, secret FROM secrets WHERE id = 1",
        //     params![],
        //     |r| Ok((r.get(0)?, r.get(1)?)),
        // ).unwrap();
        // our_one_user.compartment.label = Some(Label::from_string(row.0));
        // our_one_user.compartment.set_data(row.1);
        // 
        // // so i aborted the above code because calling Repository::new() during 
        // // insider initialization requires a pointer to a user which is not yet created / loaded
        repository
    }

    pub fn load_single_from_db(&self) -> (String, String) {
        let row: (String, String) = self.connection.query_row(
            "SELECT label, secret FROM secrets WHERE id = 1",
            params![],
            |r| Ok((r.get(0)?, r.get(1)?)),
        ).unwrap();
        row
    }
    
    // fn load_all(conn: &Connection) -> Result<HashMap<String, Entry>> {
    //     let mut stmt = conn.prepare("SELECT label, secret FROM secrets")?;
    //     let entries: Vec<Entry> = stmt
    //         .query_map([], |r| {
    //             Ok(Entry { id: r.get(0)?, label: r.get(1)?, secret: r.get(2)? })
    //         })?
    //         .collect::<Result<Vec<_>>>()?;
    //     let map: HashMap<String, Entry> = entries.into_iter().map(|e| (e.label.clone(), e)).collect();
    //     Ok(map)
    // }

    pub fn add(&self, label: String, secret: String) {
        let _result = self.connection.execute("
            INSERT INTO secrets (label, secret) VALUES (?1, ?2)",
           (&label, &secret))
        .unwrap();
    }

    pub fn _retr(&self, label: &str) -> String {
        let result: String = self.connection.query_row(
            "SELECT secret FROM iracus WHERE label = ?1",
            params!(label),
            |r| r.get(0),
        ).unwrap();
        result
    }

    pub fn update(&self, label: &str, secret: &str) {
        let _ = self.connection.execute(
            "UPDATE iracus SET secret = ?1 WHERE label = ?2",
            (secret, label)
        ).unwrap();
    }

    pub fn delete(&mut self, label: &str) {
        let _ = self.connection.execute(
            "DELETE FROM iracus WHERE label = ?1",
            params![label],
        ).unwrap();
    }
    
}



