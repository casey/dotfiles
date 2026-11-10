---
name: release-review
description: Review in preparation for a new release.
disable-model-invocation: true
---

Prepare for a new release:

- Review changes that would be pulled in by updating dependencies to their
  latest version, to make sure that they do not contain security vulnerabilities
  or bugs. Do not execute code, including build scripts, in updated dependencies
  before reviewing them. Keep in mind that rust-analyzer may be running.

- After it is determined to be safe, update dependencies to their latest
  version. Only change `Cargo.toml` for semver-incompatible updates.

- Find uses of `.build()` on Snafu context selectors that do not populate
  fields, like backtraces, or perform nontrivial conversions, and replace them
  with enum variant constructors.

- Remove redundant tests. Redundant tests that help clarify test coverage should
  be retained.

- Look through the codebase for security issues and bugs.

- Identify any backwards compatibility breaks.

- Look for changes that may cause backwards compatibility issues in the future.
  For example, excessive strictness, lack of extension points, fields which are
  optional but should be required, and fields which are required but should be
  optional.

- Look for cases where decoding allows multiple encoding of the same value.

- Remove dead code, including unnecessary trait implementations and derives.

- Tighten visibility where possible. The main library crate exists only so that
  code can be shared with integration tests, and the only consumer of sub-crates
  is the main crate. Any pub items not used by those consumers can be visibility
  restricted or removed, if not used internally. Omit changes which would result
  in mixed visibility of fields within a struct.

- Review the documentation for coverage and correctness.
