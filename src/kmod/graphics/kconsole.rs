// --------------
// MAIN CONSOLE
// --------------

// /sys/bin/kconsole starts the __sparx_console process
// Should be run before any windowing. After windowing, must run `term` which emulates kconsole and interfaces with /dev/console and /dev/tty0..n
// For n terms running. tty0 should be the active term
// After Init. Becomes foregrounded

// Supports colors, fonts, etc.
// Runs as a userspace program listening on stdin, stdout, stderr
// And keyboard / mouse presses

// renders to the framebuffer using the cpu
// gpu support with graphicsdriver
// basically

// TERMINAL EMULATORS
// /sys/bin/kterm
// the main kterm which can be run without windowing
// a simplistic cli environment that has control over the entire framebuffer

use alloc::string::{String, ToString};

// --------------
// VGA Color Values
// --------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

// stores all the chars that have been written to it
struct ConsoleBuffer {
    buffer: String,
    size_bytes: usize,
    n_lines: usize,
}

// the resolution framebuffer that is refreshed (usually rewritten entirely) on every write the to shell buffer
struct Framebuffer {
    framebuffer_addr: u64,
}

type Path = String;

// * for now, a font is simply the path to the font, implemented by neutron
type Font = Path;

type Resolution = (u64, u64);

// A graphical line-by-line interface
// that can be rendered directly by GraphicalShell in kext/graphics/default_shell
pub struct Console {
    resolution: Resolution,
    font: Font,
    color: ColorCode,
}

pub trait ConsoleFunctions {
    fn write(&self, _str: &str);
    fn writeln(&self, _str: &str);
    fn scroll_y(&self, offset: u64);
    fn scroll_x(&self, offset: u64);
    fn use_font(&self, font: Font);
    fn use_color(&self, color: ColorCode);
}

impl Console {
    pub fn new(resolution: Resolution, font: Font, color: ColorCode) -> Self {
        Self {
            resolution,
            font,
            color,
        }
    }
}

// might be better to do in services/ but whatever
impl ConsoleFunctions for Console {
    fn write(&self, _str: &str) {
        todo!()
    }

    fn writeln(&self, _str: &str) {
        todo!()
    }

    fn scroll_y(&self, offset: u64) {
        todo!()
    }

    fn scroll_x(&self, offset: u64) {
        todo!()
    }

    fn use_font(&self, font: Font) {
        todo!()
    }

    fn use_color(&self, color: ColorCode) {
        todo!()
    }
}

type Scheme = String;
type UniformResourceLocator = String;

// URI: Uniform Resource Identifier
// for logical resources -> uses HAL DeviceManager
struct URI {
    scheme: Scheme,
    url: UniformResourceLocator,
}

impl URI {
    pub fn new(scheme: Scheme, url: UniformResourceLocator) -> Self {
        Self { scheme, url }
    }
}

impl ToString for URI {
    fn to_string(&self) -> String {
        let mut output = String::new();
        match core::fmt::write(&mut output, format_args!("{}://{}", self.scheme, self.url)) {
            Ok(_) => output,
            Err(_) => "URI Error".to_string(),
        }
    }
}

// --------------
// KTerm
// --------------

// Reference implementation of Neutron Terminal
// Should be run as a userspace program

// only has the basic cd, ls, etc. functions. Does not have autocompletion
// rsh can be started or qsh on quantii
// rsh on a default build is started on the main terminal emulators
// like KTerm and ArcTerm (which uses KTerm in its render context)

struct KTerm {
    fg_framebuffer: ConsoleBuffer,
    bg_framebuffer: ConsoleBuffer,
}

// use ConsoleFunctions and the main Framebuffer
// allow two framebuffers
