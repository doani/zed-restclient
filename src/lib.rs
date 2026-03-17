use zed_extension_api as zed;

mod parser;

struct RestClientExtension {
    // Hier können wir später den Zustand (z.B. Cached Variables oder den LSP Path) verwalten
}

impl zed::Extension for RestClientExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // Zukünftiger Einstiegspunkt für das native Sidecar (Backend)
        Err("Language Server ist noch nicht implementiert".to_string())
    }
}

zed::register_extension!(RestClientExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    // Configure wasm-bindgen-test to run in Node.js or browser
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_extension_initialization() {
        // Dummy test to ensure the extension struct can be instantiated.
        let _ext = <RestClientExtension as zed::Extension>::new();
        // Since `Extension` trait isn't bringing methods directly to `ext` that we can easily assert on without a worktree,
        // we just check that the initialization itself doesn't panic.
    }
}
