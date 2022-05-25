// Driver extensions, Translation layer
// For loading windows, linux and macos drivers

pub struct DriverTranslatorManager;

impl DriverTranslatorManager {
    pub fn new() -> Self {
        Self {}
    }

    // translate sparc service routine into a cross platform driver routine
    pub fn translate_sparc_call(&self) {}
}
