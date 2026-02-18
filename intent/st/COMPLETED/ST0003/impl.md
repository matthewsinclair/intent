# Implementation - ST0003: Template System

## Implementation Notes

### Template Organization

Templates are organized in a directory structure that mirrors their final location:

```
_templ/
├── prj/        # Project document templates
│   ├── _wip.md
│   ├── _journal.md
│   └── st/
│       ├── _steel_threads.md
│       └── _ST####.md
├── eng/        # Engineering document templates
│   └── tpd/
│       ├── _technical_product_design.md
│       ├── _1_introduction.md
│       └── ...
├── usr/        # User document templates
│   ├── _user_guide.md
│   ├── _reference_guide.md
│   └── _deployment_guide.md
└── llm/        # LLM document templates
    └── _llm_preamble.md
```

### Template Design Principles

1. All templates begin with an underscore (\_) to distinguish them from actual documents
2. Templates include placeholders marked with [brackets] to indicate information that needs to be filled in
3. Each template includes guidance text that explains its purpose and how to complete it
4. Templates for modular documents (like technical product design) are split into logical sections
5. All templates include a version block section at the top for tracking changes

### LLM Integration

Each template includes a "Context for LLM" section that provides:

- The purpose of the document type
- Instructions for updating and maintaining the document
- Guidance on what information to include in each section
- Related documents that may be relevant
