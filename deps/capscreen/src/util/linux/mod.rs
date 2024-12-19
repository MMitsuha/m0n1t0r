pub struct Screen {}

impl super::Display for Screen {
    fn list() -> crate::Result<Vec<Self>>
    where
        Self: Sized,
    {
        todo!()
    }

    fn main() -> crate::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

pub struct Availability;

impl super::Permission for Availability {
    fn has_permission() -> bool {
        todo!()
    }

    fn request_permission() -> bool {
        todo!()
    }

    fn is_supported() -> crate::Result<bool> {
        todo!()
    }
}
