use crate::{frame::Frame, Error, Result};

pub struct Capturer {}

impl super::Engine for Capturer {
    fn new(config: &super::Config) -> Result<Self>
    where
        Self: Sized,
    {
        Err(Error::NotImplemented)
    }

    fn start(&mut self) -> Result<()> {
        todo!()
    }

    fn stop(&mut self) -> Result<()> {
        todo!()
    }

    fn get_frame(&mut self) -> Result<Frame> {
        todo!()
    }
}
