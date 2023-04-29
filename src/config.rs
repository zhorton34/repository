use std::collections::HashMap;

/// A contract for configuration management.
pub trait ConfigContract {
    /// Checks if a given key exists in the configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use repository::config::{ConfigContract, Repository};
    /// use std::collections::HashMap;
    ///
    /// let config_items = [("foo".to_string(), "bar".to_string())].iter().cloned().collect::<HashMap<_,_>>();
    /// let config = Repository::new(config_items);
    /// assert_eq!(config.has("foo"), true);
    /// assert_eq!(config.has("missing"), false);
    /// ```
    fn has(&self, key: &str) -> bool;

    /// Gets the value associated with the given key in the configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use repository::config::{ConfigContract, Repository};
    /// use std::collections::HashMap;
    ///
    /// let config_items = [("foo".to_string(), "bar".to_string())].iter().cloned().collect::<HashMap<_,_>>();
    /// let config = Repository::new(config_items);
    /// assert_eq!(config.get("foo"), Some("bar"));
    /// assert_eq!(config.get("missing"), None);
    /// ```
    fn get(&self, key: &str) -> Option<&str>;

    /// Sets the value associated with the given key in the configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use repository::config::{ConfigContract, Repository};
    /// use std::collections::HashMap;
    ///
    /// let mut config_items = [("foo".to_string(), "bar".to_string())].iter().cloned().collect::<HashMap<_,_>>();
    /// let mut config = Repository::new(config_items);
    /// config.set("foo", "baz");
    /// assert_eq!(config.get("foo"), Some("baz"));
    /// ```
    fn set(&mut self, key: &str, value: &str);

    /// Gets all configuration items as a reference to a hashmap.
    ///
    /// # Examples
    ///
    /// ```
    /// use repository::config::{ConfigContract, Repository};
    /// use std::collections::HashMap;
    ///
    /// let config_items = [("foo".to_string(), "bar".to_string())].iter().cloned().collect::<HashMap<_,_>>();
    /// let config = Repository::new(config_items);
    /// assert_eq!(config.all().get("foo"), Some(&"bar".to_string()));
    /// ```
    fn all(&self) -> &HashMap<String, String>;
}

/// A configuration repository that implements the ConfigContract.
pub struct Repository {
    items: HashMap<String, String>,
}

impl Repository {
    /// Constructs a new Repository with the given items.
    ///
    /// # Examples
    ///
    /// ```
    /// use repository::config::{ConfigContract, Repository};
    /// use std::collections::HashMap;
    ///
    /// let config_items = [("foo".to_string(), "bar".to_string())].iter().cloned().collect::<HashMap<_,_>>();
    /// let config = Repository::new(config_items);
    /// assert_eq!(config.get("foo"), Some("bar"));
    /// ```
    pub fn new(items: HashMap<String, String>) -> Self {
        Self { items }
    }
}


impl ConfigContract for Repository {
    /// Checks if a given key exists in the configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use repository::config::{ConfigContract, Repository};
    /// use std::collections::HashMap;
    ///
    /// let config_items = [("foo".to_string(), "bar".to_string())].iter().cloned().collect::<HashMap<_,_>>();
    /// let config = Repository::new(config_items);
    /// assert_eq!(config.has("foo"), true);
    /// assert_eq!(config.has("missing"), false);
    /// ```
    fn has(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    /// Gets the value associated with the given key in the configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use repository::config::{ConfigContract, Repository};
    /// use std::collections::HashMap;
    ///
    /// let config_items = [("foo".to_string(), "bar".to_string())].iter().cloned().collect::<HashMap<_,_>>();
    /// let config = Repository::new(config_items);
    /// assert_eq!(config.get("foo"), Some("bar"));
    /// assert_eq!(config.get("missing"), None);
    /// ```
    fn get(&self, key: &str) -> Option<&str> {
        self.items.get(key).map(|s| s.as_str())
    }

    /// Sets the value associated with the given key in the configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use repository::config::{ConfigContract, Repository};
    /// use std::collections::HashMap;
    ///
    /// let mut config_items = [("foo".to_string(), "bar".to_string())].iter().cloned().collect::<HashMap<_,_>>();
    /// let mut config = Repository::new(config_items);
    /// config.set("foo", "baz");
    /// assert_eq!(config.get("foo"), Some("baz"));
    /// ```
    fn set(&mut self, key: &str, value: &str) {
        self.items.insert(key.to_string(), value.to_string());
    }

    /// Gets all configuration items as a reference to a hashmap.
    ///
    /// # Examples
    ///
    /// ```
    /// use repository::config::{ConfigContract, Repository};
    /// use std::collections::HashMap;
    ///
    /// let config_items = [("foo".to_string(), "bar".to_string())].iter().cloned().collect::<HashMap<_,_>>();
    /// let config = Repository::new(config_items);
    /// assert_eq!(config.all().get("foo"), Some(&"bar".to_string()));
    /// ```
    fn all(&self) -> &HashMap<String, String> {
        &self.items
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has() {
        let items = HashMap::new();
        let repo = Repository::new(items);
        assert!(!repo.has("test"));
    }

    #[test]
    fn test_get() {
        let mut items = HashMap::new();
        items.insert(String::from("key"), String::from("value"));
        let repo = Repository::new(items);
        assert_eq!(Some("value"), repo.get("key"));
        assert_eq!(None, repo.get("non_existent_key"));
    }

    #[test]
    fn test_set() {
        let items = HashMap::new();
        let mut repo = Repository::new(items);
        repo.set("key", "value");
        assert_eq!(Some("value"), repo.get("key"));
    }

    #[test]
    fn test_all() {
        let mut items = HashMap::new();
        items.insert(String::from("key1"), String::from("value1"));
        items.insert(String::from("key2"), String::from("value2"));
        let repo = Repository::new(items.clone());
        assert_eq!(&items, repo.all());
    }
}
