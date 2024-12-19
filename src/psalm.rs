use zed::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json::Value, LanguageServerId, Result};

const PSALM_BINARY_NAME: &str = "psalm";

const PSALM_CONFIG_PATHS: &[&str] = &["psalm.xml"];

struct PsalmExtension;

impl PsalmExtension {
    // Returns the path if a config file exists
    pub fn config_path(&self, worktree: &zed::Worktree, settings: &Value) -> Option<String> {
        let config_path_setting = settings.get("config_path").and_then(|value| value.as_str());

        if let Some(config_path) = config_path_setting {
            if worktree.read_text_file(config_path).is_ok() {
                return Some(config_path.to_string());
            } else {
                return None;
            }
        }

        for config_path in PSALM_CONFIG_PATHS {
            if worktree.read_text_file(config_path).is_ok() {
                return Some(config_path.to_string());
            }
        }

        None
    }
}

impl zed::Extension for PsalmExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let mut args = vec!["--language-server".to_string()];

        // evaluate lsp settings
        if let Some(settings) = settings.settings {
            let require_config_file = settings
                .get("require_config_file")
                .and_then(|value| value.as_bool())
                .unwrap_or(false);

            if let Some(config_path) = self.config_path(worktree, &settings) {
                args.append(&mut vec!["--config".to_string(), config_path.clone()]);
            } else if require_config_file {
                return Err("psalm.xml is not found but require_config_file is true".to_string());
            }
        }

        // check and run psalm with custom binary
        if let Some(binary) = settings.binary {
            return Ok(zed::Command {
                command: binary
                    .path
                    .map_or(PSALM_BINARY_NAME.to_string(), |path| path),
                args: binary.arguments.map_or(args, |args| args),
                env: Default::default(),
            });
        }

        let command = worktree
            .which(PSALM_BINARY_NAME)
            .ok_or_else(|| format!("Could not find {} binary", PSALM_BINARY_NAME))?;

        Ok(zed::Command {
            command,
            args,
            env: Default::default(),
        })
    }
}

zed::register_extension!(PsalmExtension);
