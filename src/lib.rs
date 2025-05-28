use zed_extension_api as zed;

struct RuneLanguageExtension {
}

impl zed::Extension for RuneLanguageExtension {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(RuneLanguageExtension);
