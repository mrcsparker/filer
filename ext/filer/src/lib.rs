use magnus::{define_module, exception, function, Error};

mod rb_metadata;
mod rb_path_buf;

fn canonicalize(path: String) -> Result<rb_path_buf::RbPathBuf, Error> {
    match std::fs::canonicalize(path) {
        Ok(path) => Ok(rb_path_buf::RbPathBuf::new_from_path_buf(path)),
        Err(err) => Err(Error::new(
            exception::runtime_error(),
            format!("failed to canonicalize: {}", err),
        )),
    }
}

fn copy(from: String, to: String) -> Result<u64, Error> {
    match std::fs::copy(from, to) {
        Ok(size) => Ok(size),
        Err(err) => Err(Error::new(
            exception::runtime_error(),
            format!("failed to copy: {}", err),
        )),
    }
}

fn create_dir(path: String) -> Result<(), Error> {
    match std::fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::new(
            exception::runtime_error(),
            format!("failed to create directory: {}", err),
        )),
    }
}

fn create_dir_all(path: String) -> Result<(), Error> {
    match std::fs::create_dir_all(path) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::new(
            exception::runtime_error(),
            format!("failed to create directory: {}", err),
        )),
    }
}

fn metadata(path: String) -> Result<rb_metadata::RbMetadata, Error> {
    rb_metadata::RbMetadata::new(path)
}

fn write(path: String, contents: String) -> Result<(), Error> {
    match std::fs::write(path, contents) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::new(
            exception::runtime_error(),
            format!("failed to write: {}", err),
        )),
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let namespace = define_module("Filer")?;
    namespace.define_module_function("canonicalize", function!(canonicalize, 1))?;
    namespace.define_module_function("copy", function!(copy, 2))?;
    namespace.define_module_function("create_dir", function!(create_dir, 1))?;
    namespace.define_module_function("create_dir_all", function!(create_dir_all, 1))?;
    namespace.define_module_function("metadata", function!(metadata, 1))?;
    namespace.define_module_function("write", function!(write, 2))?;

    rb_metadata::setup(namespace)?;
    rb_path_buf::setup(namespace)?;

    Ok(())
}
