# Backlog.md Installation Guide

## Error: Backlog.md is not installed

The STP Backlog integration requires Backlog.md to be installed on your system.

## Installation Instructions

### Method 1: Using npm (Recommended)

```bash
npm install -g backlog-md
```

### Method 2: Using yarn

```bash
yarn global add backlog-md
```

### Method 3: From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/MrLesk/Backlog.md.git
   cd Backlog.md
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Link globally:
   ```bash
   npm link
   ```

## Verify Installation

After installation, verify that Backlog is available:

```bash
backlog --version
```

## Initialize Backlog for STP

Once installed, initialize Backlog with STP-friendly settings:

```bash
stp bl init
```

## More Information

- Backlog.md Repository: https://github.com/MrLesk/Backlog.md
- Backlog.md Documentation: https://github.com/MrLesk/Backlog.md#readme
- STP Integration Guide: Run `stp help backlog` for integration-specific help

## Troubleshooting

If you continue to see this error after installation:

1. Ensure the installation directory is in your PATH
2. Try opening a new terminal session
3. Check that the `backlog` command is accessible:
   ```bash
   which backlog
   ```

If you're using a non-standard shell or environment, you may need to manually add the npm global bin directory to your PATH.