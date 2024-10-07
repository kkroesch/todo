use crate::model::Todo;
use sled::Db;

const DB_PATH: &str = ".storage";

fn open_db() -> sled::Result<Db> {
    sled::open(DB_PATH)
}

pub fn insert(item: Todo, done: bool, due_days: u16) -> sled::Result<()> {
    let num_done = if done { 1 } else { 0 };
    let key = format!("todo:{}:{}:{}", due_days, num_done, item.id);
    let value =
        serde_json::to_vec(&item).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let db = open_db()?;

    db.insert(key.as_bytes(), value)?;
    db.flush()?;

    Ok(())
}

pub fn list(prefix: &str) -> sled::Result<Vec<Todo>> {
    let db = open_db()?;

    let mut result = Vec::new();
    for row in db.scan_prefix(prefix) {
        let (_, value) = row?;
        let todo: Todo = serde_json::from_slice(&value).unwrap();
        result.push(todo);
    }
    Ok(result)
}

pub fn delete(key: &str) -> sled::Result<()> {
    let db = open_db()?;
    db.remove(&key)?;
    Ok(())
}
