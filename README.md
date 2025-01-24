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

```bash
pixi-diff pixi.lock pixi.lock.old
```

## Install

```bash
pixi global install pixi-diff
```

You can also use this tool in a temporary environment using `pixi exec`:

```bash
pixi exec pixi-diff pixi.lock pixi.lock.old
```
