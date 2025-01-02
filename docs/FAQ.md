# FAQ

## Encountering System Security Policy on First Run of Katharsis on macOS

When you first run Katharsis on macOS, you might see a system security warning that looks like this:

![Not Opened](../assets/faq/not_opened.png)

This happens because the Katharsis binary was not signed with an [Apple Developer Certificate](https://developer.apple.com/support/certificates) during its build process in the [Actions](https://github.com/kurosakishigure/katharsis/actions/workflows/cd.yml).

> We plan to use artifact attestations to establish provenance for builds. For more details, see [#73](https://github.com/kurosakishigure/katharsis/issues/73).

### Plan 1

You can use the following operations:

![Allow Anyway](../assets/faq/allow_anyway.png)

### Plan 2

You can refer to the **Cargo** section in [USE.md](https://github.com/kurosakishigure/katharsis/blob/canary/docs/USE.md#linux-and-macos) to install Katharsis.  

> Please **ensure** that the Cargo package manager is installed.

### Plan 3

If you prefer, you can also choose to install from source. For detailed instructions, please refer to [INSTALL.md](INSTALL.md).

> In the future, we plan to address this problem by enabling installation of Katharsis through package managers like [brew](https://brew.sh). If you're interested in helping us implement this feature, you can contribute by following the guidelines in [CONTRIBUTING.md](../.github/CONTRIBUTING.md).
