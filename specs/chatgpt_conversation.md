# Review Extension UX

> **Source:** https://chatgpt.com/share/6aa4710d-2b38-83e9-8d47-01a5cf0c31b3 | **Date:** 2026-09-11 21:22:21 UTC | **Model:** `gpt-5-6`

### User

I need your help to review this extension that I am creating. Here is the constitution and specifciation from that:

I need to make it more clean user friendly, we have some buttons repeated, in the left side bar inside of the app, and also at the top, also inside of the 3 dots, I need an critical advisory to make it clean and very useful, like best in class

### Tool Output

The output of this plugin was redacted.

### Tool Output

The output of this plugin was redacted.

### Tool Output

The output of this plugin was redacted.

### Tool Output

The output of this plugin was redacted.

### Tool Output

The output of this plugin was redacted.

### ChatGPT

Yes. I’d be quite critical here: **the extension already has a strong technical foundation, but the current UI is exposing the architecture instead of exposing the user’s job.**

The biggest issue is not aesthetics. It is **information architecture and action hierarchy**.

Your constitution explicitly aims for progressive disclosure, context clarity, and user agency, while the feature spec currently puts several of those capabilities into parallel UI surfaces. fileciteturn0file0 fileciteturn0file1

# My overall verdict

**Technical product:** 9/10  
**Current UX:** 6/10  
**Potential:** 9.5/10

The graph itself is visually impressive.

But right now it feels like:

> **"A developer tool exposing all its capabilities."**

It should feel like:

> **"A calm control center for understanding my project's knowledge and AI context."**

That distinction is important.

---

# 1. The biggest problem: too many places to do the same thing

Looking at the screenshot, I see roughly three competing control layers:

### Left sidebar

- Open Knowledge Graph
- Set Central Hub
- Rescan Vault
- Fix Broken Links
- Install Git Hooks

### Top-right toolbar

- Token count
- Search
- Filters
- Settings
- appearance/view controls
- fullscreen
- `...`

### VS Code native UI

- Activity Bar
- editor tabs
- status bar
- VS Code menus

The result is **action duplication + visual competition**.

The user has to learn:

> "Where does this action live?"

That's exactly the cognitive load your product philosophy says we should eliminate.

---

# 2. I would completely rethink the left panel

Currently it is behaving like a combination of:

**Dashboard + navigation + action toolbar + diagnostics + settings.**

That's too much.

The left panel should answer only:

> **"What is important about my knowledge base right now?"**

I'd change it to something closer to this:

```text
┌─────────────────────────────┐
│  LLM Wiki                   │
│  ● Synced                   │
│                             │
│  KNOWLEDGE BASE              │
│                             │
│  923 notes                   │
│  2,875 connections           │
│                             │
│  ⚠ 343 broken links          │
│                             │
│  AI CONTEXT                  │
│                             │
│  4,883 tokens active         │
│  ███████░░░ 8%              │
│                             │
│  Central hub                 │
│  Workspace Wiki Master      │
│                             │
│  ─────────────────────────  │
│                             │
│  [ Review issues ]           │
│                             │
└─────────────────────────────┘
```

That's much more powerful.

Notice what disappeared:

- Set Central Hub
- Rescan Vault
- Install Git Hooks
- Open Knowledge Graph

Not because those capabilities disappear.

They simply stop being **primary UI concepts**.

---

# 3. "Open Knowledge Graph" is especially wrong in the current state

You are already inside the Knowledge Graph.

So this:

> **Open Knowledge Graph**

at the top of the sidebar creates a weird mental model.

It makes sense if that sidebar exists globally and the graph is another destination.

But once the user is inside the graph, it becomes:

> "Why am I being asked to open the thing I'm already looking at?"

### Better

The top of the panel should identify the current destination:

**Knowledge Graph**

and possibly provide:

`← Back` only if there is somewhere meaningful to return to.

Otherwise, no button.

---

# 4. Don't expose "Set Central Hub" as a primary action

This is one of the most interesting capabilities technically, but it shouldn't be a prominent button.

The user's mental model shouldn't be:

> "I need to configure a hub."

It should be:

> **"This is my project's knowledge center."**

You already have automatic hub discovery in the architecture. The system should do the work.

So:

### Default experience

```text
Central hub
Workspace Wiki Master
[Change]
```

"Change" opens a contextual picker.

Even better:

> **Central hub · Auto**

and clicking it gives:

```text
Central hub

● Automatically detected
  Workspace Wiki Master

○ Choose manually

○ Add another hub
```

That's much more Apple-like.

**System decides → user confirms**, exactly matching the product philosophy in your supplied PM guidance.

---

# 5. "Rescan Vault" should almost never be visible

This is an implementation concept.

Users don't fundamentally want:

> Rescan Vault

They want:

> **"Make sure my graph is up to date."**

And your architecture already deliberately avoids continuous filesystem watchers and uses Git-driven synchronization. fileciteturn0file0

Therefore:

### Normal state

```text
● Synced · 12s ago
```

### Something is happening

```text
◌ Updating knowledge...
```

### Something is wrong

```text
⚠ Knowledge graph needs attention
[Review]
```

Only put **Rescan** in an advanced menu.

I'd actually rename it:

> **Rebuild index**

if the operation really is a full rebuild.

"Rescan Vault" exposes your internal implementation model.

---

# 6. "Fix Broken Links" is also too implementation-oriented

This is actually a great feature.

But the CTA shouldn't make the user understand the mechanism.

Instead of:

> Fix Broken Links

I would surface the **problem**:

```text
343
Broken links

[Review issues]
```

Then:

```text
Knowledge issues

343 broken links

Missing files
    217

Missing anchors
     86

Unresolved wikilinks
     40

[Review all]
```

And once they select one:

```text
docs/architecture/foo.md
→ missing: system-design.md

Line 42

[Open]
```

This is much more actionable.

Your spec already defines the machinery to locate broken links and jump to file/line coordinates. fileciteturn0file1

The UI should hide that machinery until the user needs it.

---

# 7. Git Hooks absolutely should NOT be a prominent button

This is probably my strongest recommendation.

You currently have:

> Install Git Hooks

next to the core knowledge product.

But Git hooks are **infrastructure configuration**.

They are not part of the primary user journey.

Imagine Apple putting:

> Configure background synchronization daemon

on the main screen.

No.

Move it to:

**Settings → Integrations → Git**

And ideally:

```text
Git synchronization

● Automatically synchronized

prek
Last sync: 14s ago

[Configure]
```

If Git isn't configured:

```text
Git synchronization

○ Not configured

Automatically keep your knowledge graph
up to date when documentation changes.

[Enable]
```

That's a much better experience.

---

# 8. The graph needs to become the unquestionable hero

Right now the graph is visually impressive, but the UI around it competes with it.

I would make the product hierarchy:

```text
              KNOWLEDGE GRAPH
                     ↓
       ┌──────────────────────────┐
       │                          │
       │                          │
       │         GRAPH            │
       │                          │
       │                          │
       └──────────────────────────┘
              ↑
        contextual controls
```

Everything else should support interpretation of the graph.

Not compete with it.

---

# 9. The top toolbar should be dramatically simplified

Currently you have a row of icons:

```text
⚡ 4,883 tok | 🔍 | sliders | ⚙ | ◉ | AI | fullscreen | ...
```

This is classic developer-tool accumulation.

I'd reduce it to approximately:

```text
[ 4.9k active ]   [ Search ] [ Filter ] [ ⋯ ]
```

That's it.

Potentially:

```text
[4.9k active]   [⌕ Search]   [Filter]   [⋯]
```

### What belongs here?

Only things that manipulate **the current graph view**:

- Search
- Filter
- View options
- Zoom/reset
- maybe focus
- token/context indicator

### What does NOT belong here?

- Git hooks
- hub configuration
- indexing
- configuration
- diagnostics
- cluster management

Those are different concepts.

---

# 10. The token indicator is good — but currently underspecified

This:

> `4,883 tok`

is potentially your **killer product feature**.

But right now it looks like a random telemetry counter.

It should communicate meaning.

Instead:

```text
AI Context

4.9k active
──────────────
48.3k reachable
```

Or compactly:

```text
AI Context · 4.9k
```

Clicking opens:

```text
AI Context

Active context
4,883 tokens

Reachable context
48,300 tokens

────────────────

Healthy

Your active context is well
below the 60k warning threshold.

[Explore context]
```

Your specification already has the distinction between active harness tokens and accessible/on-demand context. fileciteturn0file1

**That distinction should become a first-class product concept.**

---

# 11. I would introduce one major concept: "Context"

I think your current product is slightly too graph-centric.

The graph is the **mechanism**.

The user's actual value proposition is closer to:

> **Understand what your AI can see, why it can see it, and how much context it consumes.**

That is much stronger.

I'd therefore structure the product around three concepts:

```text
KNOWLEDGE
   ↓
CONTEXT
   ↓
HEALTH
```

Or:

### Knowledge Graph

Understand relationships.

### AI Context

Understand what enters the AI context.

### Issues

Understand what needs attention.

This is far cleaner than exposing:

- graph
- hub
- tokens
- broken links
- Git hooks
- clusters
- rescan
- settings

as peers.

---

# 12. The graph should have a "Focus" state

Your graph currently shows everything.

That's dangerous.

923 nodes is already visually overwhelming.

And your architecture explicitly introduces hop levels:

- Level 0
- Level 1
- Level 2
- Level 3+
- orphan nodes. fileciteturn0file0

Use that to create a **Focus Mode**.

Default:

```text
Focus: Workspace Wiki Master

● Hub
● Direct references
○ Deep references
· Everything else
```

The graph becomes dramatically easier to understand.

Potential controls:

```text
Focus

● Hub
○ 1 hop
○ 2 hops
○ All
```

But don't put all four as permanent buttons.

Use one dropdown:

> **Focus · 2 hops**

---

# 13. Your graph needs semantic visual hierarchy, not just physics

The current graph looks somewhat like:

> "Here is a giant network."

It needs to communicate:

> **"Here is the structure of your knowledge."**

I'd establish a stronger hierarchy:

### Level 0

Large + strong label + full opacity.

### Level 1

Medium + readable.

### Level 2

Small + subdued.

### Level 3+

Very subtle.

### Orphans

Not necessarily permanently visible.

Maybe:

```text
923 notes
  781 connected
  142 orphaned
```

Then:

> `Show orphans`

instead of throwing them into the primary visualization.

Your opacity law is technically well defined in the constitution, so this is a perfect opportunity to make the visual hierarchy do real product work rather than merely styling nodes. fileciteturn0file0

---

# 14. Cluster colors: don't expose this in the main interface

This is another place where I would aggressively apply progressive disclosure.

The feature is powerful:

```text
#architecture → purple
#security → red
docs/tests/** → green
```

But most users don't need to configure it every day.

So don't have:

> Cluster Groups

sitting permanently in the primary interface.

Instead:

**⋯ → Appearance → Cluster groups**

or:

**Settings → Graph → Color groups**

Then the graph simply displays the result.

---

# 15. The three-dot menu should become your "Advanced" layer

Your `...` menu is currently the perfect place to hide complexity.

I'd make it something like:

```text
More

Graph
  Reset view
  Recenter hub
  Export graph

Knowledge
  Rebuild index
  Review broken links

Configuration
  Central hub
  Color groups

Integrations
  Git synchronization

Developer
  Diagnostics
  Copy graph state
```

This gives you an extremely clean primary interface while retaining the power expected from a developer tool.

---

# 16. I would remove most text buttons from the graph screen

The current left panel has:

```text
Set Central Hub
Rescan Vault
Fix Broken Links
Install Git Hooks
```

Four buttons.

That's a smell.

A world-class product should ideally have **one primary action**, or zero if the product is primarily exploratory.

I'd replace them with a single status-oriented area:

```text
ATTENTION

343 broken links
[Review]

────────────────

SYNC

● Up to date

────────────────

HUB

Workspace Wiki Master
[Change]
```

Notice:

**actions emerge from states.**

That's much more natural.

---

# 17. I'd change the information architecture to this

My recommended structure:

```text
LLM WIKI
│
├── Knowledge Graph
│
├── AI Context
│
└── Issues
```

Then:

```text
Knowledge Graph
│
├── Graph
├── Search
├── Filters
└── Focus
```

```text
AI Context
│
├── Active context
├── Reachable context
├── Context paths
└── Token analysis
```

```text
Issues
│
├── Broken links
├── Orphans
├── Unresolved references
└── Sync problems
```

And:

```text
Settings
│
├── Hub
├── Color groups
├── Git synchronization
├── Indexing
└── Advanced
```

That's a much stronger product architecture.

---

# 18. One thing I would NOT do

I would **not** turn this into a huge Obsidian clone.

That is the trap.

You don't want:

```text
Files
Folders
Tags
Backlinks
Graph
Search
Properties
Templates
Plugins
Git
AI
Settings
...
```

You have a much sharper proposition.

The product should be:

> **The knowledge/context layer for AI-assisted development inside VS Code.**

That's differentiated.

The graph is not the product.

**Understanding and controlling AI context is the product.**

---

# 19. My proposed screen

If I were designing v2 of this exact screen, I'd aim for something like:

```text
┌───────────────────────────────────────────────────────────────┐
│ Knowledge Graph                    4.9k active   Search Filter ⋯│
├───────────────┬───────────────────────────────────────────────┤
│               │                                               │
│ KNOWLEDGE     │                                               │
│               │                                               │
│ 923 notes     │                                               │
│ 2,875 links   │                KNOWLEDGE GRAPH                 │
│               │                                               │
│ ⚠ 343 issues  │                     ●                         │
│               │                  ●  │  ●                      │
│               │               ●─────●──────●                  │
│ AI CONTEXT    │                  ╲  │  ╱                      │
│               │                   ╲ │ ╱                       │
│ 4.9k active   │                    ●                          │
│ 48.3k reachable│                                               │
│               │                                               │
│ HUB           │                                               │
│ Workspace Wiki│                                               │
│ Master        │                                               │
│               │                                               │
│               │                                               │
│ [Review issues]│                                              │
│               │                                               │
│ ● Synced      │                                               │
└───────────────┴───────────────────────────────────────────────┘
```

Much calmer.

---

# 20. And then use contextual UI

This is the key.

When nothing is selected:

> **overview**

When a node is selected:

```text
┌──────────────────────┐
│ Workspace Wiki Master│
│                      │
│ Central hub          │
│ 4,883 tokens         │
│ 54 incoming          │
│ 12 outgoing          │
│                      │
│ Context              │
│ Level 0              │
│                      │
│ [Open file]           │
│ [Set as hub]          │
└──────────────────────┘
```

When a broken edge is selected:

```text
┌──────────────────────┐
│ Broken reference     │
│                      │
│ architecture.md      │
│ → missing-file.md    │
│                      │
│ Line 42              │
│                      │
│ [Open source]        │
│ [Fix reference]      │
└──────────────────────┘
```

When the user clicks the token indicator:

```text
┌──────────────────────┐
│ AI Context           │
│                      │
│ Active               │
│ 4,883 tokens         │
│                      │
│ Reachable            │
│ 48,300 tokens        │
│                      │
│ ───────────────────  │
│ Healthy              │
│                      │
│ [Explore context]    │
└──────────────────────┘
```

**The UI changes according to intent.**

That's the big upgrade.

---

# 21. The deeper product opportunity

There is something much more interesting hiding inside your specification.

You currently describe:

> Graph + tokens + hubs + links + clusters.

I would frame the product as:

## **"AI Context Observatory"**

The graph is the visualization layer.

The actual system tells the developer:

> **What knowledge is connected to my AI?**

> **Why is it connected?**

> **How deep is the dependency?**

> **How expensive is it?**

> **What's broken?**

> **What should I fix?**

That's a very compelling developer product.

And it aligns extremely well with the architecture you've already built: organic hub discovery, hop-based context tiers, token economics, broken-link detection, and incremental synchronization. fileciteturn0file1

---

# 22. My priority list

I would **not rewrite the whole UI**.

I'd do this in order:

### P0 — Remove duplication

Remove/reposition:

- Open Knowledge Graph
- Set Central Hub button
- Rescan Vault button
- Install Git Hooks button
- duplicate top/sidebar controls

### P0 — Establish hierarchy

Make:

**Graph → Context → Issues**

the core product hierarchy.

### P0 — Simplify toolbar

Keep only:

**Token status + Search + Filter + More**

### P1 — Convert actions into states

Instead of:

> Fix Broken Links

show:

> **343 broken links → Review**

Instead of:

> Rescan Vault

show:

> **● Synced**

Instead of:

> Set Central Hub

show:

> **Central hub: Workspace Wiki Master**

### P1 — Contextual inspector

Clicking a node/edge opens a contextual information panel.

### P1 — Focus mode

Default graph should emphasize:

**Hub → 1 hop → 2 hops**

rather than rendering the entire 923-node universe equally.

### P2 — Advanced configuration

Move:

- color groups
- Git hooks
- indexing
- hub rules
- diagnostics

into Settings / More.

---

# 23. The rule I'd use going forward

For every button currently in the UI, ask:

> **"Does the user come here specifically wanting to perform this action?"**

If no → hide it.

Then ask:

> **"Can the system detect when this action becomes relevant?"**

If yes → **show the action contextually.**

That is the biggest difference between a powerful developer tool and a **best-in-class developer product**.

Your architecture is already sophisticated enough. The next step isn't adding capabilities.

It's **hiding 70% of the capabilities until they're needed.**

And honestly, **I'd make that the next product/design milestone before adding more functionality.** The current constitution/spec already gives you enough capability to make a genuinely differentiated product. fileciteturn0file0 fileciteturn0file1

If I were taking this to the next level, the next artifact I'd produce would be a **complete v2 UX specification for this screen**: exact sidebar hierarchy, toolbar, graph states, node-selection behavior, menus, empty/error states, and the interaction rules for each component—essentially a design-system-level blueprint that you can hand directly to the implementation.

### User

good, add also on that purpose, an way to provide a blocklist, that is like a way to simply provide that some files I dont want to see in the graph and keep being annoyed to be orphan, because they could be unrelated at all, README from libraries, or folders from packages, submoules, so not only a file, but a folder at all also

### ChatGPT

Yes — and I think this is **more than a nice-to-have**. It closes an important UX gap in the current graph model.

The graph should distinguish:

> **"This file is disconnected because it is an orphan"**

from:

> **"This file is intentionally outside my knowledge graph."**

Those are fundamentally different states.

Your current specification says disconnected nodes become `orphan` nodes, but it doesn't yet define an explicit exclusion concept. fileciteturn0file1

## 1. Add a first-class "Excluded" concept

I would introduce:

### `excluded`

A file or directory intentionally excluded from the knowledge graph by the user or by default workspace rules.

It should **not**:

- appear as an orphan
- affect orphan counts
- clutter the graph
- participate in graph centrality
- consume graph visualization space
- trigger "broken link" noise merely because it isn't represented

But there's an important nuance:

**Excluded files should still exist in the filesystem and remain valid link targets.**

If:

```text
architecture.md
    ↓
packages/foo/README.md
```

and `packages/foo/**` is excluded, the system shouldn't tell the user:

> ❌ Broken link

The link resolves correctly; the target is simply outside the graph.

That's a very important distinction.

---

# 2. Don't call it "Blocklist" in the UI

I would use:

> **Excluded paths**

or

> **Ignored paths**

I prefer **Excluded paths**.

"Blocklist" sounds security-oriented and implies something is being actively prevented.

The user is really saying:

> "This isn't part of my knowledge graph."

That's an exclusion.

---

# 3. Support both files AND folders

This should be glob-based rather than a list of individual files.

For example:

```yaml
exclude:
  - ".git/**"
  - "node_modules/**"
  - "target/**"
  - "dist/**"
  - "packages/**/README.md"
  - "vendor/**"
```

But I'd actually separate your existing **technical scan exclusions** from your new **knowledge graph exclusions**.

That's an important architectural/product distinction.

---

# 4. Two different kinds of exclusion

Your current spec already has:

```yaml
exclude:
  - "node_modules/**"
  - ".git/**"
  - "target/**"
```

But those are really:

> **Do not scan these things.**

The new feature is:

> **Don't represent these things in my knowledge graph.**

Those shouldn't necessarily be the same.

I'd change the configuration model to:

```yaml
include:
  - "**/*.md"

scan:
  exclude:
    - ".git/**"
    - "node_modules/**"
    - "target/**"
    - "dist/**"

graph:
  exclude:
    - "**/README.md"
    - "packages/**/docs/**"
    - "vendor/**"
```

### Why this matters

Imagine:

```text
docs/api.md
   ↓
packages/foo/README.md
```

If `packages/foo/README.md` is **scanned but graph-excluded**, the engine can still understand:

> "This is a valid target, but the user doesn't want it represented."

That's much better than treating it as nonexistent.

---

# 5. The UI should make this ridiculously easy

This should **not** require editing YAML.

Imagine the user sees an annoying orphan:

```text
README.md
Orphan
```

They right-click:

```text
README.md

Open
Reveal in Explorer
───────────────
Exclude this file
Exclude this folder
───────────────
Copy path
```

That's the experience I'd prioritize.

### If they choose "Exclude this folder"

If the file is:

```text
packages/foo/README.md
```

the product could intelligently offer:

```text
Exclude

README.md
packages/foo/**
packages/**/README.md

[Cancel] [Exclude]
```

This is excellent because you're helping the user express **intent**, not forcing them to understand glob syntax.

---

# 6. Even better: contextual exclusion

When a user selects a node:

```text
┌─────────────────────────┐
│ README.md               │
│                         │
│ packages/foo/README.md  │
│ 812 tokens              │
│ Orphan                  │
│                         │
│ [Open file]             │
│                         │
│ ⋯ More                  │
└─────────────────────────┘
```

Then:

```text
More

Exclude from graph
  • This file
  • This folder
  • Matching files...
```

This is much better than exposing a permanent:

> Blocklist Manager

button.

---

# 7. Add an "Excluded paths" section to Settings

This is where users can manage the rules globally.

```text
Settings
────────────────────

Knowledge graph

Central hub
  Workspace Wiki Master

Excluded paths

  node_modules/**
  packages/**/README.md
  vendor/**

                         + Add path
```

Clicking a rule:

```text
packages/**/README.md

Matches 84 files

Preview
  ✓ packages/auth/README.md
  ✓ packages/core/README.md
  ✓ packages/api/README.md
  ...

[Remove rule]
```

That's much more trustworthy than blindly saving a glob.

---

# 8. Give users a one-click solution for noisy folders

Your example is exactly the kind of thing the product should detect.

For example:

```text
Graph health

923 notes
2,875 links

142 orphan notes

Common patterns detected

84 README files
38 package documentation files
12 generated notes
8 vendor files

These may not belong in your knowledge graph.

[Review suggestions]
```

Then:

```text
Suggested exclusions

☑ node_modules/**
☑ packages/**/README.md
☐ vendor/**
☐ generated/**

[Apply selected]
```

This would be a **very strong UX feature**.

The system is essentially saying:

> "I noticed why your graph is noisy. Want me to clean it up?"

That's far better than making the user discover glob patterns.

---

# 9. But don't automatically exclude things without consent

I'd be careful here.

A README can be extremely important.

For example:

```text
README.md
```

could be:

- library documentation
- the project's primary entry point
- architecture documentation
- onboarding instructions
- AI agent instructions

So I would **suggest exclusions**, not silently apply them.

Good:

> "84 README files appear disconnected. Exclude them?"

Bad:

> "We decided README files aren't useful."

---

# 10. Orphans should become more useful after this

Once exclusion exists, the orphan metric becomes much more meaningful.

Instead of:

```text
923 Notes
142 Orphans
```

show:

```text
923 Notes
37 Orphans
105 Excluded
```

That's a much more truthful representation.

Potentially:

```text
Knowledge base

781 connected
37 orphaned
105 excluded
```

Now the user can understand the state of the system.

---

# 11. I would also add "Show excluded" — but only as an optional diagnostic mode

The default graph:

```text
● Connected
○ Orphan
```

Excluded:

> **not rendered**

But if the user wants to inspect them:

```text
Filter

☑ Connected
☑ Orphans
☐ Excluded
☐ Broken links
```

If enabled, excluded nodes could appear extremely muted/ghosted.

Something like:

```text
○ normal node
· orphan
⋯ excluded
```

But **never make excluded nodes part of the normal graph**.

Otherwise we've defeated the entire purpose.

---

# 12. Important: excluded ≠ deleted

This needs to be explicit in the product model.

If a user excludes:

```text
packages/foo/**
```

the system should communicate:

> **Excluded from knowledge graph**

not:

> Ignored

because "ignored" can imply the files aren't being processed at all.

I'd use three concepts:

| State | Meaning |
|---|---|
| **Connected** | Part of graph and reachable |
| **Orphan** | Part of graph but disconnected |
| **Excluded** | Intentionally outside graph |

That's clean.

---

# 13. I'd change your graph state schema

Add:

```json
{
  "id": "packages/foo/README.md",
  "status": "excluded",
  "excluded_by": {
    "type": "glob",
    "pattern": "packages/**/README.md"
  }
}
```

But I actually wouldn't send excluded nodes to the normal graph renderer at all.

The core could maintain exclusion metadata separately:

```json
{
  "graph_summary": {
    "total_notes": 923,
    "connected_notes": 781,
    "orphan_notes": 37,
    "excluded_notes": 105
  }
}
```

That keeps the rendering payload lean.

---

# 14. Add these requirements to the specification

I'd add a new section:

## Graph Exclusion & Noise Management

### FR-046 — User-defined graph exclusions

The system MUST allow users to exclude individual Markdown files or filesystem path patterns from graph visualization.

### FR-047 — Glob-based exclusion

The system MUST support glob patterns including:

```text
path/to/file.md
folder/**
packages/**/README.md
**/vendor/**
```

### FR-048 — Excluded nodes are not orphans

Files matching graph exclusion rules MUST NOT be classified as orphan nodes and MUST NOT contribute to orphan counts.

### FR-049 — Excluded targets remain resolvable

A valid Markdown or wikilink targeting an excluded file MUST NOT be classified as a broken link solely because the target is excluded from the graph.

### FR-050 — Contextual exclusion actions

The Webview MUST provide contextual actions allowing users to:

- Exclude file
- Exclude containing folder
- Exclude matching pattern

without requiring manual YAML editing.

### FR-051 — Exclusion management

The configuration UI MUST allow users to review, edit, remove, and preview exclusion patterns.

### FR-052 — Exclusion preview

Before applying a glob exclusion, the system SHOULD display the number and representative paths of files affected.

### FR-053 — Exclusion transparency

The UI MUST distinguish:

**Connected → Orphan → Excluded**

and MUST never silently convert an excluded file into a broken link or orphan.

---

# 15. I'd also modify the configuration

Instead of:

```yaml
include:
  - "**/*.md"

exclude:
  - "node_modules/**"
  - ".git/**"
  - "target/**"
```

I'd evolve it to:

```yaml
include:
  - "**/*.md"

scan:
  exclude:
    - ".git/**"
    - "node_modules/**"
    - "target/**"
    - "dist/**"

graph:
  exclude:
    - "**/README.md"
    - "vendor/**"
    - "packages/**/docs/**"
```

And importantly:

### `scan.exclude`

Means:

> **Don't process this.**

### `graph.exclude`

Means:

> **Process it if necessary, but don't show it in my knowledge graph.**

That separation will save you architectural headaches later.

---

# 16. One more thing I'd add: Git-aware defaults

Your product is developer-focused, so there are obvious classes of noise:

```text
.git/**
node_modules/**
target/**
dist/**
build/**
coverage/**
vendor/**
```

Some of these should probably be **default scan exclusions**, not user configuration.

But I'd be more conservative with:

```text
**/README.md
```

because README files can absolutely be valuable knowledge.

So I'd have:

### Safe defaults

```text
.git/**
node_modules/**
target/**
dist/**
build/**
coverage/**
```

### Suggested exclusions

```text
**/README.md
**/CHANGELOG.md
**/LICENSE.md
vendor/**
```

The user remains in control.

---

# 17. This actually improves the core product positioning

This addition reinforces something I said previously:

> **The product should manage knowledge, not merely visualize files.**

A good knowledge system needs to understand:

```text
Relevant
     ↓
Connected
     ↓
Orphan
     ↓
Intentionally excluded
```

The graph isn't supposed to represent **everything in the filesystem**.

It's supposed to represent **the knowledge that matters**.

That should become a fundamental product principle:

> **The filesystem is the source. The knowledge graph is a curated representation of what matters.**

And that is a much stronger product than "Obsidian-style graph inside VS Code."

I'd therefore make **Noise Management / Excluded Paths a P1 feature**, not a later settings enhancement. It directly addresses one of the biggest failure modes of an automatically generated knowledge graph: **the system is technically correct but practically noisy.**
