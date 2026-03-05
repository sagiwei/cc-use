# cc-use

CLI tool to switch [Claude Code](https://docs.anthropic.com/en/docs/claude-code) `settings.json` configurations.

Manage multiple provider configs (e.g. qwen, openai, deepseek) in `~/.cc-use/` and switch between them via symlink — no file copying, instant switching.

## How it works

```
~/.cc-use/
  qwen.json
  openai.json
  deepseek.json

~/.claude/settings.json  →  symlink to ~/.cc-use/qwen.json
```

On first use, your existing `~/.claude/settings.json` is backed up to `settings.json.bak`, then replaced with a symlink pointing to the active config.

## Install

Requires [Rust toolchain](https://rustup.rs/).

```bash
git clone git@github.com:sagiwei/cc-use.git
cd cc-use
cargo install --path .
```

The binary is installed to `~/.cargo/bin/cc-use`.

## Usage

```bash
# Interactive mode — select from a list
cc-use

# Switch directly
cc-use qwen

# Add a new config (opens $EDITOR)
cc-use add openai

# List all configs
cc-use ls

# Show current (or specified) config content
cc-use show
cc-use show qwen

# Edit an existing config
cc-use edit qwen

# Remove a config (refuses if active)
cc-use rm openai
```

## Quick start

```bash
# Save your current settings as a named config
cc-use add qwen        # editor opens, paste your settings, save & quit

# Add another provider
cc-use add deepseek    # paste deepseek settings

# Switch between them
cc-use qwen
cc-use deepseek

# Or use interactive mode
cc-use
```

## License

MIT
