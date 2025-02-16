<!-- markdownlint-disable MD033 MD041 -->
<!--

NOTE: This file will be bundled into the final product (`*.tar.gz`) when it is released, so URLs (for example, `https://example.com/foo.bar`) should be used consistently within the file to point to project files, rather than local paths (for example, `./foo.bar`). This is because users won't have access to those files locally when browsing the file.

-->

<div align="center">
  <a href="https://github.com/arghena/katharsis">
    <img height="128" src="https://raw.githubusercontent.com/arghena/katharsis/refs/heads/canary/assets/logo.svg" alt="logo" />
  </a>
  <h1>Katharsis</h1>

<a href="https://github.com/arghena/katharsis/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/arghena/katharsis/ci.yml?branch=v1.0.0-canary.29&style=for-the-badge&label=CI&labelColor=363a4f&color=black&logo=github" alt="CI" /></a>
<a href="https://coveralls.io/github/arghena/katharsis"><img src="https://img.shields.io/coverallsCoverage/github/arghena/katharsis?branch=v1.0.0-canary.29&style=for-the-badge&labelColor=363a4f&color=ivory&logo=coveralls" alt="Coverage Status" /></a>
<a href="https://github.com/arghena/katharsis/actions/workflows/cd.yml"><img src="https://img.shields.io/github/actions/workflow/status/arghena/katharsis/cd.yml?branch=v1.0.0-canary.29&style=for-the-badge&label=CD&labelColor=363a4f&color=black&logo=github" alt="CD" /></a>

</div>

<!-- markdownlint-enable MD033 -->

![demo](https://raw.githubusercontent.com/arghena/katharsis/refs/heads/canary/assets/demo.gif)

## Installation

> [!NOTE]
> You can download the prebuilt binary artifacts from the [Releases](https://github.com/arghena/katharsis/releases/latest) page.

### Package Managers

[![Packaging status](https://repology.org/badge/vertical-allrepos/katharsis.svg)](https://repology.org/project/katharsis/versions)

<!-- markdownlint-disable MD033 -->

<details>
  <summary>WinGet</summary>

```bash
winget install arghena.katharsis
```

</details>

<details>
  <summary>Cargo B(inary)Install</summary>

```bash
cargo binstall katharsis@1.0.0-canary.29
```

</details>

<details>
  <summary>Paru</summary>

```bash
paru -S katharsis
```

</details>

<details>
  <summary>Apt</summary>

```bash
curl -fsSLO https://github.com/arghena/katharsis/releases/download/v1.0.0-canary.29/katharsis_1.0.0-canary.29_amd64.deb
sudo apt install katharsis_1.0.0-canary.29_amd64.deb
```

</details>

<!-- markdownlint-enable MD033 -->

### Manual

```bash
curl -fsSL https://github.com/arghena/katharsis/releases/download/v1.0.0-canary.29/katharsis-v1.0.0-canary.29-x86_64-unknown-linux-gnu.tar.gz | tar -xz
mv katharsis-v1.0.0-canary.29-x86_64-unknown-linux-gnu/katharsis /usr/local/bin
```

### Source

```bash
git clone --filter=blob:none --branch canary --single-branch https://github.com/arghena/katharsis.git
cd katharsis && cargo install --frozen --path .
```

## Get Started

> [!NOTE]
> You can refer to our [examples](https://github.com/arghena/katharsis/tree/canary/examples).

Run the following command to generate a `katharsis.config.toml` file in the current directory:

```bash
katharsis init
```

Run the following command to create a default `rss.xml`:

```bash
katharsis
```

## Overview

### Commands

| Command | Description                                                        |
| ------- | ------------------------------------------------------------------ |
| init    | Generates a `katharsis.config.toml` file in the current directory. |
| help    | Displays help information.                                         |

### Parameters

| Parameter | Description                                             |
| --------- | ------------------------------------------------------- |
| -c        | Specifies a `katharsis.config.toml` file as the config. |
| -h        | Displays help information.                              |
| -V        | Displays the current version of Katharsis.              |

> [!NOTE]
> If you do not specify the `-c` parameter, the `katharsis.config.toml` file in the current directory will be used by default.

### Fields

You can refer to the [RSS 2.0 at Harvard Law](https://cyber.harvard.edu/rss/rss.html) for more detailed documentation.

| Field           | Description                        |
| --------------- | ---------------------------------- |
| rss.title       | The title of the website.          |
| rss.description | A description of the website.      |
| rss.site_url    | The main URL of the website.       |
| rss.image       | Path to the channel's logo.        |
| rss.copyright   | Copyright information.             |
| rss.language    | Preferred language.                |
| rss.output      | Path to the local output RSS file. |

> [!NOTE]
>
> - The `rss.site_url` field should not include a trailing slash (for example, `https://example.com` rather than `https://example.com/`).
> - The `rss.image` field is relative to the `rss.site_url`, for example, `favicon.png` corresponds to `https://example.com/favicon.png`.
> - The `rss.output` field specifies the path relative to the working directory (for example, `rss.xml` corresponds to `./rss.xml`).

| Field               | Description                                                |
| ------------------- | ---------------------------------------------------------- |
| article.title       | The tag containing the article's title.                    |
| article.description | The tag or attribute containing the article's description. |
| article.input       | The HTML files that needs to be parsed.                    |
| article.author      | Information about the article's author.                    |
| article.link        | The prefix for the article's URL.                          |
| article.content     | The tag or attribute containing the article's content.     |
| article.date        | The tag containing the article's publication date.         |
| article.image       | The image files to be used as the article cover.           |
| article.sort        | Whether to sort articles by their publication date.        |

> [!NOTE]
>
> - The `article.input` field corresponds to the file stem (`*`), which is the same as the article's URL slug.
> - The `article.date` field's tag must include a [datetime](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time) attribute, and the attribute value must follow the `%Y-%m-%d` format.
> - The `article.image` field’s folder name (`**`) must match the article's URL slug.

## API Docs

Please refer to our [API Docs](https://docs.rs/katharsis).

## Contributing

Please refer to our [CONTRIBUTING.md](https://github.com/arghena/katharsis/blob/canary/.github/CONTRIBUTING.md).

## Security

> [!NOTE]
> If you believe you have found a security vulnerability in Katharsis, we encourage you to **responsibly disclose this and NOT open a public issue**.

Please refer to our [SECURITY.md](https://github.com/arghena/katharsis/blob/canary/.github/SECURITY.md).

## Releases

The version naming of Katharsis follows [Semantic Versioning 2.0.0](https://semver.org/#semantic-versioning-200).

### Long-term Support

> [!NOTE]
> When Katharsis releases a new major version, the previous stable version will be designated as an LTS version.

The LTS version of Katharsis provides security updates and bug fixes for six months.

### Release Table

| Version | Codename | Released at | LTS EOL |
| ------- | -------- | ----------- | ------- |
| 1.0.0   | -        | -           | -       |

## License

Katharsis is distributed under the [MIT](https://github.com/arghena/katharsis/blob/canary/LICENSE) license and provides information about the code of other open-source projects used in the project in the [NOTICE](https://github.com/arghena/katharsis/blob/canary/LICENSES/NOTICE) file.
