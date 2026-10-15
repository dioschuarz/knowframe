---
wiki:
  content_hash: 88058249590962a1c3789adc95ce647cddae0cfef2068205ba93258a25163c44
  fragments:
    geminimd-gemini-cli-system-instructions-knowledge-index:
      tokens: 1487
      inbound: 0
      outbound: 67
    1-operating-conventions-guidelines:
      tokens: 66
      inbound: 0
      outbound: 0
    knowledge-index:
      tokens: 1376
      inbound: 0
      outbound: 67
    governance-invariants:
      tokens: 23
      inbound: 0
      outbound: 1
    feature-specifications-architecture:
      tokens: 1327
      inbound: 0
      outbound: 65
    001-core-architecture-vs-code-shell:
      tokens: 123
      inbound: 0
      outbound: 6
    002-ai-context-observatory-curated-knowledge-ux:
      tokens: 125
      inbound: 0
      outbound: 6
    003-knowledge-quality-admission-control-debt:
      tokens: 117
      inbound: 0
      outbound: 6
    004-fragment-architecture-frontmatter-layer-quality-scorer:
      tokens: 121
      inbound: 0
      outbound: 6
    005-knowledge-organization-systems-method-evolution:
      tokens: 123
      inbound: 0
      outbound: 6
    006-management-experience-ui-webview:
      tokens: 129
      inbound: 0
      outbound: 7
    007-strategy-selection-combination:
      tokens: 141
      inbound: 0
      outbound: 7
    008-ai-oriented-documentation-governance:
      tokens: 143
      inbound: 0
      outbound: 7
    009-deterministic-knowledge-engine-ipc:
      tokens: 144
      inbound: 0
      outbound: 7
    010-deterministic-traceability-pr-review-receipts:
      tokens: 155
      inbound: 0
      outbound: 7
    historical-design-records:
      tokens: 22
      inbound: 0
      outbound: 1
---
# GEMINI.md — Gemini CLI System Instructions & Knowledge Index

Welcome to `llm_wiki_ext4vsc`. This file serves as the deterministic knowledge and instructions entry point for Gemini CLI and Antigravity.

## 1. Operating Conventions & Guidelines
- **Files & Notes**: Use `snake_case` for markdown notes and documentation.
- **Code Symbols**: Use `CamelCase` for types, classes, and components.
- **Architecture**: Dual-process architecture (Rust core sidecar + TypeScript VS Code extension host).

## Knowledge Index

### Governance & Invariants
- [LLM Wiki Constitution & 16 Invariants](CONSTITUTION.md)

### Feature Specifications & Architecture

#### 001: Core Architecture & VS Code Shell
- [Feature Specification: Core Shell](specs/001-llm-wiki-ext/spec.md)
- [Implementation Plan](specs/001-llm-wiki-ext/plan.md)
- [Tasks Matrix](specs/001-llm-wiki-ext/tasks.md)
- [Data Model](specs/001-llm-wiki-ext/data-model.md)
- [Research Findings](specs/001-llm-wiki-ext/research.md)
- [Quickstart Guide](specs/001-llm-wiki-ext/quickstart.md)

#### 002: AI Context Observatory & Curated Knowledge UX
- [Feature Specification: Observatory UX](specs/002-context-observatory-ux/spec.md)
- [Implementation Plan](specs/002-context-observatory-ux/plan.md)
- [Tasks Matrix](specs/002-context-observatory-ux/tasks.md)
- [Data Model](specs/002-context-observatory-ux/data-model.md)
- [Research Findings](specs/002-context-observatory-ux/research.md)
- [Quickstart Guide](specs/002-context-observatory-ux/quickstart.md)

#### 003: Knowledge Quality Admission Control & Debt
- [Feature Specification: Admission Control](specs/003-knowledge-admission-control/spec.md)
- [Implementation Plan](specs/003-knowledge-admission-control/plan.md)
- [Tasks Matrix](specs/003-knowledge-admission-control/tasks.md)
- [Data Model](specs/003-knowledge-admission-control/data-model.md)
- [Research Findings](specs/003-knowledge-admission-control/research.md)
- [Quickstart Guide](specs/003-knowledge-admission-control/quickstart.md)

#### 004: Fragment Architecture, Frontmatter Layer & Quality Scorer
- [Feature Specification: Fragment Architecture](specs/004-fragment-architecture-quality/spec.md)
- [Implementation Plan](specs/004-fragment-architecture-quality/plan.md)
- [Tasks Matrix](specs/004-fragment-architecture-quality/tasks.md)
- [Data Model](specs/004-fragment-architecture-quality/data-model.md)
- [Research Findings](specs/004-fragment-architecture-quality/research.md)
- [Quickstart Guide](specs/004-fragment-architecture-quality/quickstart.md)

#### 005: Knowledge Organization Systems & Method Evolution
- [Feature Specification: Structural Evolution](specs/005-structural-evolution-strategies/spec.md)
- [Implementation Plan](specs/005-structural-evolution-strategies/plan.md)
- [Tasks Matrix](specs/005-structural-evolution-strategies/tasks.md)
- [Data Model](specs/005-structural-evolution-strategies/data-model.md)
- [Research Findings](specs/005-structural-evolution-strategies/research.md)
- [Quickstart Guide](specs/005-structural-evolution-strategies/quickstart.md)

#### 006: Management Experience UI & Webview
- [Feature Specification: Management UI](specs/006-management-experience-ui/spec.md)
- [Implementation Plan](specs/006-management-experience-ui/plan.md)
- [Tasks Matrix](specs/006-management-experience-ui/tasks.md)
- [Data Model](specs/006-management-experience-ui/data-model.md)
- [Research Findings](specs/006-management-experience-ui/research.md)
- [Quickstart Guide](specs/006-management-experience-ui/quickstart.md)
- [Requirements Checklist](specs/006-management-experience-ui/checklists/requirements.md)

#### 007: Strategy Selection & Combination
- [Feature Specification: Strategy Selection](specs/007-strategy-selection-combination/spec.md)
- [Implementation Plan](specs/007-strategy-selection-combination/plan.md)
- [Tasks Matrix](specs/007-strategy-selection-combination/tasks.md)
- [Data Model](specs/007-strategy-selection-combination/data-model.md)
- [Research Findings](specs/007-strategy-selection-combination/research.md)
- [Quickstart Guide](specs/007-strategy-selection-combination/quickstart.md)
- [Requirements Checklist](specs/007-strategy-selection-combination/checklists/requirements.md)

#### 008: AI-Oriented Documentation Governance
- [Feature Specification: AI Governance](specs/008-ai-doc-governance/spec.md)
- [Implementation Plan](specs/008-ai-doc-governance/plan.md)
- [Tasks Matrix](specs/008-ai-doc-governance/tasks.md)
- [Data Model](specs/008-ai-doc-governance/data-model.md)
- [Research Findings](specs/008-ai-doc-governance/research.md)
- [Quickstart Guide](specs/008-ai-doc-governance/quickstart.md)
- [Requirements Checklist](specs/008-ai-doc-governance/checklists/requirements.md)

#### 009: Deterministic Knowledge Engine & IPC
- [Feature Specification: Deterministic Engine](specs/009-deterministic-knowledge-engine/spec.md)
- [Implementation Plan](specs/009-deterministic-knowledge-engine/plan.md)
- [Tasks Matrix](specs/009-deterministic-knowledge-engine/tasks.md)
- [Data Model](specs/009-deterministic-knowledge-engine/data-model.md)
- [Research Findings](specs/009-deterministic-knowledge-engine/research.md)
- [Quickstart Guide](specs/009-deterministic-knowledge-engine/quickstart.md)
- [Requirements Checklist](specs/009-deterministic-knowledge-engine/checklists/requirements.md)

#### 010: Deterministic Traceability & PR Review Receipts
- [Feature Specification: Traceability & Reviews](specs/010-deterministic-traceability-review/spec.md)
- [Implementation Plan](specs/010-deterministic-traceability-review/plan.md)
- [Tasks Matrix](specs/010-deterministic-traceability-review/tasks.md)
- [Data Model](specs/010-deterministic-traceability-review/data-model.md)
- [Research Findings](specs/010-deterministic-traceability-review/research.md)
- [Quickstart Guide](specs/010-deterministic-traceability-review/quickstart.md)
- [Requirements Checklist](specs/010-deterministic-traceability-review/checklists/requirements.md)

### Historical & Design Records
- [Review Extension UX & Exploration](specs/chatgpt_conversation.md)
