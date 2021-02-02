use std::{any::Any, collections::HashMap};

pub struct Logger {}

impl Logger {
    /// Log a message with optional arguments at DEBUG level. Arguments are handled in the manner of fmt.Printf.
    pub fn debug<S>(&mut self, _s: S)
    where
        S: AsRef<str>,
    {
        todo!();
    }

    /// Log a message with optional arguments at ERROR level. Arguments are handled in the manner of fmt.Printf.
    pub fn error<S>(&mut self, _s: S)
    where
        S: AsRef<str>,
    {
        todo!();
    }
    /// Returns the fields set in this logger.
    pub fn fields(&self) -> &HashMap<String, Box<dyn Any>> {
        todo!();
    }

    /// Log a message with optional arguments at INFO level. Arguments are handled in the manner of fmt.Printf.
    pub fn info<S>(&mut self, _s: S)
    where
        S: AsRef<str>,
    {
        todo!();
    }

    /// Log a message with optional arguments at WARN level. Arguments are handled in the manner of fmt.Printf.
    pub fn warn<S>(&mut self, _s: S)
    where
        S: AsRef<str>,
    {
        todo!();
    }
    /// Return a logger with the specified field set so that they are included in subsequent logging calls.
    pub fn with_field<K, V>(&self, _key: K, _val: V) -> Self
    where
        K: AsRef<str>,
        V: Any,
    {
        todo!();
    }

    /// Return a logger with the specified fields set so that they are included in subsequent logging calls.
    pub fn with_fields<K, V>(&self, _fields: HashMap<K, Box<V>>) -> Self
    where
        K: AsRef<str>,
        V: Any,
    {
        todo!();
    }
}
