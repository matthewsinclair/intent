#!/bin/bash
# setup_test_env.sh - Set up the test environment for STP
# Usage: ./setup_test_env.sh [install_dir]

# Set up colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to display error messages
error() {
  echo -e "${RED}Error: $1${NC}" >&2
  exit 1
}

# Function to display success messages
success() {
  echo -e "${GREEN}$1${NC}"
}

# Function to display warning messages
warning() {
  echo -e "${YELLOW}Warning: $1${NC}"
}

# Function to display information messages
info() {
  echo -e "$1"
}

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create lib directory if it doesn't exist
mkdir -p "$SCRIPT_DIR/lib"

# Check if Bats is already installed
if command -v bats &> /dev/null; then
  success "Bats is already installed"
else
  info "Installing Bats..."
  
  # Check if running on macOS with Homebrew available
  if [[ "$OSTYPE" == "darwin"* ]] && command -v brew &> /dev/null; then
    info "Detected macOS with Homebrew. Installing Bats using brew..."
    brew install bats-core || error "Failed to install Bats with Homebrew"
    success "Bats installed successfully using Homebrew"
  else
    # Manual installation from git
    # Default install directory
    INSTALL_DIR="/usr/local"
    
    # Use provided install directory if specified
    if [ $# -gt 0 ]; then
      INSTALL_DIR="$1"
    fi
    
    info "Installing Bats from source to $INSTALL_DIR..."
    
    # Create a temporary directory for Bats installation
    TEMP_DIR=$(mktemp -d)
    
    # Clone Bats repo
    git clone https://github.com/bats-core/bats-core.git "$TEMP_DIR/bats-core" || error "Failed to clone Bats repository"
    
    # Install Bats
    cd "$TEMP_DIR/bats-core" || error "Failed to change to Bats directory"
    
    if [ "$INSTALL_DIR" = "/usr/local" ]; then
      # Need sudo for system directories
      sudo ./install.sh "$INSTALL_DIR" || error "Failed to install Bats"
    else
      # No sudo needed for user-owned directories
      ./install.sh "$INSTALL_DIR" || error "Failed to install Bats"
    fi
    
    # Clean up
    rm -rf "$TEMP_DIR"
    
    success "Bats installed successfully to $INSTALL_DIR"
  fi
fi

# Install Bats libraries
info "Installing Bats libraries..."

# Bats Support
if [ -d "$SCRIPT_DIR/lib/bats-support" ]; then
  info "Bats Support is already installed"
else
  git clone https://github.com/bats-core/bats-support.git "$SCRIPT_DIR/lib/bats-support" || error "Failed to clone bats-support"
  success "Bats Support installed successfully"
fi

# Bats Assert
if [ -d "$SCRIPT_DIR/lib/bats-assert" ]; then
  info "Bats Assert is already installed"
else
  git clone https://github.com/bats-core/bats-assert.git "$SCRIPT_DIR/lib/bats-assert" || error "Failed to clone bats-assert"
  success "Bats Assert installed successfully"
fi

# Bats File
if [ -d "$SCRIPT_DIR/lib/bats-file" ]; then
  info "Bats File is already installed"
else
  git clone https://github.com/bats-core/bats-file.git "$SCRIPT_DIR/lib/bats-file" || error "Failed to clone bats-file"
  success "Bats File installed successfully"
fi

# Update test_helper.bash to use local libraries
if [ -f "$SCRIPT_DIR/lib/test_helper.bash" ]; then
  info "Updating test_helper.bash to use local libraries..."
  
  # Create a backup
  cp "$SCRIPT_DIR/lib/test_helper.bash" "$SCRIPT_DIR/lib/test_helper.bash.bak"
  
  # Update to use local libraries
  sed -i.tmp "s|# load '/usr/local/lib/bats-support/load.bash'|# Check if libraries exist locally\nif [ -d \"$SCRIPT_DIR/lib/bats-support\" ]; then\n  load \"$SCRIPT_DIR/lib/bats-support/load.bash\"\nfi|" "$SCRIPT_DIR/lib/test_helper.bash"
  sed -i.tmp "s|# load '/usr/local/lib/bats-assert/load.bash'|if [ -d \"$SCRIPT_DIR/lib/bats-assert\" ]; then\n  load \"$SCRIPT_DIR/lib/bats-assert/load.bash\"\nfi|" "$SCRIPT_DIR/lib/test_helper.bash"
  sed -i.tmp "s|# load '/usr/local/lib/bats-file/load.bash'|if [ -d \"$SCRIPT_DIR/lib/bats-file\" ]; then\n  load \"$SCRIPT_DIR/lib/bats-file/load.bash\"\nfi|" "$SCRIPT_DIR/lib/test_helper.bash"
  
  # Remove temporary files
  rm -f "$SCRIPT_DIR/lib/test_helper.bash.tmp"
  
  success "test_helper.bash updated successfully"
fi

# Create the tmp directory for test runs
mkdir -p "$SCRIPT_DIR/tmp"

success "Test environment setup complete!"
info ""
info "To run tests, use: ./run_tests.sh"
info "To run a specific test suite, use: ./run_tests.sh bootstrap/bootstrap_test.bats"