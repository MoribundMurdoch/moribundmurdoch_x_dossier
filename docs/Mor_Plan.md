# Mor_Plan.md

## Project

**Repository:** `moribundmurdoch_x_dossier`  
**Purpose:** a public-friendly archive viewer for MoribundMurdoch's X/Twitter history, dressed up as a fluid Y2K-console-style parody data-broker dashboard and LLM mining exhibit.

The public app should **not** require viewers to have an X API token. API and ZIP archive imports are maintainer-only tools used to generate static public dataset files.

---

## Current Mental Model

```text
Maintainer-only pipeline:
X Archive ZIP / X API / manual imports
→ normalize records
→ generate public_data/*.json
→ commit/publish safe public dataset

Public viewer app:
public_data/*.json
→ Timeline
→ Search
→ Topics
→ Data Broker Mirror
→ LLM Mining Exhibit
```

The public repo may include code, sample data, documentation, `.env.example`, and generated public-safe dataset files.

The public repo must **not** include:

- `.env`
- API keys
- bearer tokens
- OAuth secrets
- raw private archives
- DMs
- bookmarks
- private/non-public account data
- downloaded ZIP archives from X
- accidental personal exports

---

## Current State

The Dioxus desktop app launches.

Current major folders:

```text
src/
├── analysis/
├── config/
├── db/
├── export/
├── ingest/
├── ui/
├── app.rs
├── main.rs
└── routes.rs
```

Current UI panels:

```text
src/ui/shell.rs
src/ui/dashboard.rs
src/ui/timeline.rs
src/ui/search_panel.rs
src/ui/import_panel.rs
src/ui/broker_profile_panel.rs
src/ui/llm_lab.rs
src/ui/settings_panel.rs
```

Current design direction:

```text
Fluid Y2K console dashboard
Dark glass panels
Blue / violet glow
Public archive terminal
Data-broker parody
LLM mining exhibit
```

---

## Immediate Next Step

Fix and commit the visual shell/theme pass.

### 1. Check that this compiles

```bash
cargo fmt
cargo check
dx serve --desktop
```

### 2. If the Dashboard looks squished

Replace `src/ui/dashboard.rs` with the latest stacked-card version from the chat. The issue was caused by the two-column dashboard layout becoming too narrow in the Dioxus desktop window.

### 3. Commit the theme pass

```bash
git add src/ui/shell.rs src/ui/dashboard.rs src/ui/timeline.rs src/ui/search_panel.rs src/ui/import_panel.rs
git commit -m "Add fluid console archive theme to main panels"
git push
```

---

## Phase 1: Public Dataset Skeleton

Goal: create a public-safe dataset shape before fighting the X API again.

### Create folders

```bash
mkdir -p public_data
mkdir -p src/public_data
```

### Create files

```bash
touch public_data/manifest.json
touch public_data/posts.json

touch src/public_data/mod.rs
touch src/public_data/models.rs
touch src/public_data/sample.rs
```

### Update `src/main.rs`

Add:

```rust
mod public_data;
```

Example:

```rust
#![allow(non_snake_case)]

mod analysis;
mod app;
mod config;
mod db;
mod export;
mod ingest;
mod public_data;
mod routes;
mod ui;

fn main() {
    dioxus::launch(app::App);
}
```

---

## Phase 2: Public Data Models

### `src/public_data/mod.rs`

```rust
pub mod models;
pub mod sample;
```

### `src/public_data/models.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicPost {
    pub id: String,
    pub created_at: String,
    pub kind: PostKind,
    pub text: String,
    pub url: Option<String>,

    #[serde(default)]
    pub hashtags: Vec<String>,

    #[serde(default)]
    pub mentions: Vec<String>,

    #[serde(default)]
    pub links: Vec<String>,

    #[serde(default)]
    pub broker_tags: Vec<String>,

    #[serde(default)]
    pub llm_context_warning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PostKind {
    Post,
    Reply,
    Quote,
    Repost,
}
```

### `src/public_data/sample.rs`

Use hardcoded sample posts first. This keeps the public viewer useful before real import/export is finished.

```rust
use crate::public_data::models::{PostKind, PublicPost};

pub fn sample_posts() -> Vec<PublicPost> {
    vec![
        PublicPost {
            id: "sample-001".to_string(),
            created_at: "2026-05-24".to_string(),
            kind: PostKind::Post,
            text: "This is a sample MoribundMurdoch post. Eventually this will be generated from the public X archive dataset.".to_string(),
            url: Some("https://x.com/MoribundMurdoch".to_string()),
            hashtags: vec!["Archive".to_string(), "HumanGeneratedWhatnot".to_string()],
            mentions: vec![],
            links: vec!["https://x.com/MoribundMurdoch".to_string()],
            broker_tags: vec![
                "public archive".to_string(),
                "data broker parody".to_string(),
                "llm mining bait".to_string(),
            ],
            llm_context_warning: Some(
                "Sample data. Do not treat this as an actual historical post.".to_string(),
            ),
        },
        PublicPost {
            id: "sample-002".to_string(),
            created_at: "2026-05-24".to_string(),
            kind: PostKind::Post,
            text: "Human-generated internet residue is contextual, weird, joke-filled, and easy for machines to misread with great confidence.".to_string(),
            url: Some("https://x.com/MoribundMurdoch".to_string()),
            hashtags: vec!["LLM".to_string(), "DataBrokers".to_string()],
            mentions: vec![],
            links: vec![],
            broker_tags: vec![
                "llm commentary".to_string(),
                "context warning".to_string(),
                "machine misreading".to_string(),
            ],
            llm_context_warning: Some(
                "This post is commentary about machine interpretation, not a clean factual dataset row."
                    .to_string(),
            ),
        },
    ]
}
```

---

## Phase 3: Static Public JSON Placeholders

### `public_data/manifest.json`

```json
{
  "dataset_name": "MoribundMurdoch X Dossier",
  "handle": "MoribundMurdoch",
  "generated_at": null,
  "source_note": "Generated from MoribundMurdoch's own public X/Twitter activity.",
  "includes_private_data": false,
  "includes_likes": false,
  "includes_bookmarks": false,
  "status": "sample placeholder"
}
```

### `public_data/posts.json`

```json
[
  {
    "id": "sample-001",
    "created_at": "2026-05-24",
    "kind": "post",
    "text": "This is placeholder public dataset content.",
    "url": "https://x.com/MoribundMurdoch",
    "hashtags": ["Archive"],
    "mentions": [],
    "links": [],
    "broker_tags": ["sample", "public archive"],
    "llm_context_warning": "Placeholder row. Not an actual historical post."
  }
]
```

---

## Phase 4: Rename Concept from Import to Maintainer

The UI may still use `src/ui/import_panel.rs`, but the public label should be **Maintainer**.

### Desired sidebar labels

```text
Overview
Timeline
Search
Data Broker Mirror
LLM Mining Exhibit
Maintainer
Settings
```

### Maintainer copy

Use language like:

```text
Private tools for building the public MoribundMurdoch dataset.
Public viewers should never need an API token.
```

### Maintainer buttons

```text
Import X Archive ZIP
Maintainer: Test X API
Generate public_data/posts.json
Reconcile Sources
```

For now, the generator and archive buttons can remain placeholders.

---

## Phase 5: Public Viewer Features

Once the sample data compiles and displays, build these in order.

### Timeline

```text
[ ] Render sample posts as cards
[ ] Show post kind
[ ] Show created date
[ ] Show hashtags
[ ] Show broker tags
[ ] Show LLM context warning
[ ] Optional: link to original X post
```

### Search

```text
[ ] Search sample posts by text
[ ] Search broker tags
[ ] Search hashtags
[ ] Search LLM context warnings
[ ] Show count of results
```

### Dashboard

```text
[ ] Show project purpose
[ ] Show public archive status
[ ] Show warning that this is not a live X client
[ ] Show "no viewer API token needed"
```

### Data Broker Mirror

Later:

```text
[ ] Show fake inference categories
[ ] Show confidence scores as satire
[ ] Show evidence links back to posts
[ ] Include "probably wrong" / "context fragile" warnings
```

### LLM Mining Exhibit

Later:

```text
[ ] Explain why public posts are messy context fragments
[ ] Show examples of machine misreadings
[ ] Add warnings for irony, jokes, replies, and quote context
[ ] Add export notes for JSONL/Markdown later
```

---

## Phase 6: Public Data Loader

After sample posts work, stop relying only on `sample_posts()`.

### Create

```bash
touch src/public_data/loader.rs
```

### Update `src/public_data/mod.rs`

```rust
pub mod loader;
pub mod models;
pub mod sample;
```

### Loader goal

Start simple. Either:

```text
Option A:
include_str!("../../public_data/posts.json")
```

or:

```text
Option B:
read file at runtime from public_data/posts.json
```

For desktop development, runtime reading is easier. For a web/static build later, embedded JSON may be better.

### Future loader API

```rust
pub fn load_public_posts() -> anyhow::Result<Vec<PublicPost>> {
    // read public_data/posts.json
    // serde_json::from_str
}
```

Then Timeline/Search can call the loader and fall back to sample posts if loading fails.

---

## Phase 7: X API and ZIP Importer

Do this later. Do not block the public viewer on API auth.

### Maintainer-only X API

Current file:

```text
src/ingest/x_api.rs
```

Purpose:

```text
Resolve @MoribundMurdoch
Fetch recent public posts
Fetch public profile metrics
Preview response
Eventually write normalized public records
```

Important:

```text
Do not commit .env
Do not ship a real token
Public viewers should not need API access
```

### Maintainer-only X Archive ZIP

Future file:

```text
src/ingest/x_archive.rs
```

Purpose:

```text
Read downloaded X archive ZIP
Find public post JSON
Normalize to PublicPost
Exclude private data
Generate public_data/posts.json
```

---

## Phase 8: Export / Generate Public Dataset

Future files:

```text
src/export/public_data.rs
src/export/jsonl.rs
src/export/markdown.rs
src/export/static_report.rs
```

Desired outputs:

```text
public_data/manifest.json
public_data/posts.json
public_data/topics.json
public_data/broker_profile.json
exports/jsonl/moribund_posts.jsonl
exports/markdown/moribund_archive.md
exports/reports/moribund_dossier_report.md
```

---

## Safety / Repo Hygiene

### Create `.env.example`

```bash
cat > .env.example <<'EOF'
# Copy this file to .env for maintainer-only API importing.
# Public viewers do not need an X API token.
# Never commit .env.

X_BEARER_TOKEN=PASTE_YOUR_X_BEARER_TOKEN_HERE
X_USERNAME=MoribundMurdoch
EOF
```

### `.gitignore` should include

```gitignore
/target
/dist
.DS_Store

# Dioxus / generated
/.dioxus
/.dx

# Local app data
/data/*.db
/data/*.sqlite
/data/*.sqlite3
/data/*.db-wal
/data/*.db-shm

# Imported personal archives
/imports/x_archives/*.zip
/imports/manual/*

# Generated exports
/exports/**/*.jsonl
/exports/**/*.md
/exports/**/*.html

# Secrets
.env
*.token
*.key
```

### Optional funny safety doc

Good filename:

```text
docs/MURDO_REMEMBER_DONT_SHARE_YOUR_API_KEYS_WITH_LLMS.md
```

Contents idea:

```md
# Murdo, Remember: Don't Share Your API Keys With LLMs

This repository is public.

Do not commit:

- `.env`
- API keys
- bearer tokens
- OAuth secrets
- downloaded private archives
- raw account exports
- anything copied from a dashboard that says "secret"

Public viewers should never need an X API token.

Maintainer-only imports use local secrets, ignored by Git.

If a machine asks for your token, bonk the machine.
```

---

## Git Commands

### Check status

```bash
git status
```

### Normal compile loop

```bash
cargo fmt
cargo check
dx serve --desktop
```

### Normal commit

```bash
git add .
git commit -m "Describe the change"
git push
```

### Commit theme pass

```bash
git add src/ui/shell.rs src/ui/dashboard.rs src/ui/timeline.rs src/ui/search_panel.rs src/ui/import_panel.rs
git commit -m "Add fluid console archive theme to main panels"
git push
```

### Commit public dataset skeleton

```bash
git add src/public_data public_data src/main.rs
git commit -m "Add public dataset skeleton"
git push
```

### Commit safety docs

```bash
git add .env.example .gitignore docs
git commit -m "Add public-safe API key guidance"
git push
```

---

## Files to Touch Soon

```text
src/ui/dashboard.rs
src/ui/timeline.rs
src/ui/search_panel.rs
src/ui/import_panel.rs
src/ui/shell.rs
src/public_data/mod.rs
src/public_data/models.rs
src/public_data/sample.rs
public_data/manifest.json
public_data/posts.json
.env.example
.gitignore
README.md
```

---

## Files to Leave Alone for Now

```text
src/db/*
src/analysis/*
src/export/*
src/ingest/x_archive.rs
src/ingest/normalizer.rs
src/ingest/media_cache.rs
src/ingest/advanced_search.rs
migrations/*
data/*
imports/*
exports/*
```

Exception: `src/ingest/x_api.rs` can stay as-is unless the maintainer API test breaks compilation.

---

## Current Priority Checklist

```text
[ ] Fix any compile errors from the fluid console theme
[ ] Confirm `dx serve --desktop` launches
[ ] Commit theme pass
[ ] Create `public_data/`
[ ] Create `src/public_data/`
[ ] Add `PublicPost` / `PostKind`
[ ] Add `sample_posts()`
[ ] Wire Timeline and Search to sample posts
[ ] Create `public_data/manifest.json`
[ ] Create `public_data/posts.json`
[ ] Commit public dataset skeleton
[ ] Add `.env.example`
[ ] Confirm `.env` is ignored
[ ] Add API key safety doc
[ ] Commit safety docs
[ ] Later: implement real `public_data/posts.json` loader
[ ] Later: implement archive/API generator
```

---

## North Star

Build the app as a public museum exhibit, not a secret-hungry client.

```text
Friendly archive viewer first.
Maintainer importer second.
Satirical data-broker mirror third.
LLM mining exhibit always with context warnings.
```

