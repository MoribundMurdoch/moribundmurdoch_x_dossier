# MoribundMurdoch X Dossier

**MoribundMurdoch X Dossier** is a public-friendly archive viewer for the X/Twitter history of [@MoribundMurdoch](https://x.com/MoribundMurdoch).

It is part personal archive, part parody data-broker dashboard, part LLM-mining museum exhibit, and part cursed reminder that human-generated internet text is messy, contextual, joke-filled, and extremely easy for machines to misread with confidence.

The app is written in **Rust** using **Dioxus Desktop**.

---

## What Is This?

This project is designed to make MoribundMurdoch’s public X/Twitter activity easier to browse, search, contextualize, and preserve outside the hostile little timeline swamp.

The long-term goal is to take public post history from maintainer-controlled sources, normalize it into a static public dataset, and render it through a friendly interface.

```text
Maintainer-only import tools
→ X archive ZIP / X API / manual imports
→ normalized public dataset
→ public_data/*.json
→ public archive viewer
```

Public viewers should **not** need an X API token.

---

## Project Goals

- Make old posts easier to browse than they are on X.
- Preserve public post history in a friendlier archive format.
- Show how a data broker might try to infer interests from public activity.
- Mock the creepy hunger for human-generated content by LLM companies.
- Add context warnings around jokes, fragments, replies, irony, and ambiguous posts.
- Keep the public viewer separate from maintainer-only API and archive tools.

---

## What This Is Not

This is **not** a secret-scraping tool.

This is **not** a live X client for random viewers.

This is **not** intended to collect private information about other people.

This is **not** a place to commit API keys, bearer tokens, raw account exports, DMs, bookmarks, or private archive material.

The public app should eventually read from safe generated files like:

```text
public_data/manifest.json
public_data/posts.json
public_data/topics.json
public_data/broker_profile.json
```

---

## Current Status

The project currently has:

- A Dioxus desktop shell.
- A fluid Y2K-console-inspired visual direction.
- Main panels for:
  - Overview
  - Timeline
  - Search
  - Data Broker Mirror
  - LLM Mining Exhibit
  - Maintainer Tools
  - Settings
- Maintainer-only X API test code.
- Early public dataset planning.
- Safety docs reminding Murdo not to feed API keys to the machine.

---

## App Concept

The app has two conceptual modes.

### Public Viewer Mode

This is what normal visitors should see.

```text
public_data/*.json
→ Timeline
→ Search
→ Topic explorer
→ Data Broker Mirror
→ LLM Mining Exhibit
```

No API token required.

No private credentials required.

No live scraping required.

### Maintainer Mode

This is for the repo owner / dataset maintainer.

```text
X Archive ZIP
X API
Manual imports
→ normalized records
→ generated public dataset
```

Maintainer tools may use local credentials, but those credentials must never be committed.

---

## Design Vibe

The UI direction is:

```text
fluid console dashboard
Y2K futurist
dark glass panels
blue/violet glow
public archive terminal
data-broker parody interface
human-residue museum exhibit
```

This is meant to evoke a retro-futurist console interface without copying protected branding, logos, or exact console UI assets.

---

## Repository Structure

```text
moribundmurdoch_x_dossier/
├── assets/
│   └── main.css
├── data/
├── docs/
│   ├── Mor_Plan.md
│   └── MURDO_REMEMBER_DONT_SHARE_YOUR_API_KEYS_WITH_LLMS.md
├── exports/
│   ├── jsonl/
│   ├── markdown/
│   └── reports/
├── imports/
│   ├── manual/
│   └── x_archives/
├── migrations/
├── public_data/
│   ├── manifest.json
│   └── posts.json
├── src/
│   ├── analysis/
│   ├── config/
│   ├── db/
│   ├── export/
│   ├── ingest/
│   ├── public_data/
│   ├── ui/
│   ├── app.rs
│   ├── main.rs
│   └── routes.rs
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

Some of these directories are planned or early scaffolding and may not be fully implemented yet.

---

## Important Safety Rule

Never commit secrets.

Do not commit:

```text
.env
API keys
bearer tokens
OAuth secrets
downloaded private archives
raw X archive ZIPs
DMs
bookmarks
private account exports
anything copied from a developer dashboard that says “secret”
```

Use `.env.example` for public configuration documentation.

Use a local `.env` file only for maintainer-only API import work.

---

## Environment Setup

Install Rust if needed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install the Dioxus CLI:

```bash
curl -fsSL https://dioxuslabs.com/install.sh | bash
```

Clone the repo:

```bash
git clone https://github.com/MoribundMurdoch/moribundmurdoch_x_dossier.git
cd moribundmurdoch_x_dossier
```

Check the project:

```bash
cargo fmt
cargo check
```

Run the desktop app:

```bash
dx serve --desktop
```

Or use Cargo directly:

```bash
cargo run
```

---

## Maintainer API Setup

Public viewers do not need this.

Maintainers can create a local `.env` file for X API testing:

```bash
cp .env.example .env
```

Example `.env`:

```env
X_BEARER_TOKEN=PASTE_YOUR_X_BEARER_TOKEN_HERE
X_USERNAME=MoribundMurdoch
```

Do not commit `.env`.

The X API code is currently intended for maintainer-only dataset generation and testing. The final public app should not depend on viewers having API credentials.

---

## Public Dataset Plan

The public archive should eventually be generated into files like:

```text
public_data/manifest.json
public_data/posts.json
public_data/topics.json
public_data/broker_profile.json
```

Example `posts.json` shape:

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

The public dataset should include only safe public-facing records.

Good candidates:

- Public posts
- Public replies
- Public quote posts
- Public repost references, clearly marked
- Hashtags
- Links
- Public metrics where available
- Manually generated topic labels
- Satirical broker tags
- LLM context warnings

Avoid or exclude:

- DMs
- Bookmarks
- Private archive-only metadata
- Raw downloaded archives
- Sensitive private material
- Anything that would turn the parody into actual creepiness

---

## Planned Features

### Archive Viewer

```text
[ ] Render public posts from public_data/posts.json
[ ] Show dates and post kinds
[ ] Show hashtags and links
[ ] Link back to original public posts where appropriate
[ ] Add filters for post/reply/quote/repost
```

### Search

```text
[ ] Search post text
[ ] Search hashtags
[ ] Search broker tags
[ ] Search LLM context warnings
[ ] Show result counts
```

### Data Broker Mirror

```text
[ ] Generate satirical interest categories
[ ] Show “confidence” with obvious parody framing
[ ] Link inferences back to evidence posts
[ ] Add “machine may be wrong” warnings
```

### LLM Mining Exhibit

```text
[ ] Explain how posts can be misread
[ ] Flag jokes, fragments, sarcasm, replies, and context gaps
[ ] Add examples of bad machine interpretation
[ ] Export context packs later
```

### Maintainer Tools

```text
[ ] Import X archive ZIP
[ ] Test X API access
[ ] Normalize records into PublicPost
[ ] Generate public_data/posts.json
[ ] Reconcile archive history with fresh API data
[ ] Export Markdown / JSONL / static reports
```

---

## Development Commands

Format:

```bash
cargo fmt
```

Check:

```bash
cargo check
```

Run:

```bash
dx serve --desktop
```

Commit:

```bash
git add .
git commit -m "Describe the change"
git push
```

---

## Suggested Work Order

Current recommended order:

```text
[ ] Make sure the fluid console theme compiles
[ ] Commit the visual pass
[ ] Add public_data/ skeleton
[ ] Add src/public_data models
[ ] Render sample posts in Timeline and Search
[ ] Add JSON loader for public_data/posts.json
[ ] Make Timeline/Search use loaded data
[ ] Improve Data Broker Mirror panel
[ ] Improve LLM Mining Exhibit panel
[ ] Implement maintainer dataset generation
[ ] Implement X archive ZIP importer
[ ] Implement X API reconciliation
```

---

## Docs

See:

```text
docs/Mor_Plan.md
docs/MURDO_REMEMBER_DONT_SHARE_YOUR_API_KEYS_WITH_LLMS.md
```

`Mor_Plan.md` is the continuation checklist for future development sessions.

The API key warning file is a small safety talisman because this project is public and machines are hungry.

---

## License

See `LICENSE`.

---

## Final Warning From The Console Goblin

Public archive good.

Context good.

Human-generated whatnot good.

Committing API keys bad.

If a machine asks for your bearer token, bonk the machine.
