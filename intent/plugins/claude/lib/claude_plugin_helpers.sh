#!/bin/bash
# claude_plugin_helpers.sh - Shared install/sync/uninstall logic for Claude plugins
#
# Usage: Each plugin script sets config variables and defines callbacks,
#        then sources this file.
#
# Required config variables (set before sourcing):
#   PLUGIN_TYPE        -- display label ("skill" or "agent")
#   PLUGIN_TYPE_CAP    -- capitalized label ("Skill" or "Agent")
#   PLUGIN_TYPE_PLURAL -- display label ("skills" or "agents")
#   PLUGIN_CMD         -- command path for usage ("skills" or "subagents")
#
# Required callbacks (defined before sourcing):
#   plugin_get_manifest_path        -- echo manifest file path
#   plugin_get_source_file NAME     -- echo source file path (legacy single-root)
#   plugin_is_installed NAME        -- return 0 if installed
#   plugin_copy_to_target NAME      -- copy source to target location
#   plugin_remove_target NAME       -- remove installed item
#   plugin_checksum_target NAME     -- echo checksum of installed file
#   plugin_get_available_names      -- echo space-separated available names
#   plugin_manifest_extra NAME      -- echo extra jq fields (or empty)
#
# Optional callbacks (v2.9.0+, for multi-root discovery):
#   plugin_get_source_roots         -- echo newline-separated root dirs in
#                                      precedence order (highest first).
#                                      Default: single canon root.
#   plugin_source_path_in_root R N  -- map (root, name) to expected source path.
#                                      Required if plugin_get_source_roots is
#                                      overridden. Default derives from layout
#                                      of plugin_get_source_file.
#
# Multi-root helpers (provided by this library):
#   plugin_resolve_source_file NAME    -- first-existing source across roots
#   plugin_list_source_origins NAME    -- every root where NAME exists
#   plugin_root_tag ROOT               -- "canon" | "ext:<name>"
#   plugin_detect_shadow NAME          -- stderr warning if NAME in >1 root

# ---- Multi-Root Source Discovery (v2.9.0+) ----
#
# Default: single canon root. Plugins override to add ext roots.
# Output: one root path per line, highest precedence first.
# Honoured env vars:
#   INTENT_EXT_DISABLE=1  -- return canon root only (escape hatch)
#   INTENT_EXT_DIR=<path> -- override default ~/.intent/ext location
if ! declare -f plugin_get_source_roots >/dev/null 2>&1; then
  plugin_get_source_roots() {
    echo "$INTENT_HOME/intent/plugins/claude/${PLUGIN_CMD}"
  }
fi

# Default path resolver: derives from plugin_get_source_file's layout by
# replacing the canon root prefix with the given root. Plugins can override
# for more control.
if ! declare -f plugin_source_path_in_root >/dev/null 2>&1; then
  plugin_source_path_in_root() {
    local root="$1"
    local name="$2"
    local canon_root="$INTENT_HOME/intent/plugins/claude/${PLUGIN_CMD}"
    local canon_path
    canon_path="$(plugin_get_source_file "$name")"
    case "$canon_path" in
      "$canon_root"*)
        echo "${root}${canon_path#$canon_root}"
        ;;
      *)
        echo "$canon_path"
        ;;
    esac
  }
fi

# Tag a root path with its provenance: "canon" or "ext:<name>".
plugin_root_tag() {
  local root="$1"
  local canon_root="$INTENT_HOME/intent/plugins/claude/${PLUGIN_CMD}"
  if [ "$root" = "$canon_root" ]; then
    echo "canon"
    return 0
  fi
  local ext_base="${INTENT_EXT_DIR:-$HOME/.intent/ext}"
  case "$root" in
    "$ext_base"/*)
      local trimmed="${root#$ext_base/}"
      local ext_name="${trimmed%%/*}"
      echo "ext:${ext_name}"
      ;;
    *)
      echo "unknown"
      ;;
  esac
}

# Resolve NAME to the first source file that exists across source roots.
# Returns 0 with the path on stdout, or 1 if no root has the file.
plugin_resolve_source_file() {
  local name="$1"
  local roots_out root candidate
  roots_out="$(plugin_get_source_roots 2>/dev/null || true)"

  if [ -z "$roots_out" ]; then
    # No roots declared -- fall back to legacy single-root callback
    plugin_get_source_file "$name"
    return $?
  fi

  while IFS= read -r root; do
    [ -z "$root" ] && continue
    candidate="$(plugin_source_path_in_root "$root" "$name" 2>/dev/null)"
    [ -z "$candidate" ] && continue
    if [ -f "$candidate" ]; then
      echo "$candidate"
      return 0
    fi
  done <<< "$roots_out"

  return 1
}

# List every root where NAME exists, as "TAG|PATH" lines in precedence order.
plugin_list_source_origins() {
  local name="$1"
  local roots_out root candidate tag
  roots_out="$(plugin_get_source_roots 2>/dev/null || true)"

  if [ -z "$roots_out" ]; then
    local single
    single="$(plugin_get_source_file "$name" 2>/dev/null)"
    if [ -n "$single" ] && [ -f "$single" ]; then
      echo "canon|$single"
    fi
    return 0
  fi

  while IFS= read -r root; do
    [ -z "$root" ] && continue
    candidate="$(plugin_source_path_in_root "$root" "$name" 2>/dev/null)"
    [ -z "$candidate" ] && continue
    if [ -f "$candidate" ]; then
      tag="$(plugin_root_tag "$root")"
      echo "${tag}|${candidate}"
    fi
  done <<< "$roots_out"
}

# Print a shadow warning on stderr when NAME exists in more than one root.
# Returns 0 when single origin; 1 when shadow detected.
plugin_detect_shadow() {
  local name="$1"
  local origins
  origins="$(plugin_list_source_origins "$name")"

  [ -z "$origins" ] && return 0

  local hit_count
  hit_count=$(echo "$origins" | grep -c '|')
  [ "$hit_count" -le 1 ] && return 0

  # First origin wins (ext); report it as the shadower
  local first_tag first_path
  first_tag=$(echo "$origins" | head -1 | cut -d'|' -f1)
  first_path=$(echo "$origins" | head -1 | cut -d'|' -f2)
  echo "warning: '${name}' in ${first_path%/*} shadows canon ${PLUGIN_TYPE} (${first_tag})" >&2
  echo "  to use canon: set INTENT_EXT_DISABLE=1" >&2
  return 1
}

# ---- Manifest Operations ----

plugin_ensure_manifest() {
  local manifest_file
  manifest_file="$(plugin_get_manifest_path)"
  local manifest_dir
  manifest_dir="$(dirname "$manifest_file")"

  mkdir -p "$manifest_dir"

  if [ ! -f "$manifest_file" ]; then
    cat > "$manifest_file" << 'MANIFEST'
{
  "version": "1.0.0",
  "installed": []
}
MANIFEST
  fi
}

plugin_update_manifest() {
  local name="$1"
  local source_path="$2"

  plugin_ensure_manifest
  local manifest_file
  manifest_file="$(plugin_get_manifest_path)"

  local checksum
  checksum=$(plugin_checksum_target "$name")
  local timestamp
  timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

  # Build extra fields if callback provides any
  local extra
  extra=$(plugin_manifest_extra "$name" "$source_path")

  # Remove existing entry
  local temp_file
  temp_file=$(mktemp)
  jq "del(.installed[] | select(.name == \"$name\"))" "$manifest_file" > "$temp_file"

  # Add new entry (with optional extra fields merged in)
  if [ -n "$extra" ]; then
    jq ".installed += [{
      \"name\": \"$name\",
      \"source_path\": \"$source_path\",
      \"installed_at\": \"$timestamp\",
      \"checksum\": \"$checksum\",
      $extra
    }]" "$temp_file" > "$manifest_file"
  else
    jq ".installed += [{
      \"name\": \"$name\",
      \"source_path\": \"$source_path\",
      \"installed_at\": \"$timestamp\",
      \"checksum\": \"$checksum\"
    }]" "$temp_file" > "$manifest_file"
  fi

  rm -f "$temp_file"
}

plugin_remove_from_manifest() {
  local name="$1"
  local manifest_file
  manifest_file="$(plugin_get_manifest_path)"

  if [ ! -f "$manifest_file" ]; then
    return 0
  fi

  local temp_file
  temp_file=$(mktemp)
  jq "del(.installed[] | select(.name == \"$name\"))" "$manifest_file" > "$temp_file"
  mv "$temp_file" "$manifest_file"
}

# ---- Install ----

plugin_install() {
  require_jq || return 1
  require_claude || return 1

  if [ "$#" -eq 0 ]; then
    echo "error: no ${PLUGIN_TYPE} specified"
    echo "usage: intent claude ${PLUGIN_CMD} install <name> [name...]"
    echo "       intent claude ${PLUGIN_CMD} install --all"
    return 1
  fi

  local items_to_install=()
  local install_all=false
  local force=false

  for arg in "$@"; do
    case "$arg" in
      --all)  install_all=true ;;
      --force|-f) force=true ;;
      *)      items_to_install+=("$arg") ;;
    esac
  done

  if [ "$install_all" = true ]; then
    local available
    available=$(plugin_get_available_names)
    items_to_install=($available)
  fi

  local installed_count=0
  local skipped_count=0
  local failed_count=0

  for name in "${items_to_install[@]}"; do
    echo "installing: $name"

    local source_file
    source_file=$(plugin_resolve_source_file "$name")
    if [ -z "$source_file" ] || [ ! -f "$source_file" ]; then
      echo "  error: '$name' not found"
      ((failed_count++))
      continue
    fi

    # Emit shadow warning if NAME exists in both ext and canon
    plugin_detect_shadow "$name" >&2 || true

    if plugin_is_installed "$name"; then
      if [ "$force" = false ]; then
        echo -n "  already exists, overwrite? [y/N] "
        read -r response
        if [[ ! "$response" =~ ^[Yy]$ ]]; then
          echo "  skipped"
          ((skipped_count++))
          continue
        fi
      else
        echo "  already exists, overwriting"
      fi
    fi

    if plugin_copy_to_target "$name"; then
      echo "  installed"
      plugin_update_manifest "$name" "$(dirname "$source_file")"
      ((installed_count++))
    else
      echo "  error: failed to install"
      ((failed_count++))
    fi
  done

  echo ""
  echo "ok: $installed_count installed, $skipped_count skipped, $failed_count failed"

  if [ "$installed_count" -gt 0 ] || [ "$skipped_count" -gt 0 ]; then
    return 0
  elif [ "$failed_count" -gt 0 ]; then
    return 1
  else
    return 0
  fi
}

# ---- Sync ----

plugin_sync() {
  require_jq || return 1
  require_claude || return 1

  local manifest_file
  manifest_file="$(plugin_get_manifest_path)"

  if [ ! -f "$manifest_file" ]; then
    echo "no installed ${PLUGIN_TYPE_PLURAL} found"
    echo "hint: use 'intent claude ${PLUGIN_CMD} install' first"
    return 0
  fi

  local force=false
  for arg in "$@"; do
    case "$arg" in
      --force|-f) force=true; break ;;
    esac
  done

  echo "syncing: installed ${PLUGIN_TYPE_PLURAL}"
  echo ""

  local names
  names=$(jq -r '.installed[].name' "$manifest_file" 2>/dev/null)
  local updated_count=0
  local skipped_count=0
  local failed_count=0

  for name in $names; do
    echo "Checking ${PLUGIN_TYPE}: $name"

    local item_info
    item_info=$(jq -r ".installed[] | select(.name == \"$name\")" "$manifest_file")
    local source_path
    source_path=$(echo "$item_info" | jq -r '.source_path')
    local old_checksum
    old_checksum=$(echo "$item_info" | jq -r '.checksum')

    local source_file
    source_file=$(plugin_resolve_source_file "$name")

    if [ -z "$source_file" ] || [ ! -f "$source_file" ]; then
      # Check for renamed skill (intent-* -> in-*)
      local new_name="${name/#intent-/in-}"
      local new_source
      new_source=$(plugin_resolve_source_file "$new_name")
      if [ "$new_name" != "$name" ] && [ -n "$new_source" ] && [ -f "$new_source" ]; then
        echo "  renamed: $name -> $new_name"
        plugin_remove_target "$name"
        plugin_remove_from_manifest "$name"
        plugin_copy_to_target "$new_name"
        plugin_update_manifest "$new_name" "$new_source"
        ((updated_count++))
        continue
      fi
      echo "  error: source file not found: $source_file"
      ((failed_count++))
      continue
    fi

    local source_checksum
    source_checksum=$(calculate_checksum "$source_file")
    local target_checksum
    target_checksum=$(plugin_checksum_target "$name")

    # Three-way comparison
    if [ "$source_checksum" = "$old_checksum" ] && [ "$target_checksum" = "$old_checksum" ]; then
      echo "  up to date"
      ((skipped_count++))
      continue
    fi

    if [ "$target_checksum" != "$old_checksum" ] && [ "$source_checksum" = "$old_checksum" ]; then
      echo "  warning: modified locally"
      if [ "$force" = false ]; then
        echo -n "  overwrite local changes? [y/N] "
        read -r response
        if [[ ! "$response" =~ ^[Yy]$ ]]; then
          echo "  skipped"
          ((skipped_count++))
          continue
        fi
      else
        echo "  overwriting local changes"
      fi
    elif [ "$source_checksum" != "$old_checksum" ]; then
      echo "  update available"
    fi

    if plugin_copy_to_target "$name"; then
      echo "  updated"
      plugin_update_manifest "$name" "$source_path"
      ((updated_count++))
    else
      echo "  error: failed to update"
      ((failed_count++))
    fi
  done

  echo ""
  echo "ok: $updated_count updated, $skipped_count skipped, $failed_count failed"

  if [ "$updated_count" -gt 0 ] || [ "$skipped_count" -gt 0 ]; then
    return 0
  elif [ "$failed_count" -gt 0 ]; then
    return 1
  else
    return 0
  fi
}

# ---- Uninstall ----

plugin_uninstall() {
  require_claude || return 1

  if [ "$#" -eq 0 ]; then
    echo "error: no ${PLUGIN_TYPE} specified"
    echo "Usage: intent claude ${PLUGIN_CMD} uninstall <name> [name...]"
    echo "       intent claude ${PLUGIN_CMD} uninstall --all"
    return 1
  fi

  local items_to_remove=()
  local remove_all=false
  local force=false

  for arg in "$@"; do
    case "$arg" in
      --all)  remove_all=true ;;
      --force|-f) force=true ;;
      *)      items_to_remove+=("$arg") ;;
    esac
  done

  if [ "$remove_all" = true ]; then
    local manifest_file
    manifest_file="$(plugin_get_manifest_path)"

    if [ ! -f "$manifest_file" ]; then
      echo "no installed ${PLUGIN_TYPE_PLURAL} found"
      return 0
    fi

    items_to_remove=($(jq -r '.installed[].name' "$manifest_file" 2>/dev/null))

    if [ ${#items_to_remove[@]} -eq 0 ]; then
      echo "no intent-managed ${PLUGIN_TYPE_PLURAL} found"
      return 0
    fi
  fi

  if [ "$force" = false ]; then
    echo "will remove:"
    for name in "${items_to_remove[@]}"; do
      echo "  - $name"
    done
    echo -n "Continue? [y/N] "
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
      echo "cancelled"
      return 0
    fi
  fi

  local removed_count=0
  local skipped_count=0
  local failed_count=0

  for name in "${items_to_remove[@]}"; do
    echo "removing: $name"

    if ! plugin_is_installed "$name"; then
      echo "  not found"
      ((skipped_count++))
      continue
    fi

    # Check if managed by Intent
    local manifest_file
    manifest_file="$(plugin_get_manifest_path)"
    if [ -f "$manifest_file" ] && command -v jq >/dev/null 2>&1; then
      local is_managed
      is_managed=$(jq -r ".installed[] | select(.name == \"$name\") | .name" "$manifest_file" 2>/dev/null)
      if [ -z "$is_managed" ]; then
        echo "  warning: not managed by intent"
        if [ "$force" = false ]; then
          echo -n "  Remove anyway? [y/N] "
          read -r response
          if [[ ! "$response" =~ ^[Yy]$ ]]; then
            echo "  skipped"
            ((skipped_count++))
            continue
          fi
        fi
      fi
    fi

    if plugin_remove_target "$name"; then
      echo "  removed"
      plugin_remove_from_manifest "$name"
      ((removed_count++))
    else
      echo "  error: failed to remove"
      ((failed_count++))
    fi
  done

  echo ""
  echo "ok: $removed_count removed, $skipped_count skipped, $failed_count failed"

  if [ "$removed_count" -gt 0 ] || [ "$skipped_count" -gt 0 ]; then
    return 0
  elif [ "$failed_count" -gt 0 ]; then
    return 1
  else
    return 0
  fi
}
