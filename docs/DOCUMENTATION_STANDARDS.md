# Documentation Standards

This document defines the purpose and scope of each markdown file in the Ironhold project.

## README.md
- **Purpose:** High-level overview, goals, quick start.
- **Include:** Project summary, quick start commands.
- **Exclude:** Detailed architecture, troubleshooting.
- **Where to place extra content:** Architecture details → ARCHITECTURE.md; troubleshooting → TROUBLESHOOTING.md.

## ARCHITECTURE.md
- **Purpose:** Design decisions, architecture overview, API surface.
- **Include:** System components, data flow, API signatures.
- **Exclude:** Implementation status, task tracking.
- **Where to place extra content:** Progress → TODO.md.

## BUILD.md
- **Purpose:** Build instructions and prerequisites.
- **Include:** Commands, environment setup for wasm, windows and linux.
- **Exclude:** Editor-specific notes, architecture.
- **Where to place extra content:** Editor bootstrap → EDITOR_NOTES.md.

## CHANGELOG.md
- **Purpose:** Versioned changes.
- **Include:** Added/Changed/Fixed per release.
- **Exclude:** Developer preferences.
- **Where to place extra content:** Preferences → DEV_PREFERENCES.md.

## CODING_STANDARDS.md
- **Purpose:** Coding style rules.
- **Include:** Rust, HTML, CSS, JS standards.
- **Exclude:** Contribution workflow.
- **Where to place extra content:** Workflow → CONTRIBUTING.md.

## CONTRIBUTING.md
- **Purpose:** Contribution guidelines.
- **Include:** Branching, commit style, PR checklist.
- **Exclude:** Detailed coding standards.
- **Where to place extra content:** Coding rules → CODING_STANDARDS.md.

## DEV_PREFERENCES.md
- **Purpose:** Lead developer notes.
- **Include:** Personal workflow tips.
- **Exclude:** Official standards.

## EDITOR_NOTES.md
- **Purpose:** Editor-specific design notes.
- **Include:** UI layout, planned panels.
- **Exclude:** Build instructions.

## ROADMAP.md
- **Purpose:** High-level future goals.
- **Include:** Short, medium, long-term milestones.
- **Exclude:** Detailed tasks.
- **Where to place extra content:** Detailed tasks → TODO.md.

## TODO.md
- **Purpose:** Current sprint tasks.
- **Include:** Immediate actionable items.
- **Exclude:** Long-term ideas.
- **Where to place extra content:** Future ideas → ROADMAP.md.

## TROUBLESHOOTING.md
- **Purpose:** Known issues and fixes.

## WEBGPU_SETUP.md
- **Purpose:** WebGPU initialization flow.

---
Keep documents focused. If unsure, reference this file.
