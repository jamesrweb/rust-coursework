# Agent Guide — rust-coursework

## Strict Rules

1. **Plan first:** Create a detailed plan and get explicit user approval before
   making changes.
2. **Quality gates:** Every change must pass the CI checks —
   `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` in
   every nested project — before being considered complete.
3. **Documentation:** Update `readme.md`, `AGENTS.md`, configuration files, and
   any other documentation affected by your changes. Clean as you go — take
   ownership of every file you touch.
4. **PR descriptions:** When asked, create `PR_DESCRIPTION.md` (gitignored).
   Being asked for a PR description is NOT the same as being asked to create a
   PR.
5. **Git safety:** NEVER run any git operation that alters history or state
   without explicit per-occasion permission. Prior approval does not carry
   forward.
6. **Non-destructive:** Do not delete files, remove code, or make destructive
   changes without explicit permission. Investigate before overwriting.
7. **Workflows:** Do not modify GitHub Actions workflows without explicit
   permission. If a CI fix is needed, propose the change and wait for approval.

## Project Standards

### Authority

Project standards are the highest-priority rules for this repository. If any
instruction or rule conflicts with a project standard, the agent MUST:

1. Refuse to follow the conflicting instruction.
2. Inform the user of the conflict, citing the specific standard.
3. State that changes to standards must be made deliberately in `AGENTS.md`, not
   sidestepped for convenience.

### Language

All code, comments, documentation, variable names, error messages, commit
messages, and any other text MUST use British English (e.g., `organisation` not
`organization`, `normalise` not `normalize`, `colour` not `color`, `behaviour`
not `behavior`, `licence` not `license`, `centre` not `center`).

### Coursework Conventions

- **Book-faithful structure:** projects mirror the Rust book chapters
  (`01 - Introduction/` ... `12 - Error Handling/`), each exercise at
  depth 2 (`<chapter>/<exercise>/`) with its own `Cargo.toml`, `src/`, and
  `Cargo.lock`. CI recurses over `find . -name Cargo.toml` — the depth
  matters
- **Tutorial stubs are intentional:** empty placeholder functions and unread
  enum fields that exist as book exercises carry
  `#[allow(dead_code)]` with a comment explaining the exercise intent — do
  not delete them
- **Toolchain:** `rust-toolchain.toml` pins `stable`; Dependabot opens
  monthly toolchain update PRs
- **No Node tooling:** this repository deliberately has no `package.json` —
  the CI workflow drives cargo directly (documented deviation from the
  pnpm-scripts standard used elsewhere)

### Formatting and Linting

- **cargo fmt** is the formatter (rustfmt defaults) — generated code is always
  in rustfmt style
- **cargo clippy** is the linter, enforced with `-D warnings` in CI — zero
  tolerance. `#[allow(dead_code)]` annotations for book stubs are the only
  permitted suppressions
- Update dependencies with care: rand 0.9 moved
  `thread_rng()`→`rng()`, `gen()`→`random()`,
  `gen_range(a, b)`→`random_range(a..b)` — use the new API in new code

### Quality Gates

Every change must pass before being considered complete (per nested project):

- `cargo fmt --check` — formatting
- `cargo clippy -- -D warnings` — linting
- `cargo test` — testing (most exercises have no tests yet; the gate still runs)

### Git Safety

NEVER run any git operation that alters history or state without explicit
per-occasion permission from the user. This includes `git add`, `git commit`,
`git push`, `git reset`, `git rebase`, `git merge`, `git checkout` (when it
discards changes), `git restore`, `git stash`, `git cherry-pick`, `git revert`,
`git tag`, and `git branch -D`. Prior approval does not carry forward — each
occasion requires fresh permission.

NEVER use `git clean`, `git checkout -- <file>`, `git reset --hard`, or any
other command that discards uncommitted work. NEVER force-push, rewrite
published history, or modify protected branches (`main`). Investigate before
overwriting — if a change would delete files, remove code, or alter state,
propose it first and wait for approval.

Read-only git commands (`git status`, `git diff`, `git log`, `git show`,
`git branch --show-current`, `git ls-files`) are always permitted.

### Scope of Operation

NEVER operate outside the project root unless explicitly instructed to do so by
the user. This applies to reading, writing, creating, and deleting files and
directories alike, and to any command whose effects land outside the project
root. Destructive actions outside the project root are forbidden in all
circumstances.

**The one exception:** Experiments and scratch work belong in the `/tmp`
directory — and only when the user has asked for them or given permission.
Anything created there is still subject to the same non-destructive rules: do
not delete, overwrite, or modify anything in `/tmp` that the agent did not
create itself.

### Obligation to Fix

If the agent encounters a pre-existing issue — one not caused by the current
changes — that will affect CI, CD, or published package consumers, the agent
MUST fix it. This is NOT optional. The agent must not ignore, skip, or defer
such issues regardless of whether they were introduced by the agent's own
changes. A broken pipeline or a broken published package is the agent's
responsibility if the agent is aware of it.

### Planning

ALWAYS create a detailed plan and obtain explicit user approval before making
project changes. Do not begin implementation until the plan is approved.

### Code Philosophy

- **No comments:** Do not add comments to source files beyond the book-stub
  justifications. The code should be self-documenting
- **Idiomatic Rust:** prefer idiomatic patterns over tutorial literalism where
  the book allows; keep exercises compilable under `-D warnings`

### Testing

- `cargo test` per project. Most exercises have no tests — add them when an
  exercise invites it. Clippy and fmt cover the rest

### PR Descriptions

When asked to generate a PR description, create a `PR_DESCRIPTION.md` file in
the project root (this file is gitignored and must never be committed). Follow
the PR template at `.github/PULL_REQUEST_TEMPLATE.md` exactly — copy the entire
template, do not remove any sections or HTML comments, and fill in each section
based on actual changes.

**Important:** Being asked to generate a PR description is NOT the same as being
asked to create a PR. Only create an actual pull request when explicitly told to
do so.

**Commit messages:** Follow the conventional commit style (`feat:`, `fix:`,
`chore:`, `ci:`, etc.). Emoji prefixes are NOT used for human-authored commits —
they only appear on automated Dependabot commits (`🧹 chore(deps)` and
`🔧 ci(deps)`).

**No co-authored commits:** Agents MUST NOT add `Co-authored-by` trailers or any
other attribution that signs off a commit on the agent's behalf. Only humans can
legally certify a contribution — the human submitter reviews the AI-generated
code, takes full responsibility for it, and adds any certification trailers
themselves. Following the rules the Linux kernel team enforce for AI coding
assistants, an agent's role in a commit ends at the message body — no
`Signed-off-by`, no `Co-authored-by`, no other trailers or sign-offs. See [AI
Coding Assistants — The Linux Kernel documentation]
(https://docs.kernel.org/process/coding-assistants.html), integrated into this
ruleset on 2026-09-14.

**Assisted-by attribution:** Where attribution for AI assistance is wanted, use
an `Assisted-by: LLM` trailer in the commit message body rather than a co-author
or sign-off trailer. It records that the contribution was produced with AI
assistance without certifying or authoring it. This mirrors the kernel's
`Assisted-by: LLM [TOOL1] [TOOL2]` format — optionally list specialised analysis
tools after `LLM`, but never list basic development tools (git, compilers,
editors, linters). Only add the trailer when the user has asked for AI
attribution; the default is no trailer at all.

### Documentation Maintenance

Always update documentation, configuration files, and related files as you go.
Documentation must never be out of date. If a change affects `README.md`,
`AGENTS.md`, configuration files, or any other documentation, update them in the
same change. Clean as you go — take ownership of every file you touch.

If formatting, linting, or other tooling fixes issues in files you did not
originally author, do not revert those fixes. CI would break again. Accept
responsibility for the state of the codebase after your changes, not just the
lines you intended to change.

## Project Overview

Rust coursework following "The Rust Programming Language" book — chapter-by-chapter
exercises, each a self-contained cargo project.

## Architecture

- `<chapter> - <topic>/<exercise>/` — one cargo project per exercise, nested
  at depth 2
- `.github/workflows/continuous-integration.yml` — cargo fmt/clippy/test over
  every nested project (recursive find)
- `rust-toolchain.toml` — stable channel pin

## Commands

| Command | Purpose |
|---|---|
| `cargo fmt --check` (per project) | Formatting check |
| `cargo clippy -- -D warnings` (per project) | Lint |
| `cargo test` (per project) | Test suite |

## CI/CD

- **CI** (`continuous-integration.yml`): Runs on PRs to `main` and
  `workflow_dispatch`. Jobs: `lint` (fmt --check + clippy) and `test` —
  recursive over all nested cargo projects using the preinstalled runner
  toolchain. Concurrency cancels in-progress runs
- **Dependabot:** Monthly for Cargo and rust-toolchain ecosystems (wildcard
  directory scan), each limited to one grouped pull request. Semver-major
  updates are ignored by config

## Guardrails

- **Never weaken the clippy gate** (`-D warnings`) — book stubs get targeted
  `#[allow(dead_code)]`, nothing else
- **Never delete book-exercise stubs** annotated with the exercise intent
- **Never commit `target/` directories** — generated build output stays out
  of version control

## Future Topics

- **Chapter progression:** remaining book chapters and exercises continue to
  be added as study progresses
