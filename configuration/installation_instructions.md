# smbCloud MCP server

Gives the agent access to your smbCloud account: mail domains and inbox routes,
auth apps, projects, tenants, and deploys.

## 1. Sign in

The server reuses the session that the `smb` CLI stores on disk, so sign in once
from a terminal:

```sh
npx -y @smbcloud/cli login
```

Any tool other than `mail_send` works off that session. Nothing else to configure.

## 2. Optional: a mail API key

`mail_send` sends real email, so it needs a Mail app API key rather than your
user session. Create one in the smbCloud dashboard and put it in the settings
below as `mail_api_key` — it is passed to the server as `SMB_MAIL_API_KEY`.

## Settings

- `scope` — which tool profile to expose. `cloud` (default) serves smbCloud
  resources; `automation` serves the mobile and TV device tools instead.
- `environment` — `production` (default) or `dev`.
- `mail_api_key` — the `smb_mail_...` key described above.

If you already have `smb` installed natively (Homebrew, cargo, or the install
script), set `command.path` to its location and the extension will use that
binary instead of downloading the npm package.
