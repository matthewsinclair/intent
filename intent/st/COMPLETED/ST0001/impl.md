# Implementation - ST0001: Directory Structure

## Implementation Notes

### Directory Structure Created

The following directory structure has been established:

```
STP/
├── stp/                # Main STP directory
│   ├── _templ/         # Templates directory
│   │   ├── prj/        # Project document templates
│   │   │   ├── _wip.md
│   │   │   ├── _journal.md
│   │   │   └── st/
│   │   │       ├── _steel_threads.md
│   │   │       └── _ST####.md
│   │   ├── eng/        # Engineering document templates
│   │   │   └── tpd/
│   │   │       ├── _technical_product_design.md
│   │   │       ├── _1_introduction.md
│   │   │       ├── ...
│   │   ├── usr/        # User document templates
│   │   │   ├── _user_guide.md
│   │   │   ├── _reference_guide.md
│   │   │   └── _deployment_guide.md
│   │   └── llm/        # LLM document templates
│   │       └── _llm_preamble.md
│   ├── bin/            # STP scripts
│   │   ├── .help       # Help for each STP command
│   │   ├── stp         # Main STP command
│   │   ├── stp_init    # Init command implementation
│   │   ├── stp_st      # Steel thread command implementation
│   │   ├── stp_help    # Help command implementation
│   │   └── ...         # Other command implementations
│   ├── prj/            # Project documentation
│   │   ├── st/         # Steel threads
│   │   │   ├── steel_threads.md   # Steel thread index
│   │   │   ├── ST0001.md          # Individual steel thread
│   │   │   └── ...
│   │   ├── wip.md      # Work in progress
│   │   └── journal.md  # Project journal
│   ├── eng/            # Engineering docs
│   │   └── tpd/        # Technical Product Design
│   │       ├── technical_product_design.md   # Main TPD document
│   │       ├── 1_introduction.md            # TPD sections
│   │       └── ...
│   ├── usr/            # User documentation
│   │   ├── user_guide.md
│   │   ├── reference_guide.md
│   │   └── deployment_guide.md
│   └── llm/            # LLM-specific content
│       ├── llm_preamble.md
│       └── *.prompt.md     # Canned prompts
├── bin/                # Executable scripts (outside the stp structure)
```

### Naming Conventions

- All templates begin with an underscore (\_)
- All steel thread documents follow the pattern ST####.md (with 4-digit IDs)
- All scripts follow the pattern stp_command

### Template Organization

Templates are organized to mirror the actual directory structure where the instantiated files will reside. This makes it easier to understand the relationship between templates and their final locations.
