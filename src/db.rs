use crate::config::load_settings;
use crate::model::Todo;
use sled::Db;
use std::ops::Range;

pub struct Database {
    db: Db,
}

impl Database {
    pub fn new() -> sled::Result<Self> {
        let config = load_settings().unwrap();

        let db = sled::open(config.db_path)?;
        Ok(Database { db })
    }

    pub fn insert(&self, item: Todo, done: bool, due_days: u16) -> sled::Result<()> {
        let num_done = if done { 1 } else { 0 };
        let key = format!("todo:{}:{}:{}", due_days, num_done, item.id);
        let value = serde_json::to_vec(&item)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        self.db.insert(key.as_bytes(), value)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn list(&self, prefix: &str) -> sled::Result<Vec<Todo>> {
        let mut result = Vec::new();
        for row in self.db.scan_prefix(prefix) {
            let (_, value) = row?;
            let todo: Todo = serde_json::from_slice(&value).unwrap();
            result.push(todo);
        }
        Ok(result)
    }

    pub fn range(&self, range: Range<&[u8]>) -> sled::Result<Vec<Todo>> {
        let mut result = Vec::new();
        for row in self.db.range(range) {
            let (_, value) = row?;
            let todo: Todo = serde_json::from_slice(&value).unwrap();
            result.push(todo);
        }
        Ok(result)
    }

    pub fn delete(&self, key: &str) -> sled::Result<Todo> {
        let old_value = self.db.remove(key)?;
        if let Some(value) = old_value {
            let todo: Todo = serde_json::from_slice(&value).unwrap();
            Ok(todo)
        } else {
            Err(sled::Error::Unsupported("Key not found".into()))
        }
    }

    pub fn complete_key(&self, prefix: &str) -> sled::Result<String> {
        let mut iter = self.db.scan_prefix(prefix);
        if let Some(Ok((key, _))) = iter.next() {
            let key_str =
                String::from_utf8(key.to_vec()).unwrap_or("Key is not utf8-encoded.".to_string());
            Ok(key_str)
        } else {
            Err(sled::Error::Unsupported("Key not found".into()))
        }
    }
}
//
// TESTS
//

#[cfg(test)]
mod tests {
    use crate::model::Priority;

    use super::*;
    use sled;
    use tempfile::TempDir;
    use uuid::Uuid;

    fn setup_database() -> Database {
        let temp_dir = TempDir::new().unwrap();
        #[allow(unused)] // TODO: Move temp_path to settings!
        let temp_path = temp_dir.path().to_str().expect("Pfad ist ungültiges UTF-8");
        Database::new().unwrap()
    }

    fn make_task(tite: String) -> Todo {
        Todo {
            id: Uuid::new_v4().to_string(),
            title: tite,
            due_date: None,
            finished: false,
            priority: Priority::Low,
            tags: vec!["test".to_string(), "qa".to_string()],
            repeats: None,
        }
    }

    #[test]
    fn test_insert_task() -> sled::Result<()> {
        let db = setup_database();

        let task = make_task("Test Task I".to_string());

        let short_key = format!("todo:0:0:{}", &task.id[..4]);
        db.insert(task, false, 0)?;

        Ok(())
    }

    #[test]
    fn test_completion() -> sled::Result<()> {
        let db = setup_database();

        let task = make_task("Test Task I".to_string());
        let short_key = format!("todo:0:0:{}", &task.id[..4]);
        db.insert(task, false, 0)?;

        let result = db.complete_key(&short_key)?;
        Ok(())
    }

    #[test]
    fn test_finish_task() -> sled::Result<()> {
        Ok(())
    }
}
