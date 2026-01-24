# Lessons Learned

## X/Twitter (bird) login on Linux

We’re using the `bird` CLI with cookie auth.

### Load credentials

```bash
source ~/.config/bird/creds.sh
```

### Verify login

```bash
bird whoami
```

### Notes
- Keep `~/.config/bird/creds.sh` permissions tight:
  ```bash
  chmod 600 ~/.config/bird/creds.sh
  ```
- The creds file should export `AUTH_TOKEN` and `CT0`.
- If `bird check` complains about Linux keyring / cookie decryption, the Clawdbot-managed Chrome may be launched with `--password-store=basic` (no keyring integration). In that case, use manual cookies (AUTH_TOKEN/CT0) or a normal Chrome profile that uses GNOME keyring.

---

## Google Workspace (gog) auth on Linux (Gmail + Calendar)

We’re using the `gog` CLI.

### Keyring issue + fix

If you see errors like:
- `Failed to store token: ... No such interface "org.freedesktop.DBus.Properties" on object at path /`

…the auto keyring backend is broken/misdetected in this environment. Use the encrypted **file** backend instead:

```bash
gog auth keyring file
```

The file backend needs a password. In non-interactive contexts (no TTY), set:

```bash
export GOG_KEYRING_PASSWORD='clawdbot'
```

If you see:
- `no TTY available for keyring file backend password prompt; set GOG_KEYRING_PASSWORD`

…that’s the fix.

### Reset tokens (destructive)

If tokens were stored with an unknown password, wipe and re-auth:

```bash
# backup/remove encrypted token store
mv ~/.config/gogcli/keyring ~/.config/gogcli/backups/keyring.$(date -u +%Y%m%dT%H%M%SZ)
mkdir -p ~/.config/gogcli/keyring
chmod 700 ~/.config/gogcli/keyring
```

### Add account (manual OAuth paste flow)

```bash
export GOG_KEYRING_PASSWORD='clawdbot'

gog auth add you@domain.com --services gmail,calendar --manual
# follow the printed URL, then paste the redirect URL back into gog
```

Important: the pasted redirect URL must match the current run’s `state`.

### Verify

```bash
export GOG_KEYRING_PASSWORD='clawdbot'

gog auth list
```

### Example usage

Gmail:
```bash
export GOG_KEYRING_PASSWORD='clawdbot'

gog gmail search 'newer_than:2d in:inbox' --max 10 --account you@domain.com
```

Calendar:
```bash
export GOG_KEYRING_PASSWORD='clawdbot'

gog calendar events primary --from 2026-01-24T00:00:00Z --to 2026-01-31T00:00:00Z --account you@domain.com
```
