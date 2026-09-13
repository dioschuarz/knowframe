# Quickstart: Fragment-Level Architecture & 5-Pillar Quality Scenarios

**Feature Branch:** `004-fragment-architecture-quality`  
**Target Delivery:** `llm_wiki_ext4vsc`  

---

## Scenario 1: Pre-Commit Frontmatter Materialization & Automatic Staging

### Objective
Verify that editing a Markdown document with headings causes `llm-wiki-validate` during pre-commit to calculate token counts and reference traffic, materializing `wiki.fragments` into YAML frontmatter without mangling content, and automatically re-staging the file into the Git index.

### Steps
1. Open a test document `docs/architecture.md` containing several heading sections:
   ```markdown
   ---
   title: System Architecture
   tags: [arch, core]
   ---
   # System Architecture
   ## Authentication
   Token management and session state...
   ## Authorization
   Role-based access control rules...
   ```
2. Stage the file:
   ```bash
   git add docs/architecture.md
   ```
3. Run candidate validation:
   ```bash
   target/release/llm-wiki-core validate --staged-only
   ```
4. Verify output:
   - Frontmatter updated with:
     ```yaml
     wiki:
       fragments:
         authentication:
           tokens: 42
           inbound: 0
           outbound: 0
         authorization:
           tokens: 38
           inbound: 0
           outbound: 0
     ```
   - User keys (`title: System Architecture`, `tags: [arch, core]`) preserved untouched.
   - The hook automatically re-stages the modified file (`git add docs/architecture.md`), so `git status` confirms the file is cleanly staged for commit with zero working tree drift (Invariant I & II).

---

## Scenario 2: Section Fragment Link Resolution & Dangling Anchor Detection

### Objective
Verify that standard Markdown links with `#anchor` fragment targets are validated by the Rust AST engine, and broken anchors are caught at pre-commit.

### Steps
1. Create a reference to a valid section:
   ```markdown
   See [Authentication Details](./architecture.md#authentication).
   ```
2. Create a reference to a non-existent anchor:
   ```markdown
   See [Missing Spec](./architecture.md#non-existent-section).
   ```
3. Stage and validate:
   ```bash
   git add .
   target/release/llm-wiki-core validate --staged-only
   ```
4. Expected result:
   - Validation returns `BLOCK` (exit code 1) on new regressions (Invariant XII).
   - Diagnostic output points directly to file and line:
     ```text
     [BLOCK] Integrity Regression in notes/intro.md:14:
       Dangling anchor '#non-existent-section' targeting 'docs/architecture.md'.
       Heading anchor does not exist. Available anchors: #authentication, #authorization.
     ```

---

## Scenario 3: Context Trace Section-Level Routing & Context Reduction

### Objective
Verify that querying for a specific concept discovers the section fragment from `AGENTS.md` and quantifies context token reduction.

### Steps
1. Execute CLI trace:
   ```bash
   target/release/llm-wiki-core trace "token refresh and session authentication" --entrypoint "AGENTS.md"
   ```
2. Expected output:
   ```text
   Query: "token refresh and session authentication"
   Entrypoint: AGENTS.md
   Target: docs/architecture.md#authentication
   Path: AGENTS.md -> docs/architecture.md#authentication (Depth: 1)
   Document Tokens: 50,231
   Fragment Tokens: 5,132
   Context Reduction: 89.8% (~90%)
   Relevance Reasons:
     - Fragment heading "Authentication" matches query term "authentication"
     - Token overlap: "token", "refresh", "session"
   ```

---

## Scenario 4: Webview Fragment Inspector & 5-Pillar Health Inspection

### Objective
Verify that selecting a node in the Webview displays the 3-column Fragments table and the 5-pillar quality breakdown with strictly compliant compact formatting ($\le 5$ characters).

### Steps
1. In VS Code, run command `LLM Wiki: Open Knowledge Graph`.
2. Click on `docs/architecture.md`.
3. In the Inspector Drawer on the right:
   - Inspect the **Fragments** section:
     ```text
     Fragment          Tok     I/O
     Authentication    5.13k   31/7
     Authorization     8.42k    4/3
     Deployment        12.0k    3/5
     ```
   - All token numbers strictly obey the $\le 5$ character limit (Invariant X).
   - Click on `Authentication` row: Editor opens `docs/architecture.md` focused on line 12.
4. Click `[Review issues]` or open Diagnostics:
   - Verify 5 tabs: `All`, `Integrity`, `Structure`, `Consistency`, `Discoverability`, `Freshness`.
   - Inspect explainable reasons for Structure and Freshness scores.
