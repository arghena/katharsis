<!--

NOTE: Please keep the links absolute (starting with `/`).
Relative links don't work properly when accessing this guide via the GitHub UI tabs.

-->

# Contributing to Katharsis

<!-- vale Google.Exclamation = NO -->

Thank you so much for contributing to Katharsis! We truly appreciate every single contribution you make.

<!-- vale Google.Exclamation = YES -->

> [!NOTE]
>
> - Before jumping into a PR be sure to search [existing PRs](https://github.com/arghena/katharsis/pulls) or [issues](https://github.com/arghena/katharsis/issues) for an open or closed item that relates to your submission.
> - Before performing any actions, such as submitting a PR or opening an issue, follow the relevant templates and process guidelines.
> - PRs that are used for code formatting should be ignored in the [.git-blame-ignore-revs](/.git-blame-ignore-revs) file.
> - The static assets for the documentation should be stored in the [assets](https://github.com/arghena/assets) repository.

## Guidelines

### Documentation

Here are some guidelines to maintain a consistent style and voice across the docs:

<!-- alex ignore easy -->

- Write clear, concise sentences. Avoid tangents.
  - If you find yourself using a lot of commas, consider breaking the sentence into multiple sentences or use a list.
  - Swap out complex words for simpler ones. For example, use instead of utilize.
- Be mindful with the word this. It can be ambiguous and confusing, don't be afraid to repeat the subject of the sentence if unclear.
  - For example, Katharsis uses Rust instead of Katharsis uses this.
- Use an active voice instead of passive. An active sentence is easier to read.
  - For example, Katharsis uses Rust instead of Rust is used by Katharsis. If you find yourself using words like was and by you may be using a passive voice.
- Avoid using words like easy, quick, simple, just, etc. This is subjective and can be discouraging to users.
- Avoid negative words like don't, can't, won't, etc. This can be discouraging to readers.
- Write in second person (you/your). This is more personal and engaging.
- Use gender-neutral language. Use developers, users, or readers, when referring to the audience.
- If adding code examples, ensure they are properly formatted and working.

> If you'd like to explore further into technical writing, check out the [Google Technical Writing Course](https://developers.google.com/tech-writing/overview).

### Code

When you first build the project, [rustup](https://github.com/rust-lang/rustup) will automatically configure the toolchain for you based on the information in the [rust-toolchain.toml](/rust-toolchain.toml) file. For more information about the toolchain, please refer to the relevant official documentation:

- [The toolchain file](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)
- [rustfmt](https://rust-lang.github.io/rustfmt)
- [Clippy](https://doc.rust-lang.org/clippy)
- [rust-analyzer](https://rust-analyzer.github.io)

## Code of Conduct

Do note that our [Code of Conduct](/.github/CODE-OF-CONDUCT.md) applies to all Katharsis community channels. Users are **highly encouraged** to read and adhere to them to avoid repercussions.

## Signing Commits

All pull requests must include [signed commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).

## Developer Certificate of Origin

All pull request submissions to this project must comply with the [DCO](https://developercertificate.org).

## Developing

- The development branch is `canary`.
- All pull requests should be opened against `canary`.

### Dependencies

- Install Rust and Cargo via [rustup](https://rustup.rs).
- Install [nextest](https://nexte.st/docs/installation/pre-built-binaries).
- Install [GitHub CLI](https://github.com/cli/cli#installation).

### Naming

> [!NOTE]
> The naming should follow the lowercase format.

<!-- vale Vale.Spelling["ci", "perf"] = NO -->

| Type     | Description                                        |
| -------- | -------------------------------------------------- |
| docs     | Changes affecting the documentation                |
| feat     | Introduces a new feature                           |
| fix      | Bug fix                                            |
| test     | Changes affecting the tests                        |
| chore    | Miscellaneous tasks                                |
| revert   | Revert a commit                                    |
| perf     | Performance improvements                           |
| refactor | Code refactoring                                   |
| style    | Code formatting                                    |
| build    | Changes affecting the build system or dependencies |
| ci       | Changes affecting the CI pipeline                  |

<!-- vale Vale.Spelling["ci", "perf"] = YES -->

| Action | Format          |
| ------ | --------------- |
| Branch | `TYPE/NAME`     |
| Commit | `TYPE: SUBJECT` |
| PR     | `TYPE: TITLE`   |

> If you want to learn more about this, please refer to [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0).

### Local Development

1. Clone the Katharsis repository (download only recent commits for faster clone):

   ```bash
   gh repo clone arghena/katharsis -- --filter=blob:none --branch canary --single-branch
   ```

1. Create a new branch:

   ```bash
   git checkout -b MY_BRANCH_NAME origin/canary
   ```

1. Install dependencies and run tests:

   ```bash
   cargo nextest run --locked
   cargo test --doc --locked
   ```

1. Start making changes to your code.
1. Run rustfmt and Clippy:

   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets --keep-going --locked
   ```

1. Run tests:

   ```bash
   cargo nextest run --locked
   cargo test --doc --locked
   ```

1. When your changes are finished, commit them to the branch:

   ```bash
   git add .
   git commit -S -s -m "DESCRIBE_YOUR_CHANGES_HERE"
   ```

1. To open a pull request you can use the GitHub CLI which automatically forks and sets up a remote branch. Follow the prompts when running:

   ```bash
   gh pr create
   ```
