pub struct Writer;

// should import Device
// and make screen and etc writable

pub trait Writable {
    pub fn write(self, &str) {
        // write to some device
        // e.g. screen, disk or usb or any non trivial storage/display device
    
        // maybe treat 'writable devices' as a 'writable class'
    }
}
