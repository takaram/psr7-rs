use std::fs::File;
use std::io::Write;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendClassObject;
use crate::class::stream::VecStream;

#[php_class(name = "Takaram\\Psr7\\Internal\\UploadedFile")]
#[derive(Debug, Default)]
pub struct UploadedFile {
    size: Option<usize>,
    error: Option<i64>,
    client_file_name: Option<String>,
    client_media_type: Option<String>,
    file: Option<ZBox<ZendClassObject<VecStream>>>,
}

impl UploadedFile {
    pub fn new() -> Self {
        Self::default()
    }
}

#[php_impl]
impl UploadedFile {
    pub fn __construct() -> Self {
        Self::new()
    }

    pub fn get_stream(&mut self) -> PhpResult<&mut ZendClassObject<VecStream>> {
        self.file
            .as_deref_mut()
            .ok_or("Stream is unavailable.".into())
    }

    pub fn move_to(&mut self, target_path: &str) -> PhpResult<()> {
        if target_path == "" {
            return Err("Target path is empty.".into());
        }
        let stream = self.file.as_ref().ok_or("Stream is unavailable.")?;
        let mut file = File::create(target_path).map_err(|e| e.to_string())?;
        file.write_all(stream.get_body()).map_err(|e| e.to_string())?;

        // self.file is now moved and not here anymore
        self.file = None;

        Ok(())
    }

    pub fn get_size(&self) -> Option<usize> {
        self.size
    }

    pub fn get_error(&self) -> Option<i64> {
        self.error
    }

    pub fn get_client_file_name(&self) -> Option<String> {
        self.client_file_name.clone()
    }

    pub fn get_client_media_type(&self) -> Option<String> {
        self.client_media_type.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_new_stream(str: &str) -> UploadedFile {
        UploadedFile {
            size: Some(str.len()),
            error: None,
            client_file_name: None,
            client_media_type: None,
            file: Some(ZendClassObject::new(VecStream::from_str(str))),
        }
    }

    #[test]
    fn get_stream() {
        let mut uploaded_file = create_new_stream("foobar");
        let stream = uploaded_file.get_stream();
        assert_eq!(stream.get_body(), "foobar".as_bytes());
    }
}
