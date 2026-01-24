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

