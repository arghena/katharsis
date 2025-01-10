# Installation

## Package Managers

[![Packaging status](https://repology.org/badge/vertical-allrepos/katharsis.svg)](https://repology.org/project/katharsis/versions)

### [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget)

```bash
winget install kurosakishigure.katharsis
```

### [Cargo B(inary)Install](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo-binstall katharsis
```

### [Paru](https://github.com/Morganamilo/paru)

```bash
paru -S katharsis
```

### [Apt](https://wiki.debian.org/AptCLI)

```bash
sudo apt install ~/your/download/path/katharsis.deb
```

## Manual

```bash
# Extract the tarball
tar -xzf ~/your/download/path/katharsis.tar.gz

# Move the binary to the appropriate directory
mv ~/your/extract/path/katharsis /usr/local/bin
```

## Source

```bash
# Clone the repository
git clone --filter=blob:none --branch BRANCH_NAME --single-branch https://github.com/kurosakishigure/katharsis.git

# Install the project
cd katharsis && cargo build --release

# Move the binary to the appropriate directory
mv target/release/katharsis /usr/local/bin
```
