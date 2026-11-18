---
name: merge
description: >-
  Merge the current changes of the leetcode Rust repository
  (github.com/Papsanly/leetcode) into main: commit them, rebase onto
  origin/main, check them with cargo clippy, fast-forward push to GitHub,
  and fast-forward the main checkout. Only invoke this skill when the user
  has explicitly requested merging (or landing) the changes, not merely for
  review, preparation, passing checks, or installing or editing this skill.
disable-model-invocation: true
metadata:
  delta-action: land
---

# Merge changes into `main`

This skill applies only to the `leetcode` crate (`Cargo.toml`, package
`leetcode`) whose `origin` remote is `https://github.com/Papsanly/leetcode`.
If `git remote get-url origin` prints anything else, stop: this workflow is
not for that repository.

## Ground rules

- Invoking this skill (for example `/merge`) **is** the explicit merge
  request. Carry the workflow through to the end. Do not ask whether the user
  wants to merge, and never answer that you are waiting for an explicit merge
  request. Stop only for a genuine blocker or unresolved scope.
- The job is done only when the commits are on `origin/main` and that has
  been verified (step 7). Committing, rebasing, or starting a push is not
  success. If a blocker stops you, say plainly that the changes have **not**
  merged, name the blocker, and describe the state you left (for example
  "committed locally, nothing pushed").
- **`cargo clippy` is a required check** (step 4). The owner installed a
  stable Rust toolchain through `rustup`. Never install, update, or switch
  toolchains or components yourself (`rustup default`, `rustup update`,
  `rustup toolchain install`, `rustup component add`). If `cargo clippy`
  cannot run, that is a blocker: stop and report it.
- Never force-push (`--force`, `--force-with-lease`), never rewrite commits
  that are already on `origin/main`, and never push to another branch as a
  substitute for `main`.
- Never write to git config (`git config --global/--local ... <value>`).
- Use only non-interactive git commands: no `rebase -i`, no `add -p`, no
  command that opens an editor.
- `LF will be replaced by CRLF` warnings come from `core.autocrlf=true` and
  are not errors.

The history is linear with direct commits on `main` (no merge commits, no
pull requests; see `git log --oneline --graph origin/main`). Keep it that way.

## 1. Preflight and scope

```
git remote get-url origin
git status --short --branch
git fetch origin
git log --oneline origin/main..HEAD
```

- If `git status` reports a rebase, merge, or cherry-pick already in
  progress, stop and report it; do not abort or continue someone else's
  operation.
- The change to merge is **everything in this working copy**: all
  uncommitted changes (modified, deleted, and untracked files not ignored by
  `.gitignore`) plus every commit in `origin/main..HEAD`. The current branch
  may be `main` or any other branch; either way the result is pushed to
  `main`.
- If the working tree is clean and `origin/main..HEAD` is empty, there is
  nothing to merge: report that and stop.
- Stop and ask before including anything that looks unrelated or unsafe:
  credentials or `.env`-style files, build output (`/target` is already
  ignored by `.gitignore`), large binaries, or files that clearly do not
  belong to the work being merged. Do not guess and do not silently drop
  them.

## 2. Commit uncommitted work

Skip this step if the working tree is already clean. Leave existing unpushed
commits as they are.

**Author identity.** Check whether git has an explicit identity:

```
git -c user.useConfigOnly=true var GIT_AUTHOR_IDENT
```

- Exit code 0: an identity is configured; use plain `git` commands.
- Otherwise: reuse the author of the latest commit on `origin/main`, passed
  per command. Read it with

  ```
  git log -1 --format=%an origin/main
  git log -1 --format=%ae origin/main
  ```

  and add `-c "user.name=<name>" -c "user.email=<email>"` right after `git`
  on **every** command that creates commits: `commit`, `rebase`, and
  `rebase --continue`.

**Grouping.** Stage by explicit path (`git add -- <paths>`), never
`git add -A` blindly.

- One commit per problem: the solution file `src/<difficulty>/pN_<slug>.rs`
  together with its `mod pN_<slug>;` line in `src/<difficulty>/mod.rs`.
- If changes for several problems share a file and cannot be separated by
  path, put them in one commit that names all of them.
- Files that are not solutions (tooling, configuration, skills, docs) go in
  their own commit.

**Subject.** Match the existing style in `git log --oneline origin/main`
(`solve p10`, `solve p11`): short, lowercase, subject only.

- New solution: `solve p<N>` (several: `solve p14 p15`).
- Rework of an existing solution: a short verb plus the problem, such as
  `improve p23`, `fix p23`, or `refactor p23`.
- Other files: a short description such as `add merge skill`.

```
git add -- <paths>
git commit -m "<subject>"
```

Finish with `git status --short`; it must print nothing.

## 3. Sync with `origin/main`

```
git fetch origin
git merge-base --is-ancestor origin/main HEAD
```

If the second command exits 0, `HEAD` is already on top of `origin/main`;
go to step 4. Otherwise rebase (with the identity options from step 2 if
they are needed):

```
git rebase origin/main
```

**Conflicts: resolve automatically when the intended result is clear.**

- Clear: both sides add different `mod pN_<slug>;` lines to the same
  `mod.rs` (keep both, in the file's existing sort order); both sides touch
  different problems or clearly independent lines. Resolve, keeping both
  sides' work, then:

  ```
  git add -- <resolved files>
  git -c core.editor=true rebase --continue
  ```

- Not clear: both sides changed the same solution's logic, one side deleted
  what the other modified, or any resolution would discard someone's work.
  Run `git rebase --abort`, report which files conflicted and why, and state
  that the changes have not merged. Do not pick a side by guessing.
- Never use `git rebase --skip` and never drop a commit to get past a
  conflict.

## 4. Checks on the exact commit being merged

Record `git rev-parse HEAD`. Every check below must hold for that exact
commit; if `HEAD` changes afterwards (for example after another rebase), run
them again.

**Repository-defined checks.** When this skill was written the repository
defined none of its own: no CI configuration (`.github/workflows/` or
similar), no git hooks, and no `AGENTS.md`, `CONTRIBUTING`, or other
contribution policy. Confirm that is still true:

```
git ls-files .github AGENTS.md AGENT.md "CONTRIBUTING*" "README*"
git config --get core.hooksPath
```

If either prints something that defines checks or contribution requirements,
those are now required: every one of them must have passed for this exact
commit before pushing. Pending, failing, missing, or unverifiable checks are
not a pass. If one cannot be run or verified here (it needs a tool that is
not installed, or only runs after a push), stop, report that the changes
have not merged, and say that this skill needs updating for the new checks.

**Static checks** (always):

1. No leftover conflict markers:

   ```
   git grep -n -E '^(<<<<<<<|>>>>>>>) '
   ```

   Exit code 1 with no output is a pass. Any match is a failure.

2. Module declarations agree with the files. `src/lib.rs` declares the
   difficulty modules (`mod easy; mod hard; mod medium;`) and each
   `src/<difficulty>/mod.rs` holds exactly one `mod <file stem>;` line per
   `p*.rs` file beside it. Compare `git ls-files src` with the `mod.rs`
   files: every solution file needs its line, and no line may point at a
   missing file. A new difficulty directory likewise needs its `mod` line in
   `src/lib.rs`.

**Clippy** (always, after the static checks):

```
cargo clippy --all-targets -- -D warnings
```

Exit code 0 is a pass; anything else is a failure, including `cargo` not
being able to run.

- `--all-targets` includes the `#[cfg(test)]` modules. `-D warnings` turns
  every warning, from clippy or the compiler, into an error, so a run that
  finished with warnings cannot be mistaken for a pass.
- Run it on the clean working tree that step 2 leaves, so it checks exactly
  the commit being pushed. Clippy only sees files that are in the module
  tree, which is why static check 2 comes first.
- On failure, report the diagnostics (file, line, lint name). Do not edit
  code, add `#[allow(...)]`, run `cargo clippy --fix`, or drop `-D warnings`
  to get past it.

If any check fails, stop and report that the changes have not merged (the
commits stay local; nothing is pushed).

## 5. Merge: fast-forward push to GitHub

```
git -c credential.interactive=false push origin HEAD:main
```

GitHub access uses the machine's Git Credential Manager
(`git config --get credential.helper` prints `manager`); the option makes the
push fail instead of waiting on a login prompt.

- Rejected as non-fast-forward: `origin/main` moved. Go back to step 3 and
  repeat (at most twice more), then stop and report.
- Authentication failure: stop and report that the changes have not merged
  and that the GitHub login needs refreshing by the user.
- Rejected by branch protection or a required status check: stop and report.
  Do not try to bypass it.

## 6. Fast-forward the main checkout

A working copy created from the owner's main checkout has a remote named
`local` that points at that checkout, set up so that a push to its
checked-out `main` updates its files (see
`git config --get remote.local.receivepack`, which sets
`receive.denyCurrentBranch=updateInstead`).

```
git remote get-url local
```

- If that fails, there is no separate main checkout to update (you are
  probably in it already): skip to step 7.
- Otherwise:

  ```
  git push local HEAD:main
  ```

  On success, also refresh that checkout's view of GitHub so it does not
  look ahead of `origin/main`, using the path printed by
  `git remote get-url local`:

  ```
  git --git-dir="<path printed above>" fetch origin
  ```

- If the push to `local` is rejected (that checkout has uncommitted work or
  its `main` has diverged), leave it untouched. Do not force. The merge on
  GitHub still stands; tell the user to run `git pull` in the main checkout.

## 7. Verify and report

```
git rev-parse HEAD
git ls-remote origin refs/heads/main
git ls-remote local refs/heads/main
git status --short --branch
```

(Skip the `local` line when there is no `local` remote.)

- **Merged** only if the hash from `origin` equals `HEAD`. If it does not,
  the changes have not merged: report that with the reason.
- If the hash from `local` differs, report that GitHub is updated but the
  main checkout is not, and that `git pull` there will bring it up to date.

Report the merged commit hash, the commit subjects that went in, and whether
the main checkout was updated.
