use std::fs::Metadata;

use magnus::{class, exception, function, method, Error, Module, Object, RModule};

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Filer::Metadata", free_immediately, size)]
pub struct RbMetadata {
    metadata: Metadata,
}

impl RbMetadata {
    pub fn new(path: String) -> Result<Self, magnus::Error> {
        match std::fs::metadata(path) {
            Ok(metadata) => Ok(Self { metadata }),
            Err(err) => Err(Error::new(
                exception::runtime_error(),
                format!("failed to get metadata: {}", err),
            )),
        }
    }

    fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }

    fn is_file(&self) -> bool {
        self.metadata.is_file()
    }

    fn is_symlink(&self) -> bool {
        self.metadata.is_symlink()
    }

    fn len(&self) -> u64 {
        self.metadata.len()
    }
}

pub fn setup(namespace: RModule) -> Result<(), magnus::Error> {
    let metadata_class = namespace.define_class("Metadata", class::object())?;
    metadata_class.define_singleton_method("new", function!(RbMetadata::new, 1))?;
    metadata_class.define_method("is_dir", method!(RbMetadata::is_dir, 0))?;
    metadata_class.define_method("is_file", method!(RbMetadata::is_file, 0))?;
    metadata_class.define_method("is_symlink", method!(RbMetadata::is_symlink, 0))?;
    metadata_class.define_method("len", method!(RbMetadata::len, 0))?;
    Ok(())
}
