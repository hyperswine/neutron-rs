use embedded_graphics::{pixelcolor::Rgb565, prelude::DrawTarget};
use core::convert::TryInto;
use embedded_graphics::{
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};

// VGA 640*480 16 color
pub const VGA_16_GOP_ADDR: u64 = 0xA0000;

pub struct FrameBuffer {
    addr: u64,
    height: usize,
    width: usize,
}

// RGB 565

impl DrawTarget for FrameBuffer {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // self.height and self.width
            if let Ok((x @ 0..=639, y @ 0..=479)) = coord.try_into() {
                // Calculate the index in the framebuffer (16bits? 480)
                let index: u32 = x + y * 480;
            
                // color = blue | green >> 5 | red >> 11
                let color: u16 = color.b() as u16 | color.g() as u16 >> 5 | color.r() as u16 >> 11;
                unsafe {
                    core::ptr::write_volatile(self.addr as *mut u16, color)
                }
            }
        }

        Ok(())
    }
}

impl OriginDimensions for FrameBuffer {
    fn size(&self) -> Size {
        Size::new(640, 480)
    }
}

pub async fn draw_20_circle() {
    let mut display = FrameBuffer {
        addr: VGA_16_GOP_ADDR,
        height: 640,
        width: 480,
    };

    // Draw a circle with top-left at `(22, 22)` with a diameter of `20` and a white stroke
    let circle = Circle::new(Point::new(22, 22), 20)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 1));

    circle.draw(&mut display).unwrap();

    // Update the display
    display.clear(Rgb565::BLUE);
}
