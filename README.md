<div align="center">

[![License][license-badge]](LICENSE)
[![CI Status][ci-badge]][ci]
[![Conda Platform][conda-badge]][conda-url]
[![Project Chat][chat-badge]][chat-url]
[![Pixi Badge][pixi-badge]][pixi-url]

[license-badge]: https://img.shields.io/github/license/pavelzw/pixi-diff?style=flat-square
[ci-badge]: https://img.shields.io/github/actions/workflow/status/pavelzw/pixi-diff/ci.yml?style=flat-square&branch=main
[ci]: https://github.com/pavelzw/pixi-diff/actions/
[conda-badge]: https://img.shields.io/conda/vn/conda-forge/pixi-diff?style=flat-square
[conda-url]: https://prefix.dev/channels/conda-forge/packages/pixi-diff
[chat-badge]: https://img.shields.io/discord/1082332781146800168.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2&style=flat-square
[chat-url]: https://discord.gg/kKV8ZxyzY4
[pixi-badge]: https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/prefix-dev/pixi/main/assets/badge/v0.json&style=flat-square
[pixi-url]: https://pixi.sh

</div>

# pixi-diff

A simple executable to generate a JSON diff (similar to `pixi update --json`) between two pixi lockfiles.

```text
$ pixi-diff pixi.lock.old pixi.lock.new
{
  "version": 1,
  "environment": {
    "default": {
      "osx-arm64": [
        {
          "name": "libmpdec",
          "before": null,
          "after": {
            "conda": "https://conda.anaconda.org/conda-forge/osx-arm64/libmpdec-4.0.0-h99b78c6_0.conda",
            "sha256": "f7917de9117d3a5fe12a39e185c7ce424f8d5010a6f97b4333e8a1dcb2889d16",
            "md5": "7476305c35dd9acef48da8f754eedb40",
            "depends": [
              "__osx >=11.0"
            ],
            "license": "BSD-2-Clause",
            "license_family": "BSD",
            "size": 69263,
            "timestamp": 1723817629767
          },
          "type": "conda"
        },
// ...
```

## Install

```bash
pixi global install pixi-diff
```

You can also use this tool in a temporary environment using `pixi exec`:

```bash
pixi exec pixi-diff pixi.lock.old pixi.lock.new
```

## Usage

```text
Generate JSON diffs between pixi lockfiles

Usage: pixi-diff [OPTIONS] [BEFORE] [AFTER]

Arguments:
  [BEFORE]  First lockfile to be compared
  [AFTER]   Second lockfile to be compared

Options:
  -b, --before <BEFORE>                First lockfile to be compared
  -a, --after <AFTER>                  Second lockfile to be compared
      --manifest-path <MANIFEST_PATH>  Pixi manifest file. Used to determine whether a dependency is explicit
  -v, --verbose...                     Increase logging verbosity
  -q, --quiet...                       Decrease logging verbosity
  -h, --help                           Print help
  -V, --version                        Print version
```

You can either specify both the old and new lockfile via CLI parameters:

```bash
pixi-diff pixi.lock.old pixi.lock.new
# or equivalently
pixi-diff --before pixi.lock.old --after pixi.lock.new
```

Or specify either the "before" or "after" lockfile via stdin:

```bash
pixi-diff --after pixi.lock <(git show HEAD~20:pixi.lock)
# or equivalently
git show HEAD~20:pixi.lock | pixi-diff --after pixi.lock --before -
```

You can specify the manifest path (this tool tries out `pixi.toml` and `pyproject.toml` if not specified) to add `explicit: true/false` to your JSON diff.
If no manifest file is found, `explicit: true/false` is not added to the diff.

```bash
pixi-diff pixi.lock.old pixi.lock.new --manifest-path pixi.toml
```

### pixi-diff-to-markdown

This tool integrates with [pixi-diff-to-markdown](https://github.com/pavelzw/pixi-diff-to-markdown).
You can pass this tool's stdout to `pixi-diff-to-markdown` and generate markdown diffs this way.

```bash
$ git show HEAD~20:pixi.lock | pixi exec pixi-diff --after pixi.lock | pixi exec pixi-diff-to-markdown
# Explicit dependencies
...
```

#### View with md-tui

You can view this generated markdown file in your terminal using [md-tui](https://github.com/henriklovhaug/md-tui) (available on [conda-forge](https://prefix.dev/channels/conda-forge/packages/md-tui)).

```bash
git show HEAD~20:pixi.lock | pixi exec pixi-diff --after pixi.lock | pixi exec pixi-diff-to-markdown > diff.md
pixi exec -s md-tui -- mdt diff.md
```

Two issues with this approach:

- `md-tui` does not support bold and italic links ([henriklovhaug/md-tui #91](https://github.com/henriklovhaug/md-tui/issues/91))
- `md-tui` does not support reading from stdin ([henriklovhaug/md-tui #167](https://github.com/henriklovhaug/md-tui/issues/167))
