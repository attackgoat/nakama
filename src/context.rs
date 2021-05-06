use {
    super::sys::{str_as_nk_string, NkContext},
    std::{ffi::CStr, mem::MaybeUninit},
};

pub struct Context(NkContext);

impl Context {
    pub(crate) fn as_ptr(&self) -> *const NkContext {
        self.0.ptr as *const _
    }

    // Value returns the value associated with this context for key, or nil
    // if no value is associated with key. Successive calls to Value with
    // the same key returns the same result.
    //
    // Use context values only for request-scoped data that transits
    // processes and API boundaries, not for passing optional parameters to
    // functions.
    //
    // A key identifies a specific value in a Context. Functions that wish
    // to store values in Context typically allocate a key in a global
    // variable then use that key as the argument to context.WithValue and
    // Context.Value. A key can be any type that supports equality;
    // packages should define keys as an unexported type to avoid
    // collisions.
    pub fn value<K>(&self, key: K) -> Option<String>
    where
        K: AsRef<str>,
    {
        let mut out_value = MaybeUninit::uninit();
        let key_str = key.as_ref();
        let key_nk_string = str_as_nk_string(key_str);
        let value = self.0.value.unwrap();

        let res = {
            let out_value_ptr = out_value.as_mut_ptr();

            unsafe { value(self.0.ptr, key_nk_string, out_value_ptr) }
        };

        if res == 0 {
            let out_value_ptr = *out_value.as_ptr();
            let out_value_cstr = unsafe { CStr::from_ptr(out_value_ptr) };

            Some(out_value_cstr.to_str().unwrap().to_owned())
        } else {
            None
        }
    }
}

impl From<&NkContext> for Context {
    fn from(ctx: &NkContext) -> Self {
        Self(*ctx)
    }
}
