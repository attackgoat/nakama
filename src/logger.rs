use {
    super::sys::{str_as_nk_string, NkLogger},
    std::{any::Any, collections::HashMap},
};

pub struct Logger(NkLogger);

impl Logger {
    /// Log a message at DEBUG level.
    pub fn debug<S>(&self, s: S)
    where
        S: AsRef<str>,
    {
        let str_s = s.as_ref();
        let nk_s = str_as_nk_string(str_s);
        let debug = self.0.debug.unwrap();

        unsafe {
            debug(self.0.ptr, nk_s);
        }
    }

    /// Log a message at ERROR level.
    pub fn error<S>(&self, s: S)
    where
        S: AsRef<str>,
    {
        let str_s = s.as_ref();
        let nk_s = str_as_nk_string(str_s);
        let error = self.0.error.unwrap();

        unsafe {
            error(self.0.ptr, nk_s);
        }
    }

    /// Returns the fields set in this logger.
    pub fn fields(&self) -> &HashMap<String, Box<dyn Any>> {
        todo!();
    }

    /// Log a message at INFO level.
    pub fn info<S>(&self, s: S)
    where
        S: AsRef<str>,
    {
        let str_s = s.as_ref();
        let nk_s = str_as_nk_string(str_s);
        let info = self.0.info.unwrap();

        unsafe {
            info(self.0.ptr, nk_s);
        }
    }

    /// Log a message at WARN level.
    pub fn warn<S>(&self, s: S)
    where
        S: AsRef<str>,
    {
        let str_s = s.as_ref();
        let nk_s = str_as_nk_string(str_s);
        let warn = self.0.warn.unwrap();

        unsafe {
            warn(self.0.ptr, nk_s);
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

impl From<NkLogger> for Logger {
    fn from(logger: NkLogger) -> Self {
        Self(logger)
    }
}
