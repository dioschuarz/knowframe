---
wiki:
  content_hash: f0b202010360df568d9655fe49ae1c07686b7cf3b3e78b22d3e8ab28d5d9f5e4
  fragments:
    technical-research-methodology-formulations-deterministic-traceability-review-consistency:
      tokens: 3406
      inbound: 0
      outbound: 1
    1-research-question-1-schema-representation-for-code-dependencies:
      tokens: 1847
      inbound: 0
      outbound: 0
    context-problem:
      tokens: 47
      inbound: 0
      outbound: 0
    evaluation-of-options:
      tokens: 820
      inbound: 0
      outbound: 0
    decision-rationale:
      tokens: 965
      inbound: 0
      outbound: 0
    2-research-question-2-git-diff-ingestion-affected-documentation-algorithm:
      tokens: 562
      inbound: 0
      outbound: 0
    context-problem-1:
      tokens: 72
      inbound: 0
      outbound: 0
    algorithmic-formulation:
      tokens: 405
      inbound: 0
      outbound: 0
    performance-complexity:
      tokens: 65
      inbound: 0
      outbound: 0
    3-research-question-3-prci-review-receipts-for-reviewed-unchanged:
      tokens: 420
      inbound: 0
      outbound: 0
    context-problem-2:
      tokens: 75
      inbound: 0
      outbound: 0
    evaluated-alternatives:
      tokens: 176
      inbound: 0
      outbound: 0
    decision:
      tokens: 146
      inbound: 0
      outbound: 0
    4-research-question-4-architectural-rules-engine-vs-semantic-drift:
      tokens: 318
      inbound: 0
      outbound: 0
    context-problem-3:
      tokens: 57
      inbound: 0
      outbound: 0
    rule-specification-schema-in-context-vaultyaml:
      tokens: 153
      inbound: 0
      outbound: 0
    deterministic-rule-checker:
      tokens: 89
      inbound: 0
      outbound: 0
    5-research-question-5-graphify-spike-isolation-architecture:
      tokens: 197
      inbound: 0
      outbound: 0
    context-problem-4:
      tokens: 59
      inbound: 0
      outbound: 0
    decision-1:
      tokens: 122
      inbound: 0
      outbound: 0
---
# Technical Research & Methodology Formulations: Deterministic Traceability, Review & Consistency

**Feature Branch**: `010-deterministic-traceability-review`  
**Date**: 2026-10-13  
**Status**: Completed  
**Spec**: [spec.md](spec.md)  

---

## 1. Research Question 1: Schema Representation for Code Dependencies

### Context & Problem
We need to represent explicit dependencies linking Markdown knowledge documents to underlying source code files in the repository. Currently, `sources` in frontmatter is used for knowledge references. Reusing `sources` could break compatibility.

### Evaluation of Options
1. **Option A: Overload `sources`** with URI schemes (e.g. `file://crates/cli/src/lib.rs`).
   - *Pros*: No new frontmatter key.
   - *Cons*: Ambiguous semantics with external knowledge sources; breaks backward compatibility with existing parsers expecting URLs or titles.
2. **Option B: Dedicated `code_dependencies` Frontmatter Key** (List of relative POSIX strings).
   - *Pros*: Explicit, isolated, backward-compatible; matches the Portuguese technical specification (`LLM_Wiki_Specificacao_Camada_Deterministica.md` Section 6.1).
   - *Cons*: New key to parse in frontmatter.
*.context-vault/code_map.yaml*3*.context-vault/code_map.yaml*.*.context-vault/code_map.yaml* *.context-vault/code_map.yaml***.context-vault/code_map.yaml***.context-vault/code_map.yaml*O*.context-vault/code_map.yaml*p*.context-vault/code_map.yaml*t*.context-vault/code_map.yaml*i*.context-vault/code_map.yaml*o*.context-vault/code_map.yaml*n*.context-vault/code_map.yaml* *.context-vault/code_map.yaml*C*.context-vault/code_map.yaml*:*.context-vault/code_map.yaml* *.context-vault/code_map.yaml*C*.context-vault/code_map.yaml*e*.context-vault/code_map.yaml*n*.context-vault/code_map.yaml*t*.context-vault/code_map.yaml*r*.context-vault/code_map.yaml*a*.context-vault/code_map.yaml*l*.context-vault/code_map.yaml* *.context-vault/code_map.yaml*W*.context-vault/code_map.yaml*o*.context-vault/code_map.yaml*r*.context-vault/code_map.yaml*k*.context-vault/code_map.yaml*s*.context-vault/code_map.yaml*p*.context-vault/code_map.yaml*a*.context-vault/code_map.yaml*c*.context-vault/code_map.yaml*e*.context-vault/code_map.yaml* *.context-vault/code_map.yaml*M*.context-vault/code_map.yaml*a*.context-vault/code_map.yaml*n*.context-vault/code_map.yaml*i*.context-vault/code_map.yaml*f*.context-vault/code_map.yaml*e*.context-vault/code_map.yaml*s*.context-vault/code_map.yaml*t*.context-vault/code_map.yaml* *.context-vault/code_map.yaml*(*.context-vault/code_map.yaml*`*.context-vault/code_map.yaml*.*.context-vault/code_map.yaml*c*.context-vault/code_map.yaml*o*.context-vault/code_map.yaml*n*.context-vault/code_map.yaml*t*.context-vault/code_map.yaml*e*.context-vault/code_map.yaml*x*.context-vault/code_map.yaml*t*.context-vault/code_map.yaml*-*.context-vault/code_map.yaml*v*.context-vault/code_map.yaml*a*.context-vault/code_map.yaml*u*.context-vault/code_map.yaml*l*.context-vault/code_map.yaml*t*.context-vault/code_map.yaml*/*.context-vault/code_map.yaml*c*.context-vault/code_map.yaml*o*.context-vault/code_map.yaml*d*.context-vault/code_map.yaml*e*.context-vault/code_map.yaml*_*.context-vault/code_map.yaml*m*.context-vault/code_map.yaml*a*.context-vault/code_map.yaml*p*.context-vault/code_map.yaml*.*.context-vault/code_map.yaml*y*.context-vault/code_map.yaml*a*.context-vault/code_map.yaml*m*.context-vault/code_map.yaml*l*.context-vault/code_map.yaml*`*.context-vault/code_map.yaml*)*.context-vault/code_map.yaml***.context-vault/code_map.yaml***.context-vault/code_map.yaml*.*.context-vault/code_map.yaml*
*.context-vault/code_map.yaml*   - *Pros*: Works for documents with no frontmatter.
   - *Cons*: Violates locality; creates two diverging sources of truth.

### Decision & Rationale
*path/to/file.rs***path/to/file.rs***path/to/file.rs*A*path/to/file.rs*d*path/to/file.rs*o*path/to/file.rs*p*path/to/file.rs*t*path/to/file.rs* *path/to/file.rs*O*path/to/file.rs*p*path/to/file.rs*t*path/to/file.rs*i*path/to/file.rs*o*path/to/file.rs*n*path/to/file.rs* *path/to/file.rs*B*path/to/file.rs* *path/to/file.rs*(*path/to/file.rs*`*path/to/file.rs*c*path/to/file.rs*o*path/to/file.rs*d*path/to/file.rs*e*path/to/file.rs*_*path/to/file.rs*d*path/to/file.rs*e*path/to/file.rs*p*path/to/file.rs*e*path/to/file.rs*n*path/to/file.rs*d*path/to/file.rs*e*path/to/file.rs*n*path/to/file.rs*c*path/to/file.rs*i*path/to/file.rs*e*path/to/file.rs*s*path/to/file.rs*`*path/to/file.rs* *path/to/file.rs*f*path/to/file.rs*r*path/to/file.rs*o*path/to/file.rs*n*path/to/file.rs*t*path/to/file.rs*m*path/to/file.rs*a*path/to/file.rs*t*path/to/file.rs*t*path/to/file.rs*e*path/to/file.rs*r*path/to/file.rs* *path/to/file.rs*k*path/to/file.rs*e*path/to/file.rs*y*path/to/file.rs*)*path/to/file.rs* *path/to/file.rs*+*path/to/file.rs* *path/to/file.rs*O*path/to/file.rs*p*path/to/file.rs*t*path/to/file.rs*i*path/to/file.rs*o*path/to/file.rs*n*path/to/file.rs* *path/to/file.rs*B*path/to/file.rs* *path/to/file.rs*f*path/to/file.rs*a*path/to/file.rs*l*path/to/file.rs*l*path/to/file.rs*b*path/to/file.rs*a*path/to/file.rs*c*path/to/file.rs*k*path/to/file.rs* *path/to/file.rs*(*path/to/file.rs*i*path/to/file.rs*n*path/to/file.rs*l*path/to/file.rs*i*path/to/file.rs*n*path/to/file.rs*e*path/to/file.rs* *path/to/file.rs*M*path/to/file.rs*a*path/to/file.rs*r*path/to/file.rs*k*path/to/file.rs*d*path/to/file.rs*o*path/to/file.rs*w*path/to/file.rs*n*path/to/file.rs* *path/to/file.rs*l*path/to/file.rs*i*path/to/file.rs*n*path/to/file.rs*k*path/to/file.rs*s*path/to/file.rs* *path/to/file.rs*`*path/to/file.rs*[*path/to/file.rs*L*path/to/file.rs*a*path/to/file.rs*b*path/to/file.rs*e*path/to/file.rs*l*path/to/file.rs*]*path/to/file.rs*(*path/to/file.rs*p*path/to/file.rs*a*path/to/file.rs*t*path/to/file.rs*h*path/to/file.rs*/*path/to/file.rs*t*path/to/file.rs*o*path/to/file.rs*/*path/to/file.rs*f*path/to/file.rs*i*path/to/file.rs*l*path/to/file.rs*e*path/to/file.rs*.*path/to/file.rs*r*path/to/file.rs*s*path/to/file.rs*)*path/to/file.rs*`*path/to/file.rs*)*path/to/file.rs***path/to/file.rs***path/to/file.rs*.*path/to/file.rs*
*path/to/file.rs*- Frontmatter format:
  ```yaml
  ---
  title: Indexing Flow
  code_dependencies:
    - crates/core/src/indexing/engine.rs
    - extension/src/indexing.ts
  ---
  ```
- **Normalization Rule**: All paths are trimmed, Windows backslashes `\` converted to `/`, leading `./` stripped, and resolved relative to the repository root.
- **Path Traversal Guard**: Any path resolving with `../` that escapes the repository root is rejected at parse time with finding `PATH_TRAVERSAL_PROHIBITED`.
- **Precedence**: Frontmatter `code_dependencies` is canonical. Inline links `[Label](code_path)` are secondary observed citations. If both exist, the union is tracked with origin metadata (`DependencySource::Frontmatter` vs `DependencySource::InlineMarkdownLink`).

---

## 2. Research Question 2: Git Diff Ingestion & Affected Documentation Algorithm

### Context & Problem
How to identify which documentation is affected by a Git change across four distinct operational contexts:
1. Working tree (`git status` / uncommitted files);
2. Pre-commit hook (staged index candidate delta);
3. Commit range (`git diff A..B`);
4. Pull Request / CI (`git diff base...head`).

### Algorithmic Formulation
```text
Input:
  workspace_root: Path
  changed_files: Vec<GitFileDelta> (path: String, status: Added | Modified | Deleted | Renamed(from, to))
  reverse_index: ReverseImpactIndex

Algorithm:
  affected_docs = HashMap<DocId, ReviewObligation>

  FOR file_delta IN changed_files:
    code_path = normalize_path(file_delta.path)
    
    // 1. Direct match in reverse index
    citing_docs = reverse_index.get_docs_for_code(code_path)
    
    // 2. Handle rename/move
    IF file_delta.status IS Renamed(old_path, new_path):
      old_citing = reverse_index.get_docs_for_code(old_path)
      FOR doc IN old_citing:
        record_obsolete_reference_finding(doc, old_path, new_path)
        citing_docs.insert(doc)
    
    // 3. Handle deletion
    IF file_delta.status IS Deleted:
      FOR doc IN citing_docs:
        record_deleted_code_finding(doc, code_path)

    // 4. Record review obligation
    FOR doc_id IN citing_docs:
      obligation = affected_docs.entry(doc_id).or_insert(ReviewObligation {
        doc_id,
        status: ReviewStatus::Pending,
        triggering_code_files: [],
        dependency_sources: [reverse_index.get_source(doc_id, code_path)]
      })
      obligation.triggering_code_files.push(code_path)

  // 5. Evaluate resolution state
  staged_or_changed_docs = { delta.path FOR delta IN changed_files WHERE delta.path.ends_with(".md") }
  review_receipts = load_review_receipts(workspace_root)

  FOR (doc_id, obligation) IN affected_docs:
    IF staged_or_changed_docs.contains(doc_id):
      obligation.status = ReviewStatus::Updated
    ELSE IF review_receipts.contains(doc_id):
      obligation.status = ReviewStatus::ReviewedUnchanged
    ELSE:
      obligation.status = ReviewStatus::Pending
```

### Performance Complexity
- Looking up `code_to_docs` is $O(1)$ per changed file.
- Total runtime for 50 changed files with 2,000 documents: $\le 5\text{ ms}$, comfortably within NFR-002 ($\le 50\text{ ms}$).

---

## 3. Research Question 3: PR/CI Review Receipts for `reviewed_unchanged`

### Context & Problem
In a Pull Request, code changes often don't alter documented behaviors (e.g., internal variable renames, bug fixes in private methods, test additions). Git has no native mechanism to record "this unchanged documentation note was verified by a human/agent". We need a reproducible, verifiable receipt without forcing artificial whitespace edits to the Markdown file.

### Evaluated Alternatives
1. **Option A: Touch the Markdown file with empty commit or comment**.
   - *Flaw*: Pollutes git log, invalidates hashes, causes spurious merge conflicts.
2. **Option B: Git Commit Trailer (`Reviewed-Docs: docs/arch.md`)**.
   - *Pros*: Native Git mechanism, completely immutable once committed, transparent in `git log`.
3. **Option C: Local/PR Review State File (`.context-vault/reviews.json`)**.
   - *Pros*: Easy to write in local developer environments and CI steps (`llm-wiki-core review resolve docs/arch.md`).
4. **Option D: CI Check / GitHub Review API**.
   - *Pros*: Integrated with GitHub UI.
   - *Cons*: Violates Invariant IV (Locality) if mandated.

### Decision
Support **Multi-Tier Receipt Ingestion**:
- **Tier 1 (CLI & Local Workflow)**: `.context-vault/reviews.json` maps commit SHA / branch to array of approved doc IDs. Can be written via `llm-wiki-core review resolve <doc_id>`.
- **Tier 2 (Git-Native Commit Trailer)**: Commit messages containing `Reviewed-Docs: <path>` or `Reviewed-Docs: all` automatically satisfy review obligations for that commit.
- **Tier 3 (IPC Parameter)**: In VS Code, the extension sends `doc_ids: ["docs/arch.md"]` (with backward-compatible alias `resolved_reviews`) during pre-commit candidate evaluation.

---

## 4. Research Question 4: Architectural Rules Engine vs. Semantic Drift

### Context & Problem
Section 9 of the specification mandates strict separation between:
- `change_detected` (code changed);
- `review_required` (documentation needs human/agent review);
- `rule_violation` (verifiable failure of an explicit machine-checkable rule).

### Rule Specification Schema in `.context-vault.yaml`
```yaml
architecture:
  rules:
    - id: "NO_DIRECT_AST_FROM_EXTENSION"
      type: "prohibited_dependency"
      source_pattern: "extension/src/**"
      forbidden_target: "crates/core/src/ast/**"
      severity: "error"
      explanation: "Extension host must interact with core engine exclusively through JSON-RPC"

    - id: "MANDATORY_ARCHITECTURE_DOC"
      type: "mandatory_documentation"
      code_pattern: "crates/core/src/evolution/**"
      required_doc: "docs/architecture/evolution.md"
      severity: "warning"
      explanation: "Core evolution engine changes require documentation in evolution.md"
```

### Deterministic Rule Checker
The rule engine evaluates AST/code citations against this explicit manifest.
- If a rule matches $\to$ emit `rule_violation` with exact rule ID and explanation.
- If no rule matches, but `code_dependencies` citations exist $\to$ emit `review_required`.
- If an unprompted semantic accuracy check is requested $\to$ classify as Epistemic Tier D (`not_verifiable`).

---

## 5. Research Question 5: Graphify Spike Isolation Architecture

### Context & Problem
We must evaluate Graphify and Understand Anything as potential sources of observed architecture without:
1. Making them runtime dependencies of `llm-wiki-core`;
2. Overwriting or replacing the canonical knowledge graph;
3. Compromising the zero-external-AI invariant.

### Decision
Implement `crates/core/examples/graphify_spike.rs`:
- Reads an exported Graphify graph JSON / NetworkX artifact.
- Compares its extracted nodes/edges against the canonical `NoteNode` and `NoteEdge` graph of a reference repository.
- Computes empirical metrics:
  - Overlap % (precision / recall against manual citations);
  - Extraction latency and RSS memory;
  - Handling of deletions and file renames.
- Generates Markdown comparison report to `target/graphify_evaluation_report.md`.
- Strictly segregated under `examples/` with zero production dependency linkages.
