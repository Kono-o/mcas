# mcas

A minimal CLI to fetch and download vanilla Minecraft texture packs by version.

## Usage

```
mcas get <version>
mcas get <version> <dir>
mcas get <version> <+dir>
```

### Examples

```
mcas get 1.16.4
mcas get 1.16.4 save/pack/here
mcas get 1.16.4 +extended/path
```

### Help

```
mcas help
mcas info
```

## Install

### From crates.io

```
cargo install mcas
```

### From source

```
git clone https://github.com/yourname/mcas
cd mcas
cargo install --path .
```
