# Use the Katharsis

## Example

> [!NOTE]
> You can refer to our [examples](../examples).

Run the following command to generate a `katharsis.config.toml` file in the current directory:

```bash
katharsis init
```

Run the following command to create a default `rss.xml`:

```bash
katharsis
```

## Command Overview

| Command | Description                                                        |
| ------- | ------------------------------------------------------------------ |
| init    | Generates a `katharsis.config.toml` file in the current directory. |
| help    | Displays help information.                                         |

## Parameter Overview

| Parameter | Description                                                    |
| --------- | -------------------------------------------------------------- |
| -c        | Specifies a `katharsis.config.toml` file as the configuration. |
| -h        | Displays help information.                                     |
| -V        | Displays the current version of Katharsis.                     |

> [!NOTE]
> If you do not specify the `-c` parameter, the `katharsis.config.toml` file in the current directory will be used by default.

## Explanation of `katharsis.config.toml` Fields

You can refer to the [RSS 2.0 at Harvard Law](https://cyber.harvard.edu/rss/rss.html) for more detailed documentation.

### rss

| Field       | Description                        |
| ----------- | ---------------------------------- |
| title       | The title of the website.          |
| description | A description of the website.      |
| site_url    | The main URL of the website.       |
| image       | Path to the channel's logo.        |
| copyright   | Copyright information.             |
| language    | Preferred language.                |
| output      | Path to the local output RSS file. |

> [!NOTE]
>
> - The `site_url` field should not include a trailing slash (for example, `https://example.com` rather than `https://example.com/`).
> - The `image` field is relative to the `site_url`, for example, `favicon.png` corresponds to `https://example.com/favicon.png`.
> - The `output` field specifies the path relative to the working directory (for example, `rss.xml` corresponds to `./rss.xml`).

### article

| Field       | Description                                                |
| ----------- | ---------------------------------------------------------- |
| title       | The tag containing the article's title.                    |
| description | The tag or attribute containing the article's description. |
| input       | The HTML files that needs to be parsed.                    |
| author      | Information about the article's author.                    |
| link        | The prefix for the article's URL.                          |
| content     | The tag or attribute containing the article's content.     |
| date        | The tag containing the article's publication date.         |
| image       | The image files to be used as the article cover.           |
| sort        | Whether to sort articles by their publication date.        |

> [!NOTE]
>
> - The `input` field corresponds to the file stem(`*`), which is the same as the article's URL slug.
> - The `date` field's tag must include a [datetime](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time) attribute, and the attribute value must follow the `%Y-%m-%d` format.
> - The `image` field’s folder name(`**`) must match the article's URL slug.
