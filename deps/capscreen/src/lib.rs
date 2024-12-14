pub mod capturer;
pub mod frame;
pub mod util;

mod error;
pub use error::*;

#[cfg(test)]
mod tests {
    use capturer::{Capturer, Config, Engine};

    use super::*;

    #[test]
    fn it_works() {
        let mut capturer = Capturer::new(&Config::main(120)).unwrap();
        capturer.start().unwrap();
        for _ in 0..100000000000000000i64 {
            let frame = capturer.get_frame().unwrap();
            match frame {
                frame::Frame::Bgra8(nv12) => println!("frame: ",),
                frame::Frame::Empty => println!("empty frame"),
                _ => println!("other frame"),
            }
        }
        capturer.stop().unwrap();
    }
}
