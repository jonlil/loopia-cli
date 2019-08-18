use serde::ser::{
    Serialize,
    SerializeStruct,
    Serializer,
};
use loopia::error::{Error};

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut error = serializer.serialize_struct("Error", 1)?;
        error.serialize_field("error", &self.inner())?;
        error.end()
    }
}
