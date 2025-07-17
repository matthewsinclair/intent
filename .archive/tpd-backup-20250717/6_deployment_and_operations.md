---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.2.0
---
# 6. Deployment and Operations

[index](<./technical_product_design.md>)

## 6.1 Installation

STP is designed for easy installation and use. There are two primary installation methods:

### 6.1.1 Global Installation

Global installation makes STP available system-wide:

```bash
# Clone the STP repository
git clone https://github.com/matthewsinclair/stp.git ~/stp

# Add STP bin directory to PATH in shell profile
echo 'export STP_HOME=~/stp' >> ~/.bashrc
echo 'export PATH=$PATH:$STP_HOME/bin' >> ~/.bashrc

# Reload shell configuration
source ~/.bashrc
```

### 6.1.2 Project-Specific Installation

STP can also be installed on a per-project basis:

```bash
# From your project directory
git clone https://github.com/matthewsinclair/stp.git .stp

# Create a local alias for the project
alias stp='./.stp/bin/stp'
```

## 6.2 Project Initialization

To initialize STP in an existing project:

```bash
# Navigate to project directory
cd my-project

# Initialize STP
stp init "Project Name"
```

This creates the STP directory structure within the project and populates it with template documents.

## 6.3 Configuration

STP configuration is managed through:

### 6.3.1 Environment Variables

| Variable    | Purpose                      | Default                           |
|-------------|------------------------------|-----------------------------------|
| STP_HOME    | Location of STP installation | Path to cloned repository         |
| STP_PROJECT | Current project name         | Determined from initialization    |
| STP_AUTHOR  | Default author name          | Determined from git configuration |
| STP_EDITOR  | Preferred text editor        | Determined from system defaults   |

### 6.3.2 Project-Specific Configuration

Project-specific configuration is stored in `.stp-config` in the project root:

```ini
# STP Project Configuration
PROJECT_NAME="Project Name"
AUTHOR="Default Author"
ST_PREFIX="ST"
```

## 6.4 Operations

### 6.4.1 Creating Steel Threads

```bash
# Create a new steel thread
stp st new "Implement Feature X"
```

This creates a new steel thread document with appropriate ID and initializes it with a template structure. The steel thread is also added to the steel threads index document.

### 6.4.2 Working with Steel Threads

```bash
# List all steel threads
stp st list

# List steel threads by status
stp st list --status "In Progress"

# List steel threads with custom width
stp st list --width 100

# View details of a specific steel thread
stp st show ST0001

# Edit a steel thread in your default editor
stp st edit ST0001

# Synchronize the steel threads index with individual files
stp st sync --write

# Upgrade STP files to the latest format
stp upgrade
```

### 6.4.3 Completing Steel Threads

```bash
# Mark a steel thread as complete
stp st done ST0001
```

This updates the steel thread status, adds completion date, and updates the steel threads index.

### 6.4.4 Working with Documentation

Documentation is managed through regular file operations, typically with your preferred text editor or through LLM assistance.

```bash
# Open work in progress document
$EDITOR prj/wip.md

# View technical product design
cat eng/tpd/technical_product_design.md
```

## 6.5 Maintenance

### 6.5.1 Updating STP

STP can be updated to incorporate improvements and fixes:

```bash
# Update global STP installation
cd $STP_HOME
git pull

# Update project-specific installation
cd my-project/.stp
git pull
```

### 6.5.2 Synchronizing Templates

To update project templates from the latest STP version:

```bash
stp sync templates
```

This updates the templates while preserving project-specific content.

### 6.5.3 Backup and Recovery

STP documents should be included in regular project backups, typically through version control:

```bash
# Add STP documents to version control
git add prj/ eng/ usr/ llm/
git commit -m "Update STP documentation"
```

## 6.6 Monitoring and Health

STP does not require active monitoring as it is a stateless tool. However, periodic review of documentation freshness is recommended:

```bash
# Check for outdated documents
stp health
```

## 6.7 Troubleshooting

Common issues and their solutions:

| Issue                | Solution                                                     |
|----------------------|--------------------------------------------------------------|
| Command not found    | Ensure STP_HOME is set and bin directory is in PATH          |
| Permission denied    | Check file permissions; run `chmod +x $STP_HOME/bin/*`       |
| Template errors      | Verify templates in _templ directory; reinstall if necessary |
| Configuration issues | Check .stp-config file for correct settings                  |

### 6.7.1 Logs

STP does not maintain logs by default, but can be run with verbose output:

```bash
stp --verbose command [options]
```

### 6.7.2 Support

For support, check:

- The project reference guide
- The STP repository issues
- Community forums for LLM-assisted development
