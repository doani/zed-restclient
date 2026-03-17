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
        // Für die lokale Entwicklung nutzen wir einfach "cargo run" auf dem Sidecar-Crate.
        // In einer finalen Version würde die Extension das Binary von GitHub Releases herunterladen
        // und den Pfad zur ausführbaren Datei zurückgeben.
        let manifest_path =
            "/home/dr/s3solutions/zed-rest-client-root/zed-restclient/sidecar/Cargo.toml";

        Ok(zed::Command {
            command: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                manifest_path.to_string(),
                "--bin".to_string(),
                "sidecar".to_string(),
            ],
            env: vec![],
        })
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
