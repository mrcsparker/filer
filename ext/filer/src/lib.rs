use magnus::{define_module, exception, function, Error};

mod rb_metadata;
mod rb_path_buf;

fn canonicalize(path: String) -> Result<rb_path_buf::RbPathBuf, Error> {
    match std::fs::canonicalize(path) {
        Ok(path) => Ok(rb_path_buf::RbPathBuf::new_from_path_buf(path)),
        Err(err) => Err(Error::new(
            exception::runtime_error(),
            format!("failed to get metadata: {}", err),
        )),
    }
}

fn metadata(path: String) -> Result<rb_metadata::RbMetadata, Error> {
    rb_metadata::RbMetadata::new(path)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let namespace = define_module("Filer")?;
    namespace.define_module_function("canonicalize", function!(canonicalize, 1))?;
    namespace.define_module_function("metadata", function!(metadata, 1))?;

    rb_metadata::setup(namespace)?;
    rb_path_buf::setup(namespace)?;

    Ok(())
}
