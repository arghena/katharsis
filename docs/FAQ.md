# FAQ

## Encountering System Security Policy on First Run of Katharsis on macOS

When you first run Katharsis on macOS, you might see a system security warning that looks like this:

![Not Opened](../assets/faq/not_opened.png)

This happens because the Katharsis binary was not signed with an [Apple Developer Certificate](https://developer.apple.com/support/certificates) during its build process in the [Actions](https://github.com/kurosakishigure/katharsis/actions/workflows/cd.yml).

### Plan 1

You can use the following operations:

![Allow Anyway](../assets/faq/allow_anyway.png)

### Plan 2

Do not use the [Manual](https://github.com/kurosakishigure/katharsis/blob/canary/docs/INSTALL.md#manual) installation method.

## Encountering the `no version matching requirement '*'` Error with `Cargo B(inary)Install`

This issue occurs because Katharsis has not released any stable versions yet. To install Katharsis, use the following command:

```bash
cargo-binstall katharsis@CANARY_VERSION
```
