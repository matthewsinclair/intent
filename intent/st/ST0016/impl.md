---
verblock: "16 Jul 2025:v0.2: Matthew Sinclair - Updated with JSON config and new commands"
stp_version: 1.2.1
---
# ST0016: Implementation Details

## Implementation Order

1. **Create ST0016** with this plan (DONE)
2. **Phase 0: Test Infrastructure** (CRITICAL - DO FIRST):
   - Create all example projects (v0.0.0, v1.2.0, v1.2.1, hello-world)
   - Write comprehensive BATS test suite
   - Document expected behaviors
   - Test migration scenarios
3. **Implement new commands**:
   - `intent bootstrap` for new installations
   - `intent doctor` for diagnostics
4. **Implement configuration system**:
   - JSON config parsing
   - Config loading hierarchy
   - Environment variable handling
5. **Implement restructuring**:
   - Move bin/ to top level
   - Create lib/ structure
   - Flatten intent/ structure
6. **Implement upgrade command**:
   - Version detection (with error handling)
   - Backup mechanism
   - Migration logic
   - NO rollback (fail-forward)
7. **Update all existing commands**:
   - Config loading
   - Path resolution
   - Backwards compatibility
8. **Documentation updates**:
   - README.md
   - CHANGELOG.md
   - Migration guide
   - Troubleshooting guide
9. **Release v2.0.0**

## Technical Implementation

### Config Loading Implementation

```bash
#!/bin/bash
# Config loading for intent

load_intent_config() {
  # Initialize defaults
  INTENT_VERSION="2.0.0"
  INTENT_DIR="intent"
  BACKLOG_DIR="backlog"
  
  # Find project root
  PROJECT_ROOT=$(find_project_root)
  
  # Load global config (XDG standard location)
  if [ -f "$HOME/.config/intent/config.json" ]; then
    eval "$(parse_json "$HOME/.config/intent/config.json" "global_")"
    [ -n "$global_intent_dir" ] && INTENT_DIR="$global_intent_dir"
    [ -n "$global_backlog_dir" ] && BACKLOG_DIR="$global_backlog_dir"
    [ -n "$global_author" ] && AUTHOR="$global_author"
    [ -n "$global_editor" ] && EDITOR="$global_editor"
  fi
  
  # Load local config (overrides global)
  if [ -f "$PROJECT_ROOT/.intent/config.json" ]; then
    eval "$(parse_json "$PROJECT_ROOT/.intent/config.json" "local_")"
    [ -n "$local_intent_dir" ] && INTENT_DIR="$local_intent_dir"
    [ -n "$local_backlog_dir" ] && BACKLOG_DIR="$local_backlog_dir"
    [ -n "$local_author" ] && AUTHOR="$local_author"
    [ -n "$local_editor" ] && EDITOR="$local_editor"
  fi
  
  # Environment variables override all
  [ -n "$INTENT_DIR_OVERRIDE" ] && INTENT_DIR="$INTENT_DIR_OVERRIDE"
  [ -n "$BACKLOG_DIR_OVERRIDE" ] && BACKLOG_DIR="$BACKLOG_DIR_OVERRIDE"
  
  # Legacy support: check for stp directory if intent doesn't exist
  if [ ! -d "$PROJECT_ROOT/$INTENT_DIR" ] && [ -d "$PROJECT_ROOT/stp" ]; then
    INTENT_DIR="stp"
  fi
  
  # Export for use in subcommands
  export INTENT_VERSION INTENT_DIR BACKLOG_DIR AUTHOR EDITOR PROJECT_ROOT
}

parse_json() {
  local file=$1
  local prefix=$2
  # Simple JSON parser for flat config structure
  # Extracts key-value pairs from JSON
  grep -E '^\s*"[^"]+"\s*:\s*"[^"]*"' "$file" | \
    sed -E 's/^\s*"([^"]+)"\s*:\s*"([^"]*)".*/\1="\2"/' | \
    sed -e "s/^/${prefix}/"
}

find_project_root() {
  local current_dir=$(pwd)
  while [ "$current_dir" != "/" ]; do
    # New structure
    if [ -f "$current_dir/.intent/config.json" ]; then
      echo "$current_dir"
      return 0
    fi
    # Legacy structures
    if [ -d "$current_dir/stp/.config" ] || [ -f "$current_dir/.stp-config" ]; then
      echo "$current_dir"
      return 0
    fi
    current_dir=$(dirname "$current_dir")
  done
  # No project root found
  return 1
}
```

### Bootstrap Command Implementation

```bash
#!/bin/bash
# intent_bootstrap - Initial setup for new installations

bootstrap_intent() {
  echo "Intent Bootstrap v2.0.0"
  echo "======================="
  
  # 1. Detect or validate INTENT_HOME
  if [ -z "$INTENT_HOME" ]; then
    echo "INTENT_HOME not set, detecting installation directory..."
    # Crawl up from current location to find intent directory
    local current_dir=$(pwd)
    while [ "$current_dir" != "/" ]; do
      if [ -f "$current_dir/bin/intent" ] && [ -d "$current_dir/lib" ]; then
        INTENT_HOME="$current_dir"
        echo "Found intent installation at: $INTENT_HOME"
        break
      fi
      current_dir=$(dirname "$current_dir")
    done
    
    if [ -z "$INTENT_HOME" ]; then
      echo "ERROR: Could not detect intent installation directory"
      echo "Please set INTENT_HOME and run bootstrap again"
      exit 1
    fi
  fi
  
  # 2. Validate installation
  if [ ! -f "$INTENT_HOME/bin/intent" ]; then
    echo "ERROR: Invalid INTENT_HOME - intent executable not found"
    exit 1
  fi
  
  # 3. Create global config directory
  echo "Creating global config directory..."
  mkdir -p "$HOME/.config/intent"
  
  # 4. Generate initial global config if it doesn't exist
  if [ ! -f "$HOME/.config/intent/config.json" ]; then
    echo "Creating default global configuration..."
    cat > "$HOME/.config/intent/config.json" << EOF
{
  "intent_version": "2.0.0",
  "intent_dir": "intent",
  "backlog_dir": "backlog",
  "author": "${USER}",
  "editor": "${EDITOR:-vim}"
}
EOF
  fi
  
  # 5. PATH setup recommendations
  echo ""
  echo "Setup complete! Add the following to your shell configuration:"
  echo ""
  echo "  export INTENT_HOME=\"$INTENT_HOME\""
  echo "  export PATH=\"\$INTENT_HOME/bin:\$PATH\""
  echo ""
  
  # 6. Run doctor to verify
  echo "Running intent doctor to verify installation..."
  "$INTENT_HOME/bin/intent" doctor
}
```

### Doctor Command Implementation

```bash
#!/bin/bash
# intent_doctor - Configuration diagnostics and fixes

doctor_check() {
  local fix_mode=false
  [ "$1" = "--fix" ] && fix_mode=true
  
  echo "Intent Doctor v2.0.0"
  echo "===================="
  echo ""
  
  local errors=0
  local warnings=0
  
  # Check 1: INTENT_HOME
  echo -n "Checking INTENT_HOME... "
  if [ -z "$INTENT_HOME" ]; then
    echo "ERROR: Not set"
    ((errors++))
    if [ "$fix_mode" = true ]; then
      echo "  FIX: Please run 'intent bootstrap' to set up INTENT_HOME"
    fi
  elif [ ! -d "$INTENT_HOME" ]; then
    echo "ERROR: Directory does not exist"
    ((errors++))
  else
    echo "OK ($INTENT_HOME)"
  fi
  
  # Check 2: Executables
  echo -n "Checking intent executable... "
  if [ -f "$INTENT_HOME/bin/intent" ] && [ -x "$INTENT_HOME/bin/intent" ]; then
    echo "OK"
  else
    echo "ERROR: Not found or not executable"
    ((errors++))
  fi
  
  # Check 3: Global config
  echo -n "Checking global config... "
  if [ -f "$HOME/.config/intent/config.json" ]; then
    # Validate JSON syntax
    if grep -qE '^\s*\{.*\}\s*$' "$HOME/.config/intent/config.json" 2>/dev/null; then
      echo "OK"
    else
      echo "ERROR: Invalid JSON syntax"
      ((errors++))
      if [ "$fix_mode" = true ]; then
        echo "  FIX: Backing up and creating new config..."
        mv "$HOME/.config/intent/config.json" "$HOME/.config/intent/config.json.bak"
        bootstrap_intent >/dev/null 2>&1
      fi
    fi
  else
    echo "WARNING: Not found"
    ((warnings++))
    if [ "$fix_mode" = true ]; then
      echo "  FIX: Creating default global config..."
      mkdir -p "$HOME/.config/intent"
      bootstrap_intent >/dev/null 2>&1
    fi
  fi
  
  # Check 4: Local config (if in project)
  if [ -n "$PROJECT_ROOT" ] && [ -f "$PROJECT_ROOT/.intent/config.json" ]; then
    echo -n "Checking local config... "
    if grep -qE '^\s*\{.*\}\s*$' "$PROJECT_ROOT/.intent/config.json" 2>/dev/null; then
      echo "OK"
    else
      echo "ERROR: Invalid JSON syntax"
      ((errors++))
    fi
  fi
  
  # Check 5: PATH
  echo -n "Checking PATH... "
  if echo "$PATH" | grep -q "$INTENT_HOME/bin"; then
    echo "OK"
  else
    echo "WARNING: $INTENT_HOME/bin not in PATH"
    ((warnings++))
  fi
  
  # Summary
  echo ""
  echo "Summary:"
  echo "  Errors: $errors"
  echo "  Warnings: $warnings"
  
  if [ $errors -eq 0 ] && [ $warnings -eq 0 ]; then
    echo ""
    echo "✓ All checks passed!"
    return 0
  elif [ "$fix_mode" = false ] && [ $errors -gt 0 ]; then
    echo ""
    echo "Run 'intent doctor --fix' to attempt automatic fixes"
    return 1
  fi
  
  return $errors
}
```

### Upgrade Command Implementation

```bash
#!/bin/bash
# intent_upgrade implementation

upgrade_to_v2() {
  local dry_run=false
  local auto_yes=false
  
  # Parse arguments
  while [[ $# -gt 0 ]]; do
    case $1 in
      --dry-run) dry_run=true ;;
      --yes) auto_yes=true ;;
      *) echo "Unknown option: $1"; exit 1 ;;
    esac
    shift
  done
  
  # Detect current version
  local current_version=$(detect_stp_version)
  
  if [ -z "$current_version" ]; then
    echo "ERROR: Unable to determine current STP version"
    echo ""
    echo "This could mean:"
    echo "  1. This is not an STP/intent project"
    echo "  2. The project structure is corrupted"
    echo "  3. This is a very old version we don't recognize"
    echo ""
    echo "Please verify this is an STP project before proceeding."
    exit 1
  fi
  
  echo "Current version: $current_version"
  
  if [ "$current_version" = "2.0.0" ]; then
    echo "Already at version 2.0.0"
    return 0
  fi
  
  # Create backup
  local backup_dir=".backup_$(date +%Y%m%d_%H%M%S)"
  if [ "$dry_run" = false ]; then
    echo "Creating backup in $backup_dir..."
    mkdir -p "$backup_dir"
    
    # Backup all relevant directories
    [ -d "stp" ] && cp -r stp "$backup_dir/"
    [ -f ".stp-config" ] && cp .stp-config "$backup_dir/"
    [ -d ".intent" ] && cp -r .intent "$backup_dir/"
  fi
  
  # Migration plan
  echo -e "\nMigration plan:"
  echo "1. Move stp/bin/* → bin/"
  echo "2. Move stp/_templ/* → lib/templates/"
  echo "3. Move stp/prj/st/* → intent/st/"
  echo "4. Move stp/eng/* → intent/eng/"
  echo "5. Move stp/usr/* → intent/ref/"
  echo "6. Convert configs to JSON format"
  echo "7. Create .intent/config.json"
  
  if [ "$auto_yes" = false ] && [ "$dry_run" = false ]; then
    read -p "Proceed with migration? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
      echo "Migration cancelled"
      return 1
    fi
  fi
  
  if [ "$dry_run" = true ]; then
    echo -e "\n[DRY RUN] No changes made"
    return 0
  fi
  
  # Perform migration
  echo -e "\nPerforming migration..."
  
  # Move bin files
  if [ -d "stp/bin" ]; then
    echo "Moving executables to bin/..."
    mkdir -p bin
    mv stp/bin/* bin/
    # Rename stp to intent
    [ -f "bin/stp" ] && mv bin/stp bin/intent
    # Create compatibility symlink
    ln -s intent bin/stp
    # Rename all stp_* to intent_*
    for file in bin/stp_*; do
      [ -f "$file" ] && mv "$file" "${file/stp_/intent_}"
    done
  fi
  
  # Move templates
  if [ -d "stp/_templ" ]; then
    echo "Moving templates to lib/..."
    mkdir -p lib
    mv stp/_templ lib/templates
  fi
  
  # Create intent directory and move content
  mkdir -p intent
  
  # Flatten steel threads
  if [ -d "stp/prj/st" ]; then
    echo "Flattening steel thread structure..."
    mkdir -p intent/st
    # Move all subdirectories
    for dir in stp/prj/st/*/; do
      [ -d "$dir" ] && mv "$dir" intent/st/
    done
    # Move any files
    find stp/prj/st -maxdepth 1 -type f -exec mv {} intent/st/ \;
  fi
  
  # Move other directories
  [ -d "stp/eng" ] && mv stp/eng intent/
  [ -d "stp/usr" ] && mv stp/usr intent/ref
  [ -d "stp/llm" ] && mv stp/llm intent/
  [ -d "stp/_archive" ] && mv stp/_archive intent/
  
  # Create config
  echo "Creating .intent/config.json..."
  mkdir -p .intent
  cat > .intent/config.json << EOF
{
  "intent_version": "2.0.0",
  "intent_dir": "intent",
  "backlog_dir": "backlog",
  "author": "${AUTHOR:-$USER}",
  "editor": "${EDITOR:-vim}"
}
EOF
  
  # Cleanup old structure
  if [ -d "stp" ]; then
    # Check if directory is empty
    if [ -z "$(ls -A stp)" ]; then
      rmdir stp
    else
      echo "Warning: stp/ directory not empty, manual cleanup required"
    fi
  fi
  
  echo -e "\nMigration complete!"
  echo "Backup saved in: $backup_dir"
  echo ""
  echo "Next steps:"
  echo "1. Update your PATH to include the new bin/ directory"
  echo "2. Run 'intent doctor' to verify the migration"
  echo "3. Review the changes and test your commands"
}

detect_stp_version() {
  # Check multiple locations for version information
  
  # 1. Check .intent/config.json (v2.0.0+)
  if [ -f ".intent/config.json" ]; then
    local version=$(grep -E '"intent_version"' ".intent/config.json" | sed -E 's/.*"intent_version"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')
    [ -n "$version" ] && echo "$version" && return 0
  fi
  
  # 2. Check stp/.config/version (v1.2.0+)
  if [ -f "stp/.config/version" ]; then
    local version=$(grep -E '^stp_version:' "stp/.config/version" | sed 's/stp_version:[[:space:]]*//')
    [ -n "$version" ] && echo "$version" && return 0
  fi
  
  # 3. Check for .stp-config (v0.0.0)
  if [ -f ".stp-config" ]; then
    echo "0.0.0"
    return 0
  fi
  
  # 4. Check for stp directory structure (assume v1.0.0)
  if [ -d "stp/prj/st" ]; then
    echo "1.0.0"
    return 0
  fi
  
  # Unable to determine version
  return 1
}
```

### Main Script Updates

```bash
#!/bin/bash
# Main intent script

# Detect if called as 'stp' for compatibility
SCRIPT_NAME=$(basename "$0")
if [ "$SCRIPT_NAME" = "stp" ]; then
  COMPAT_MODE=true
else
  COMPAT_MODE=false
fi

# Load configuration
source "$(dirname "$0")/intent_config"
load_intent_config

# Version
VERSION="2.0.0"

# Command routing
case "$1" in
  bootstrap)
    shift
    exec "$INTENT_ROOT/bin/intent_bootstrap" "$@"
    ;;
  doctor)
    shift
    exec "$INTENT_ROOT/bin/intent_doctor" "$@"
    ;;
  init)
    shift
    exec "$INTENT_ROOT/bin/intent_init" "$@"
    ;;
  st|steel-thread)
    shift
    exec "$INTENT_ROOT/bin/intent_st" "$@"
    ;;
  upgrade)
    shift
    exec "$INTENT_ROOT/bin/intent_upgrade" "$@"
    ;;
  # ... other commands
  *)
    if [ "$COMPAT_MODE" = true ]; then
      echo "Note: 'stp' command is deprecated, please use 'intent'"
    fi
    show_help
    ;;
esac
```

### Path Updates for All Commands

Each command needs updates like:

```bash
# Before
ST_DIR="$PROJECT_ROOT/stp/prj/st"

# After
ST_DIR="$PROJECT_ROOT/$INTENT_DIR/st"
```

### Testing Implementation

Create test fixtures for each version:

- `examples/v0.0.0-project/` - Ancient .stp-config format
- `examples/v1.2.0-project/` - File-based steel threads
- `examples/v1.2.1-project/` - Directory-based steel threads
- `examples/hello-world/` - New v2.0.0 structure

Run comprehensive tests before release:

```bash
./tests/run_upgrade_tests.sh
```
