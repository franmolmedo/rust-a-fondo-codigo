# GitHub Actions template

`appendices.yml` mirrors the editorial repository's workflow. It verifies
Windows/Linux native builds, records cross-compilation, runs Miri and a short
Linux fuzzing campaign, and uploads native application packages as artifacts.
The workflow has read-only repository permissions and does not publish releases.

For a code-only repository where this workspace is at `appendices/`:

1. Copy this YAML to the repository root's `.github/workflows/appendices.yml`.
2. Replace every `code/appendices` path with `appendices`, including working
   directories, pull-request filters and artifact paths.
3. Remove the `docs/*.Apendice-*.md` path filter and the step named
   `Check manuscript examples`. It requires the commercial manuscript.
4. Keep `python verify.py`: it verifies the committed doctest corpora without
   needing the manuscript. Keep the advanced, cross-target and packaging steps.
5. Run the workflow manually and inspect each job before calling CI verified.

Use an unused package version when releasing changes. Update the version in
Cargo.toml, Cargo.lock and the packaging command together. Downloaded artifacts
still need clean-machine testing and a publication review.

Code: MIT. Author: Francisco M. Olmedo Bueno. Developed with AI assistance.
