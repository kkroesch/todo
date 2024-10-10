use crate::model::Todo;
use sled::Db;

const DB_PATH: &str = ".storage";

struct Database {
    db: sled::Db,
}

impl Database {
    pub fn new(path: &str) -> sled::Result<Self> {
        let db = sled::open(path)?;
        Ok(Database { db })
    }
}

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

pub fn delete(key: &str) -> sled::Result<Todo> {
    let db = open_db()?;

    let old_value = db.remove(key)?;
    if let Some(value) = old_value {
        let todo: Todo = serde_json::from_slice(&value).unwrap();
        Ok(todo)
    } else {
        Err(sled::Error::Unsupported("Key not found".into()))
    }
}

pub fn complete_key(prefix: &str) -> sled::Result<String> {
    let db = open_db()?;

    let mut iter = db.scan_prefix(prefix);
    if let Some(Ok((key, value))) = iter.next() {
        let key_str =
            String::from_utf8(key.to_vec()).unwrap_or("Key is not utf8-encoded.".to_string());
        Ok(key_str)
    } else {
        Err(sled::Error::Unsupported("Key not found".into()))
    }
}

//
// TESTS
//

#[cfg(test)]
mod tests {
    use crate::model::Priority;

    use super::*;
    //use tempfile::TempDir;
    use sled;

    #[test]
    fn test_insert_task() -> sled::Result<()> {
        let db = Database::new(".testdb");
        let task = Todo {
            id: "".into(),
            title: "Test Task I".into(),
            due_date: Some("0".into()),
            finished: false,
            priority: Priority::Low,
            tags: vec![],
            repeats: Some("".into()),
        };
        insert(task, false, 0)?;
        Ok(())
    }

    #[test]
    fn test_complete_key_found() -> sled::Result<()> {
        let result = complete_key("todo:0:1:fb49")?;
        eprintln!("{result}");
        Ok(())
    }
}
