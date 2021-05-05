use {
    super::sys::{str_as_nk_string, NkLogger},
    std::{any::Any, collections::HashMap},
};

pub struct Logger(NkLogger);

impl Logger {
    /// Log a message with optional arguments at DEBUG level. Arguments are handled in the manner of fmt.Printf.
    pub fn debug<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        unsafe {
            self.0.debug.unwrap()(self.0.ptr, str_as_nk_string(s));
        }
    }

    /// Log a message with optional arguments at ERROR level. Arguments are handled in the manner of fmt.Printf.
    pub fn error<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        unsafe {
            self.0.error.unwrap()(self.0.ptr, str_as_nk_string(s));
        }
    }

    /// Returns the fields set in this logger.
    pub fn fields(&self) -> &HashMap<String, Box<dyn Any>> {
        todo!();
    }

    /// Log a message with optional arguments at INFO level. Arguments are handled in the manner of fmt.Printf.
    pub fn info<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        unsafe {
            self.0.info.unwrap()(self.0.ptr, str_as_nk_string(s));
        }
    }

    /// Log a message with optional arguments at WARN level. Arguments are handled in the manner of fmt.Printf.
    pub fn warn<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        unsafe {
            self.0.warn.unwrap()(self.0.ptr, str_as_nk_string(s));
        }
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

impl From<&NkLogger> for Logger {
    fn from(logger: &NkLogger) -> Self {
        Self(logger.clone())
    }
}
