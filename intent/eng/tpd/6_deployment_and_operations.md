---
verblock: "17 Jul 2025:v2.0.0: Matthew Sinclair - Updated for Intent v2.0.0 (As-Built)"
intent_version: 2.0.0
---
# 6. Deployment and Operations [AS-BUILT]

[index](<./technical_product_design.md>)

## 6.1 Installation [AS-BUILT]

Intent v2.0.0 provides multiple installation methods with enhanced setup:

### 6.1.1 Global Installation with Bootstrap

The recommended approach uses the bootstrap command:

```bash
# Clone the Intent repository
git clone https://github.com/matthewsinclair/intent.git ~/intent

# Run bootstrap for automatic setup
cd ~/intent
./bin/intent bootstrap

# Follow the instructions to add to PATH:
export INTENT_HOME=~/intent
export PATH=$PATH:$INTENT_HOME/bin
```

### 6.1.2 Manual Global Installation

```bash
# Clone repository
git clone https://github.com/matthewsinclair/intent.git ~/intent

# Add to shell profile (.bashrc, .zshrc, etc.)
echo 'export INTENT_HOME=~/intent' >> ~/.bashrc
echo 'export PATH=$PATH:$INTENT_HOME/bin' >> ~/.bashrc

# Create global config
mkdir -p ~/.config/intent
echo '{
  "author": "Your Name",
  "editor": "vim"
}' > ~/.config/intent/config.json

# Reload shell
source ~/.bashrc
```

### 6.1.3 Project-Specific Installation

Intent can be installed per-project:

```bash
# From your project directory
git clone https://github.com/matthewsinclair/intent.git .intent-install

# Create local alias
alias intent='./.intent-install/bin/intent'

# Or add to PATH for this project
export PATH=$PATH:$(pwd)/.intent-install/bin
```

### 6.1.4 System Requirements

- POSIX-compliant shell (bash 3.2+, zsh)
- Git for version control
- jq for JSON parsing (required)
- Optional: Backlog.md for task management

## 6.2 Project Initialization [AS-BUILT]

Intent v2.0.0 provides streamlined project setup:

```bash
# Navigate to project directory
cd my-project

# Initialize Intent
intent init "Project Name"
```

This creates:
- `.intent/config.json` - Project configuration
- `intent/` - Main directory structure
- `intent/st/` - Steel threads directory
- `intent/wip.md` - Work in progress
- `CLAUDE.md` - LLM guidelines

### 6.2.1 Configuration During Init

The init command prompts for:
- Project name
- Author name (defaults to $USER)
- Editor preference (defaults to $EDITOR)
- Backlog directory (defaults to "backlog")

## 6.3 Configuration [AS-BUILT]

Intent v2.0.0 uses hierarchical JSON configuration:

### 6.3.1 Configuration Hierarchy

1. **Environment Variables** (highest priority)
   - `INTENT_HOME` - Installation directory
   - `AUTHOR` - Default author
   - `EDITOR` - Text editor
   - `INTENT_*` - Override any config value

2. **Local Project Config** (`.intent/config.json`)
   ```json
   {
     "version": "2.0.0",
     "project_name": "My Project",
     "author": "username",
     "created": "2025-07-17",
     "st_prefix": "ST",
     "backlog_dir": "backlog",
     "intent_dir": "intent",
     "backlog_list_status": "todo"
   }
   ```

3. **Global User Config** (`~/.config/intent/config.json`)
   ```json
   {
     "author": "Your Name",
     "editor": "vim",
     "backlog_list_status": "wip"
   }
   ```

4. **Built-in Defaults**

### 6.3.2 Configuration Management

```bash
# Check configuration
intent doctor

# Fix configuration issues
intent doctor --fix

# View effective configuration
intent config show  # (if implemented)
```

## 6.4 Operations [AS-BUILT]

### 6.4.1 Creating Steel Threads

```bash
# Create a new steel thread
intent st new "Implement Feature X"
# Creates: intent/st/ST####/info.md
```

Features:
- Auto-increments thread ID
- Creates directory structure
- Populates info.md template
- Optional: Creates design.md, tasks.md

### 6.4.2 Working with Steel Threads

```bash
# List all steel threads
intent st list

# List by status
intent st list --status "In Progress"

# Show thread contents
intent st show ST0001
intent st show ST0001 design  # Show specific file

# Edit thread files
intent st edit ST0001         # Edit info.md
intent st edit ST0001 tasks   # Edit tasks.md

# Task integration
intent task create ST0001 "Implement login"
intent task list ST0001
intent status show ST0001
intent status sync ST0001
```

### 6.4.3 Enhanced Backlog Integration

```bash
# List tasks (respects backlog_list_status config)
intent bl list

# List all tasks regardless of status
intent bl list --all

# Create task linked to thread
intent bl create "ST0001 - Implement feature"

# Mark task complete
intent bl done task-123
```

### 6.4.4 Migration from STP

```bash
# Upgrade any STP version to Intent v2.0.0
intent upgrade

# Custom backup directory
intent upgrade --backup-dir ./pre-v2-backup
```

### 6.4.5 LLM Integration

```bash
# Display usage rules for LLMs
intent llm usage_rules

# Create symlink for LLM access
intent llm usage_rules --symlink

# Access project guidelines
cat CLAUDE.md
```

## 6.5 Maintenance [AS-BUILT]

### 6.5.1 Diagnostics

```bash
# Check for issues
intent doctor

# Auto-fix problems
intent doctor --fix

# Verbose output
intent doctor --verbose
```

Doctor checks:
- Configuration validity
- Directory structure
- JSON syntax
- Dependencies (jq)
- Version compatibility

### 6.5.2 Updating Intent

```bash
# Update global installation
cd $INTENT_HOME
git pull

# Re-run bootstrap if needed
intent bootstrap --force
```

### 6.5.3 Backup and Recovery

Intent documents should be version controlled:

```bash
# Add Intent files to git
git add .intent/ intent/ CLAUDE.md
git commit -m "Update Intent documentation"

# Exclude Backlog.md (has own git repo)
echo "backlog/" >> .gitignore
```

### 6.5.4 Testing

```bash
# Run all tests
cd $INTENT_HOME
./tests/run_tests.sh

# Run specific test suite
./tests/run_tests.sh tests/unit/st_commands.bats

# Verbose output
./tests/run_tests.sh -v
```

## 6.6 Migration Guide [AS-BUILT]

### 6.6.1 Migrating from STP to Intent v2.0.0

```bash
# 1. Check current version
ls -la .stp-config stp/

# 2. Run upgrade
intent upgrade

# 3. Verify migration
intent doctor
ls -la .intent/ intent/

# 4. Update shell aliases
alias stp=intent  # Temporary compatibility
```

### 6.6.2 Migration Changes

| Old (STP) | New (Intent v2.0.0) |
|-----------|--------------------|
| stp/* | intent/* |
| .stp-config | .intent/config.json |
| stp commands | intent commands |
| YAML config | JSON config |
| Nested dirs | Flattened structure |

## 6.7 Troubleshooting [AS-BUILT]

Common issues and solutions:

| Issue | Solution |
|-------|----------|
| Command not found | Run `intent bootstrap` and add to PATH |
| jq not found | Install jq: `brew install jq` or `apt install jq` |
| Permission denied | Run `chmod +x $INTENT_HOME/bin/*` |
| Config errors | Run `intent doctor --fix` |
| Migration fails | Check backup, run with `--no-backup` if safe |
| Tests fail | Ensure bash 3.2+ and BATS installed |

### 6.7.1 Debug Mode

```bash
# Enable debug output
export INTENT_DEBUG=1
intent st list

# Verbose help
intent help --verbose
```

### 6.7.2 Support Resources

- Blog series: `docs/blog/`
- GitHub issues: Project repository
- CLAUDE.md: Project-specific help
- This TPD: Technical reference

## 6.8 AS-BUILT Summary

Intent v2.0.0 deployment features:

1. **Bootstrap**: Automated global setup
2. **Doctor**: Diagnostics and fixes
3. **Upgrade**: Migration from any version
4. **JSON Config**: Hierarchical settings
5. **Enhanced UX**: Better error messages
6. **Self-Hosting**: Proven through use
