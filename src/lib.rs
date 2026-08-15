use {
    serde::Deserialize,
    std::env,
    zed_extension_api::{
        self as zed, serde_json, settings::ContextServerSettings, Command, ContextServerId,
        ContextServerConfiguration, Project, Result,
    },
};

/// The npm package that ships the `smb` CLI. Running it with `--mcp` turns it
/// into an MCP server over stdio instead of a one-shot command.
const PACKAGE_NAME: &str = "@smbcloud/cli";
const SERVER_PATH: &str = "node_modules/@smbcloud/cli/lib/index.js";

/// User-facing settings, read from the `settings` object of the
/// `context_servers.smbcloud` entry in Zed's settings.
#[derive(Debug, Default, Deserialize)]
struct SmbcloudSettings {
    /// Tool profile: `cloud` (default) or `automation`.
    scope: Option<String>,
    /// API environment: `production` (default) or `dev`.
    environment: Option<String>,
    /// Mail app API key (`smb_mail_...`), only needed by the `mail_send` tool.
    mail_api_key: Option<String>,
}

struct SmbcloudExtension;

impl SmbcloudExtension {
    fn server_args(settings: &SmbcloudSettings) -> Vec<String> {
        let mut args = vec!["--mcp".to_string()];
        if let Some(scope) = &settings.scope {
            args.push("--scope".into());
            args.push(scope.clone());
        }
        if let Some(environment) = &settings.environment {
            args.push("--environment".into());
            args.push(environment.clone());
        }
        args
    }

    fn server_env(settings: &SmbcloudSettings) -> Vec<(String, String)> {
        settings
            .mail_api_key
            .iter()
            .filter(|key| !key.is_empty())
            .map(|key| ("SMB_MAIL_API_KEY".to_string(), key.clone()))
            .collect()
    }
}

impl zed::Extension for SmbcloudExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project(context_server_id.as_ref(), project)?;
        let options: SmbcloudSettings = settings
            .settings
            .map(serde_json::from_value)
            .transpose()
            .map_err(|err| format!("invalid `settings` for the smbCloud MCP server: {err}"))?
            .unwrap_or_default();

        let args = Self::server_args(&options);
        let mut env_vars = Self::server_env(&options);

        // An explicit `command.path` wins: it lets people point at an `smb`
        // installed via Homebrew, cargo, or the install script instead of
        // pulling the npm package.
        if let Some(command) = settings.command {
            if let Some(path) = command.path.filter(|path| !path.is_empty()) {
                env_vars.extend(command.env.into_iter().flatten());
                return Ok(Command {
                    command: path,
                    args: command.arguments.unwrap_or(args),
                    env: env_vars,
                });
            }
        }

        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        if zed::npm_package_installed_version(PACKAGE_NAME)?.as_deref()
            != Some(latest_version.as_str())
        {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        // The npm entrypoint is a shim that spawns the platform binary from
        // `optionalDependencies` with inherited stdio, so it speaks MCP fine.
        let server_path = env::current_dir()
            .map(|dir| dir.join(SERVER_PATH).to_string_lossy().into_owned())
            .unwrap_or_else(|_| SERVER_PATH.to_string());

        Ok(Command {
            command: zed::node_binary_path()?,
            args: std::iter::once(server_path).chain(args).collect(),
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        Ok(Some(ContextServerConfiguration {
            installation_instructions: include_str!("../configuration/installation_instructions.md")
                .to_string(),
            settings_schema: include_str!("../configuration/settings_schema.json").to_string(),
            default_settings: include_str!("../configuration/default_settings.jsonc").to_string(),
        }))
    }
}

zed::register_extension!(SmbcloudExtension);
