# Installation

## Package Managers

### Cargo

```bash
cargo binstall katharsis
# or
cargo binstall katharsis@1.0.0-canary.24
```

### Paru

```bash
paru -S katharsis
```

### Apt

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
git clone --filter=blob:none --branch canary --single-branch https://github.com/kurosakishigure/katharsis.git

# Install the project
cd katharsis && cargo build --release

# Move the binary to the appropriate directory
mv target/release/katharsis /usr/local/bin
```
