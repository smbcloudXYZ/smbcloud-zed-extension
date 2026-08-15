# smbCloud extension for Zed

Adds the [smbCloud CLI](https://github.com/smbcloudXYZ/smbcloud-cli) as an MCP
server in Zed, so the agent can work with your mail domains, inbox routes, auth
apps, projects, tenants, and deploys.

## Install

From Zed: `zed: extensions` → search for **smbCloud** → Install.

Then sign in once from a terminal — the server reuses the session the CLI writes
to disk:

```sh
npx -y @smbcloud/cli login
```

## Settings

Configure it in Zed's settings under `context_servers.smbcloud`:

```json
{
  "context_servers": {
    "smbcloud": {
      "source": "extension",
      "settings": {
        "scope": "cloud",
        "environment": "production",
        "mail_api_key": "smb_mail_..."
      }
    }
  }
}
```

| Setting | Values | Notes |
| --- | --- | --- |
| `scope` | `cloud` (default), `automation` | `cloud` serves smbCloud resources; `automation` serves the mobile and TV device tools |
| `environment` | `production` (default), `dev` | Which API to talk to |
| `mail_api_key` | `smb_mail_...` | Only `mail_send` needs it; passed through as `SMB_MAIL_API_KEY` |

By default the extension installs `@smbcloud/cli` from npm and runs it with
Zed's bundled Node. If you already have `smb` on your machine, point at it
instead and skip the download:

```json
{
  "context_servers": {
    "smbcloud": {
      "source": "extension",
      "command": { "path": "/opt/homebrew/bin/smb" }
    }
  }
}
```

## Without the extension

The CLI is published to the [MCP registry](https://github.com/modelcontextprotocol/registry)
as `io.github.smbcloudXYZ/smbcloud-cli`, so you can also add it as a custom
local server and skip this extension entirely:

```json
{
  "context_servers": {
    "smbcloud": {
      "command": "npx",
      "args": ["-y", "@smbcloud/cli", "--mcp"],
      "env": { "SMB_MAIL_API_KEY": "smb_mail_..." }
    }
  }
}
```

## Development

```sh
rustup target add wasm32-wasip2
cargo build --release --target wasm32-wasip2
```

Then run `zed: install dev extension` and pick this directory. Logs go to
`~/Library/Logs/Zed/Zed.log`, or run `zed --foreground` for verbose output.

## Publishing

Zed's registry works by submodule. Open a PR against
[`zed-industries/extensions`](https://github.com/zed-industries/extensions) that:

1. adds this repo as a submodule at `extensions/smbcloud`
2. adds an entry to the top-level `extensions.toml` with the submodule path and version
3. runs `pnpm sort-extensions`

Keep `version` in `extension.toml` in step with that entry.

## License

Apache-2.0, matching the CLI.
