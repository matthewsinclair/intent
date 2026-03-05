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
#   plugin_get_source_file NAME     -- echo source file path
#   plugin_is_installed NAME        -- return 0 if installed
#   plugin_copy_to_target NAME      -- copy source to target location
#   plugin_remove_target NAME       -- remove installed item
#   plugin_checksum_target NAME     -- echo checksum of installed file
#   plugin_get_available_names      -- echo space-separated available names
#   plugin_manifest_extra NAME      -- echo extra jq fields (or empty)

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
    source_file=$(plugin_get_source_file "$name")
    if [ ! -f "$source_file" ]; then
      echo "  error: '$name' not found"
      ((failed_count++))
      continue
    fi

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
    source_file=$(plugin_get_source_file "$name")

    if [ ! -f "$source_file" ]; then
      # Check for renamed skill (intent-* -> in-*)
      local new_name="${name/#intent-/in-}"
      local new_source
      new_source=$(plugin_get_source_file "$new_name")
      if [ "$new_name" != "$name" ] && [ -f "$new_source" ]; then
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
