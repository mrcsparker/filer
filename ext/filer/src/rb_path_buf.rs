use std::{borrow::BorrowMut, cell::RefCell, path::PathBuf};

use magnus::{class, exception, function, method, Error, Module, Object, RModule};

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Filer::PathBuf", free_immediately, size)]
pub struct RbPathBuf {
    path_buf: RefCell<PathBuf>,
}

impl RbPathBuf {
    pub fn new() -> Self {
        Self {
            path_buf: RefCell::new(PathBuf::new()),
        }
    }

    pub fn new_from_path_buf(path_buf: PathBuf) -> Self {
        Self {
            path_buf: RefCell::new(path_buf),
        }
    }

    // as_path() -> &Path

    pub fn push(&self, path: String) {
        self.path_buf.borrow_mut().push(path)
    }

    pub fn pop(&self) -> bool {
        self.path_buf.borrow_mut().pop()
    }

    pub fn set_file_name(&self, file_name: String) {
        self.path_buf.borrow_mut().set_file_name(file_name)
    }

    pub fn set_extension(&self, extension: String) -> bool {
        self.path_buf.borrow_mut().set_extension(extension)
    }

    // as_mut_is_string -> OsString

    // into_os_string -> OsString

    // into_boxed_path -> Box<Path>

    pub fn capacity(&self) -> usize {
        self.path_buf.borrow().capacity()
    }

    pub fn clear(&self) {
        self.path_buf.borrow_mut().clear()
    }

    pub fn reserve(&self, additional: usize) {
        self.path_buf.borrow_mut().reserve(additional)
    }

    // try_reserve -> Result<(), TryReserveError>

    pub fn reserve_exact(&self, additional: usize) {
        self.path_buf.borrow_mut().reserve_exact(additional)
    }

    // try_reserve_exact -> Result<(), TryReserveExactError>

    pub fn shrink_to_fit(&self) {
        self.path_buf.borrow_mut().shrink_to_fit()
    }

    pub fn shrink_to(&self, min_capacity: usize) {
        self.path_buf.borrow_mut().shrink_to(min_capacity)
    }
}

pub fn setup(namespace: RModule) -> Result<(), magnus::Error> {
    let path_buf_class = namespace.define_class("PathBuf", class::object())?;
    path_buf_class.define_singleton_method("new", function!(RbPathBuf::new, 0))?;
    path_buf_class.define_method("push", method!(RbPathBuf::push, 1))?;
    path_buf_class.define_method("pop", method!(RbPathBuf::pop, 0))?;
    path_buf_class.define_method("set_file_name", method!(RbPathBuf::set_file_name, 1))?;
    path_buf_class.define_method("set_extension", method!(RbPathBuf::set_extension, 1))?;
    path_buf_class.define_method("capacity", method!(RbPathBuf::capacity, 0))?;
    path_buf_class.define_method("clear", method!(RbPathBuf::clear, 0))?;
    path_buf_class.define_method("reserve", method!(RbPathBuf::reserve, 1))?;
    path_buf_class.define_method("reserve_exact", method!(RbPathBuf::reserve_exact, 1))?;
    path_buf_class.define_method("shrink_to_fit", method!(RbPathBuf::shrink_to_fit, 0))?;
    path_buf_class.define_method("shrink_to", method!(RbPathBuf::shrink_to, 1))?;
    Ok(())
}
