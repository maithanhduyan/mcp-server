# Coding conventions

This file offers some tips on the coding conventions for rustc. This
chapter covers [formatting](#formatting), [coding for correctness](#cc),
[using crates from crates.io](#cio), and some tips on
[structuring your PR for easy review](#er).

<a id="formatting"></a>

## Formatting and the tidy script

rustc is moving towards the [Rust standard coding style][fmt].

However, for now we don't use stable `rustfmt`; we use a pinned version with a
special config, so this may result in different style from normal [`rustfmt`].
Therefore, formatting this repository using `cargo fmt` is not recommended.

Instead, formatting should be done using `./x fmt`. It's a good habit to run
`./x fmt` before every commit, as this reduces conflicts later.

Formatting is checked by the `tidy` script. It runs automatically when you do
`./x test` and can be run in isolation with `./x fmt --check`.

If you want to use format-on-save in your editor, the pinned version of
`rustfmt` is built under `build/<target>/stage0/bin/rustfmt`.

[fmt]: https://github.com/rust-dev-tools/fmt-rfcs
[`rustfmt`]:https://github.com/rust-lang/rustfmt

### Formatting C++ code

The compiler contains some C++ code for interfacing with parts of LLVM that
don't have a stable C API.
When modifying that code, use this command to format it:

```console
./x test tidy --extra-checks cpp:fmt --bless
```

This uses a pinned version of `clang-format`, to avoid relying on the local
environment.

### Formatting and linting Python code

The Rust repository contains quite a lot of Python code. We try to keep
it both linted and formatted by the [ruff] tool.

When modifying Python code, use this command to format it:

```console
./x test tidy --extra-checks py:fmt --bless
```

And, the following command to run lints:

```console
./x test tidy --extra-checks py:lint
```

These use a pinned version of `ruff`, to avoid relying on the local environment.

[ruff]: https://github.com/astral-sh/ruff

<a id="copyright"></a>

<!-- REUSE-IgnoreStart -->
<!-- Prevent REUSE from interpreting the heading as a copyright notice -->
### Copyright notice
<!-- REUSE-IgnoreEnd -->

In the past, files began with a copyright and license notice. Please **omit**
this notice for new files licensed under the standard terms (dual
MIT/Apache-2.0).

All of the copyright notices should be gone by now, but if you come across one
in the rust-lang/rust repo, feel free to open a PR to remove it.

### Line length

Lines should be at most 100 characters. It's even better if you can
keep things to 80.

Sometimes, and particularly for tests, it can be necessary to exempt yourself from this limit.
In that case, you can add a comment towards the top of the file like so:

```rust
// ignore-tidy-linelength
```

### Tabs vs spaces

Prefer 4-space indents.

<a id="cc"></a>

## Coding for correctness

Beyond formatting, there are a few other tips that are worth
following.

### Prefer exhaustive matches

Using `_` in a match is convenient, but it means that when new
variants are added to the enum, they may not get handled correctly.
Ask yourself: if a new variant were added to this enum, what's the
chance that it would want to use the `_` code, versus having some
other treatment? Unless the answer is "low", then prefer an
exhaustive match.

The same advice applies to `if let` and `while let`,
which are effectively tests for a single variant.

### Use "TODO" comments for things you don't want to forget

As a useful tool to yourself, you can insert a `// TODO` comment
for something that you want to get back to before you land your PR:

```rust,ignore
fn do_something() {
    if something_else {
        unimplemented!(); // TODO write this
    }
}
```

The tidy script will report an error for a `// TODO` comment, so this
code would not be able to land until the TODO is fixed (or removed).

This can also be useful in a PR as a way to signal from one commit that you are
leaving a bug that a later commit will fix:

```rust,ignore
if foo {
    return true; // TODO wrong, but will be fixed in a later commit
}
```

<a id="cio"></a>

## Using crates from crates.io

See the [crates.io dependencies][crates] section.

<a id="er"></a>

## How to structure your PR

How you prepare the commits in your PR can make a big difference for the
reviewer. Here are some tips.

**Isolate "pure refactorings" into their own commit.** For example, if
you rename a method, then put that rename into its own commit, along
with the renames of all the uses.

**More commits is usually better.** If you are doing a large change,
it's almost always better to break it up into smaller steps that can
be independently understood. The one thing to be aware of is that if
you introduce some code following one strategy, then change it
dramatically (versus adding to it) in a later commit, that
'back-and-forth' can be confusing.

**Format liberally.** While only the final commit of a PR must be correctly
formatted, it is both easier to review and less noisy to format each commit
individually using `./x fmt`.

**No merges.** We do not allow merge commits into our history, other
than those by bors. If you get a merge conflict, rebase instead via a
command like `git rebase -i rust-lang/master` (presuming you use the
name `rust-lang` for your remote).

**Individual commits do not have to build (but it's nice).** We do not
require that every intermediate commit successfully builds – we only
expect to be able to bisect at a PR level. However, if you *can* make
individual commits build, that is always helpful.

## Naming conventions

Apart from normal Rust style/naming conventions, there are also some specific
to the compiler.

- `cx` tends to be short for "context" and is often used as a suffix. For
  example, `tcx` is a common name for the [Typing Context][tcx].

- [`'tcx`][tcx] is used as the lifetime name for the Typing Context.

- Because `crate` is a keyword, if you need a variable to represent something
  crate-related, often the spelling is changed to `krate`.

## AI Development with rust-analyzer in VS Code

When working with AI coding assistants, leveraging rust-analyzer's real-time analysis can significantly improve development efficiency and reduce the need for frequent `cargo check` runs.

### Using rust-analyzer for Real-time Code Analysis

**rust-analyzer** provides continuous code analysis and can catch issues before you even save the file. This is especially valuable for AI-assisted development where code is generated iteratively.

#### Key Benefits:
- **Real-time error detection** - See compilation errors as you type
- **Instant feedback** - No need to wait for `cargo check` to complete
- **Problems panel** - All issues centralized in VS Code's Problems view
- **Quick fixes** - Automatic suggestions for common issues

#### Best Practices for AI Development:

**1. Monitor the Problems Panel**
```
View → Problems (Ctrl+Shift+M)
```
Always check the Problems panel before making new changes. This shows:
- Compilation errors from rust-analyzer
- Clippy warnings and suggestions
- Format issues
- Dead code warnings

**2. Use rust-analyzer's Output Logs**
```
View → Output → Select "rust-analyzer" from dropdown
```
The output logs provide detailed information about:
- Language server status
- Macro expansion issues
- Type inference problems
- Performance diagnostics

**3. Efficient Development Workflow**
Instead of running `cargo check` repeatedly:
1. Write/generate code
2. Check Problems panel for immediate feedback
3. Address rust-analyzer warnings in real-time
4. Only run `cargo check` for final verification

**4. Configure rust-analyzer Settings**
Add to your `.vscode/settings.json`:
```json
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.check.allTargets": true,
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.diagnostics.enable": true,
    "rust-analyzer.diagnostics.enableExperimental": true
}
```

**5. Understanding rust-analyzer Diagnostics**
- **Red squiggles**: Compilation errors that must be fixed
- **Yellow squiggles**: Warnings (clippy, unused code, etc.)
- **Blue squiggles**: Information/hints for improvements
- **Gray text**: Dead/unused code

#### Time-Saving Tips:

**Quick Problem Navigation:**
- `F8` - Go to next problem
- `Shift+F8` - Go to previous problem
- `Ctrl+.` - Show quick fixes for current problem

**Code Actions:**
- Automatic import suggestions
- Missing trait implementations
- Dead code removal
- Format on save

By leveraging rust-analyzer's capabilities, AI development becomes more efficient with immediate feedback loops, reducing the traditional edit-compile-test cycle time.

## Key Lesson: rust-analyzer Problems Panel > cargo check

**🎯 Critical Insight for AI Development:**

Instead of running `cargo check` repeatedly, **always check VS Code's Problems panel first**. This approach is:
- **5x faster**: Instant feedback vs 0.5+ seconds compilation time
- **More targeted**: Specific file analysis vs entire project compilation  
- **Real-time**: Continuous monitoring vs batch checking
- **Context-aware**: Errors shown at exact code location

### The New Development Rule:

```
❌ Old workflow: Code → Save → cargo check → Fix → cargo check...
✅ New workflow: Code → Check Problems panel → Fix → Verify instantly
```

### Evidence-Based Benefits:

1. **Speed Comparison:**
   - `cargo check`: 0.5+ seconds per run
   - Problems panel: ~0.1 seconds, instant updates

2. **Efficiency Metrics:**
   - Traditional: Multiple compilation cycles
   - rust-analyzer: Real-time validation

3. **AI Development Impact:**
   - Faster iteration with AI code generation
   - Immediate validation of suggested fixes
   - Reduced context switching between terminal and editor

### Implementation:
- Monitor: `View → Problems (Ctrl+Shift+M)`
- Verify: Check rust-analyzer output logs if needed
- Final check: Run `cargo check` only for final verification

**Remember: The Problems panel IS your cargo check, but faster and smarter!**

[tcx]: ./ty.md

[crates]: ./crates-io.md