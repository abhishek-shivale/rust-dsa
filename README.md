# rust-dsa

Rustlings-style DSA practice, 150 exercises. Each is a LeetCode problem: a
doc comment holds the question, a `todo!()` stub is where your answer goes,
and a `#[cfg(test)]` block is the judge.

## Workflow

```sh
cargo test n001              # runs (and currently fails) exercise 1
cargo test                   # runs all 150 at once
```

1. Open the file cargo points you at (e.g. `src/exercises/n001_two_sum.rs`).
2. Read the doc comment at the top — that's the question.
3. Replace `todo!()` with your implementation.
4. Re-run `cargo test n001` (or whatever number) until it's green.

No custom runner, no watcher script — `cargo test` already gives pass/fail
per exercise.

## Ordering

Files are numbered `n001`..`n150` (the `n` prefix is only there because Rust
module names can't start with a digit) in difficulty order, not by topic:

| Range     | Tier    | Roughly              |
|-----------|---------|-----------------------|
| 1-35      | low     | LeetCode Easy         |
| 36-85     | medium  | LeetCode Medium       |
| 86-130    | high    | Medium (DP/graph-heavy) or easier Hard |
| 131-150   | extreme | LeetCode Hard classics |

Work through them in order for a difficulty ramp, or jump straight to a
number if you already know what you want to drill.

## Layout

```
src/
  common.rs        ListNode / TreeNode + vec<->struct helpers used by tests
  exercises/
    mod.rs          declares all 150 exercise modules, grouped by tier
    n001_two_sum.rs  doc comment (question) + fn stub + tests (answer check)
    ...
```

Most exercises are a single `pub fn` stub. A few "design" problems (Trie,
LRU Cache, Min Stack, Twitter, etc.) are a `struct` + `impl` instead —
same idea, `todo!()` in each method body, tests call the methods in
sequence.

## Adding more exercises

1. Pick the next free number and difficulty tier.
2. Add `src/exercises/nNNN_problem_name.rs` with a `//!` doc comment (link +
   prompt), a `pub fn`/`struct` stub whose body is `todo!()`, and a
   `#[cfg(test)] mod tests` with a couple of `assert_eq!`/`assert!` cases
   from the problem's own examples.
3. Add `pub mod nNNN_problem_name;` to `src/exercises/mod.rs` in the right
   tier section.
