# Use the Katharsis

> [!NOTE]
> Cargo does not support directly installing canary versions. If you need to install a canary version, please run the following commands:
> 
> ```bash
> cargo install katharsis@1.0.0-canary.21
> # or
> cargo binstall katharsis@1.0.0-canary.21
> ```

## Prerequisites

### Linux and macOS

First, download the appropriate binary from the [Releases](https://github.com/kurosakishigure/katharsis/releases/latest) page.

> You can find the platform comparison table in the [Platform Support](https://doc.rust-lang.org/rustc/platform-support.html) section.

If you're using a Debian-based system, you can directly install the .deb file from the download directory using apt:

```bash
sudo apt install katharsis.deb
```

If you're using Cargo, please run:

```bash
cargo install katharsis
# or
cargo binstall katharsis
```

> - You might also need to add the environment variable `export PATH=$HOME/.cargo/bin:$PATH` to your `.zshrc` or `.bashrc` file.
> - If you choose to use `cargo binstall katharsis`, you might need to run `brew install cargo-binstall` first.

Otherwise, you'll need to follow the step-by-step instructions below to install Katharsis.

Run the following command to extract the downloaded file:

```bash
tar -xzf ~/your/download/path/katharsis.tar.gz
```

Next, execute the following command in your home directory to create the necessary folders:

```bash
mkdir -p Formulae/bin
```

Move the downloaded binary to ~/Formulae/bin:

```bash
mv ~/your/extract/path/katharsis ~/Formulae/bin/katharsis
```

Modify your .zshrc or .bashrc file by adding the following configuration:

```bash
# Formulae
export PATH=$HOME/Formulae/bin:$PATH
```

> Typing `katharsis` might be cumbersome, so you can use a shorter alias like `rss` in your .zshrc or .bashrc file instead:
>
> ```bash
> alias rss=katharsis
> ```

Run the following command to apply the changes:

```bash
source .zshrc
# or
source .bashrc
```

## Example

> You can refer to our [Examples](../examples).

Run the following command to generate a `katharsis.config.toml` file in the current directory:

```bash
katharsis init
```

Run the following command to create a default `rss.xml`:

```bash
katharsis
```

For a detailed explanation of the fields in the `katharsis.config.toml` file, refer to the Overview section below.

## Command Overview

| Command | Description                                                      |
|---------|------------------------------------------------------------------|
| init    | Generates a katharsis.config.toml file in the current directory. |
| help    | Displays help information.                                       |

## Parameter Overview

| Parameter | Description                                                  |
|-----------|--------------------------------------------------------------|
| -c        | Specifies a katharsis.config.toml file as the configuration. |
| -h        | Displays help information.                                   |
| -V        | Displays the current version of Katharsis.                   |

> If you do not specify the `-c` parameter, the `katharsis.config.toml` file in the current directory will be used by default.

## Explanation of katharsis.config.toml Fields

You can refer to the [RSS 2.0 at Harvard Law](https://cyber.harvard.edu/rss/rss.html) for more detailed documentation.

### rss

| Field       | Description                        |
|-------------|------------------------------------|
| title       | The title of the website.          |
| description | A description of the website.      |
| site_url    | The main URL of the website.       |
| image       | Path to the channel's logo.        |
| copyright   | Copyright information.             |
| language    | Preferred language.                |
| output      | Path to the local output RSS file. |

> Important Notes:
>
> - The site_url field should not include a trailing slash (e.g., https://example.com rather than https://example.com/).
> - The image field is relative to the site_url, for example, favicon.png corresponds
    to https://example.com/favicon.png.
> - The output field specifies the path relative to the working directory (e.g., rss.xml corresponds to ./rss.xml).

### article

| Field       | Description                                                |
|-------------|------------------------------------------------------------|
| title       | The tag containing the article's title.                    |
| description | The tag or attribute containing the article's description. |
| input       | The file(s) to be used as input, matching specified rules. |
| author      | Information about the article's author.                    |
| link        | The prefix for the article's URL.                          |
| content     | The tag or attribute containing the article's content.     |
| date        | The tag containing the article's publication date.         |
| image       | File(s) to be used as the article's cover image.           |
| sort        | Whether to sort articles by their publication date.        |

> Important Notes:
>
> - The input field corresponds to the file body, which is the same as the article's URL slug.
> - The date field's tag must include a [datetime](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time) attribute, and the attribute value must follow the `%Y-%m-%d` format.
> - The image field’s folder name must match the article's URL slug.
