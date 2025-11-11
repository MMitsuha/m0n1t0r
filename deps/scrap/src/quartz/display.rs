use core_graphics::{
    base::CGError,
    display::{CGDisplay, CGRect},
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Display(CGDisplay);

impl Display {
    pub fn primary() -> Display {
        Display(CGDisplay::main())
    }

    pub fn online() -> Result<Vec<Display>, CGError> {
        CGDisplay::active_displays().map(|ids| {
            ids.into_iter()
                .map(|id| Display(CGDisplay::new(id)))
                .collect()
        })
    }

    pub fn id(self) -> u32 {
        self.0.id
    }

    pub fn width(self) -> Option<usize> {
        self.0.display_mode().map(|mode| mode.width() as usize)
    }

    pub fn height(self) -> Option<usize> {
        self.0.display_mode().map(|mode| mode.height() as usize)
    }

    pub fn is_builtin(self) -> bool {
        self.0.is_builtin()
    }

    pub fn is_primary(self) -> bool {
        self.0.is_main()
    }

    pub fn is_active(self) -> bool {
        self.0.is_active()
    }

    pub fn is_online(self) -> bool {
        self.0.is_online()
    }

    pub fn scale(self) -> f64 {
        if let Some(display_mode) = self.0.display_mode() {
            let s = display_mode.pixel_height() as f64 / display_mode.height() as f64;

            if s > 1. {
                let enable_retina = super::ENABLE_RETINA.lock().unwrap();
                if *enable_retina {
                    return s;
                }
            }
        }
        1.
    }

    pub fn bounds(self) -> CGRect {
        self.0.bounds()
    }
}
