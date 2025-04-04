# Contribution Guidelines

## Branching Scheme

This repository employs a [trunk-based development](https://trunkbaseddevelopment.com/): development occurs on `development` branch with short-lived branches that merges into it.
When a release is met, for example a `v0.3.2` release, a `v0.3.x` branch is spun of `development` and the release tagged. If this branch already exists, relevant commits are back-ported instead.

> [!INFO]
> We don't literally stick to it as the pace is too slow to justify branching for each release.

Development is done towards `development`.

### Working branches naming scheme

While we don't care how you name branches in your forks, we encourage you to branch not PR `development` from your fork, however, if you're a contributor working on this repo directly we strongly
advice to name your branches with following format `issue/99/short-kebab-case-name`, where `99` is the number of issue you're trying to fix or address as enhancement. The work `issue` may be
replaced by `bugfix` if you're fixing a bug, or `feature` if you're working on enhancement.

## Developing with PAC changes aside

PACs used by HAL in this repository now reside in [atsamx7x-pac](https://github.com/atsams-rs/atsamx7x-pac/) repository. Crates are published from there to crates.io, and released versions
are normally enough to work with HAL.

However, if you're in need to incorporate some changes related to unreleased PACs, own variants of PACs, or you fix broken HAL after `svd2rust` generated breaking changes, you have to:
1. Clone the PACs repository to the directory one level above this one:
   ```sh
   git clone git@github.com/atsams-rs/atsamx7x-pac.git ../atsamx7x-pac
   ```

2. Add `[patch.crates]` section to `.cargo/config`, for example:
   ```toml
   [patch.crates-io]
   # This is example for one PAC kind, you may want to repeat for each you're using
   atsamv71q21b = { path = "../../atsamx7x-pac/pac/atsamv71q21b" }
   ```
   or try to run:
   ```sh
   tools/patch-pacs
   ```
   > [!WARN]
   > Please don't commit this change!

   This feature is documented [here](https://doc.rust-lang.org/cargo/reference/config.html#patch).

3. Make changes in there and (re)generate (in this directory; with Python 3 installed):
   ```sh
   pushd ../atsamx7x-pac
   tools/pacs.py svd
   popd
   ```

## Pull Requests

When you submit a **Pull Request** make sure you've described your efforts in `CHANGELOG.md`, otherwise CI will fail.

Code has to be formatted with `cargo rustfmt`, otherwise CI will fail.

