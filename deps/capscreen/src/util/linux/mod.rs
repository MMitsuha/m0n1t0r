use crate::{error::Result, Error};

pub struct Screen {}

impl super::Display for Screen {
    fn list() -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        todo!()
    }

    fn main() -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

pub struct Availability;

impl super::Permission for Availability {
    fn has_permission() -> bool {
        false
    }

    fn request_permission() -> bool {
        false
    }

    fn is_supported() -> Result<bool> {
        Ok(false)
    }
}
