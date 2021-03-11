use {
    super::{Storage, StorageError},
    log::{debug, error},
    std::fs::OpenOptions,
    std::io::prelude::*,
    std::path::{Path, PathBuf, MAIN_SEPARATOR},
};

pub struct FileStorage {
    folder: String,
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        err.into()
    }
}

impl FileStorage {
    fn _pathfor(&self, key: &str) -> PathBuf {
        let slug = key.replace(MAIN_SEPARATOR, "-").replace(".", "-");
        Path::new(&self.folder).join(format!("{}.bin", slug))
    }
}

impl Storage for FileStorage {
    fn store(&mut self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        let path = self._pathfor(&key);
        match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)
        {
            Err(err) => {
                error!("Couldn't open or create {:?}: {}", path, err);
                return Err(StorageError::Error {
                    name: err.to_string(),
                });
            }
            Ok(mut file) => {
                file.write_all(&value)?;
                file.sync_all()?;
                debug!("Wrote {} ({} bytes) into {:?}", key, value.len(), path);
                return Ok(());
            }
        };
    }

    fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let path = self._pathfor(&key);
        let mut file = match OpenOptions::new().read(true).write(false).open(&path) {
            Ok(file) => file,
            Err(err) => {
                error!("Couldn't open {:?}: {}", path, err);
                return Ok(None);
            }
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(err) => {
                error!("Couldn't read {:?}: {}", path, err);
                return Ok(None);
            }
            Ok(size) => debug!("Read {} ({} bytes) from {:?}", key, size, path),
        };

        Ok(Some(s.into_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::{FileStorage, Storage};
    use env_logger;
    use log::error;
    use std::fs::remove_file;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn cleanup(file_path: &str) {
        if remove_file(&file_path).is_err() {
            error!("Error removing file : {}", file_path);
        };
    }

    #[test]
    fn test_store_key_value_with_no_file_present() {
        init();

        let mut storage = FileStorage {
            folder: ".".to_string(),
        };
        cleanup("./test.bin");

        storage
            .store("test", "some value".as_bytes().to_vec())
            .unwrap();

        let retrieve_result = storage.retrieve("test").unwrap();
        let value_bytes = retrieve_result.unwrap();
        let value_str = String::from_utf8(value_bytes.to_vec()).unwrap();

        assert_eq!(value_str, "some value");
        cleanup("./test.bin");
    }

    #[test]
    fn test_store_overwrite_file() {
        init();

        let mut storage = FileStorage {
            folder: ".".to_string(),
        };

        storage
            .store("test", "some value".as_bytes().to_vec())
            .unwrap();

        storage
            .store("test", "new value".as_bytes().to_vec())
            .unwrap();

        let retrieve_result = storage.retrieve("test").unwrap();
        let value_bytes = retrieve_result.unwrap();
        let value_str = String::from_utf8(value_bytes.to_vec()).unwrap();

        assert_eq!(value_str, "new value");
        cleanup("./test.bin");
    }

    #[test]
    fn test_retrieve_cannot_find_file() {
        init();

        let storage = FileStorage {
            folder: ".".to_string(),
        };
        cleanup("./test.bin");

        assert!(storage.retrieve("test").unwrap().is_none());
    }

    #[test]
    fn test_store_dangerous_key() {
        init();

        cleanup("./etc-password.bin");

        let mut storage = FileStorage {
            folder: ".".to_string(),
        };

        storage
            .store("/etc/password", "some value".as_bytes().to_vec())
            .unwrap();

        remove_file("./-etc-password.bin").unwrap(); // Fails if file is missing.
    }
}
