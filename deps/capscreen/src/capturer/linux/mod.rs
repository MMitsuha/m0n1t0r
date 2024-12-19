pub struct Capturer {}

impl super::Engine for Capturer {
    fn new(config: &super::Config) -> crate::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn start(&mut self) -> crate::Result<()> {
        todo!()
    }

    fn stop(&mut self) -> crate::Result<()> {
        todo!()
    }

    fn get_frame(&mut self) -> crate::Result<crate::frame::Frame> {
        todo!()
    }
}
