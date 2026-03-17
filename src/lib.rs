use zed_extension_api as zed;

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
