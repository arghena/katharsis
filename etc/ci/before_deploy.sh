#!/usr/bin/env bash

# TODO: Migrate to `cargo-dist`.
# https://github.com/axodotdev/cargo-dist

# This script is adapted from the release script of the `delta` project by Dan Davison (MIT License).

set -euxo pipefail

pack() {
    local tempdir
    local out_dir
    local package_name

    tempdir=$(mktemp -d 2> /dev/null || mktemp -d -t tmp)
    out_dir=$(pwd)
    package_name="$PROJECT_NAME-$GITHUB_REF_NAME-$TARGET"

    mkdir "$tempdir/$package_name"

    cp "target/$TARGET/release/$PROJECT_NAME" "$tempdir/$package_name/"

    cp README.md "$tempdir/$package_name"
    cp LICENSE "$tempdir/$package_name"

    pushd "$tempdir"
    if [[ $TARGET == *windows* ]]; then
        7z a "$out_dir/$package_name.zip" "$package_name"/*
    else
        tar czf "$out_dir/$package_name.tar.gz" "$package_name"/*
    fi
    popd
    rm -r "$tempdir"
}

make_deb() {
    local tempdir
    local architecture
    local version
    local dpkgname
    local conflictname
    local homepage
    local maintainer

    homepage="https://github.com/arghena/katharsis"
    maintainer="Shigure Kurosaki <shigure@hqsy.net>"
    copyright_years="2024 - "$(date "+%Y")

    case $TARGET in
        x86_64*)
            architecture=amd64
            library_dir=""
            ;;
        i686*)
            architecture=i386
            library_dir=""
            ;;
        aarch64*)
            architecture=arm64
            library_dir=""
            ;;
        arm*hf)
            architecture=armhf
            library_dir="-l/usr/arm-linux-gnueabihf/lib"
            ;;
        *)
            echo "make_deb: skipping target '${TARGET}'" >&2
            return 0
            ;;
    esac
    version=$(echo "$GITHUB_REF_NAME" | sed -e 's/^v//' -e 's/-canary/~canary/')

    if [[ $TARGET == *musl* ]]; then
        dpkgname=$PROJECT_NAME-musl
        conflictname=$PROJECT_NAME
    else
        dpkgname=$PROJECT_NAME
        conflictname=$PROJECT_NAME-musl
    fi

    tempdir=$(mktemp -d 2> /dev/null || mktemp -d -t tmp)

    install -Dm755 "target/$TARGET/release/$PROJECT_NAME" "$tempdir/usr/bin/$PROJECT_NAME"

    # HACK: Work out shared library dependencies.
    mkdir "./debian"
    touch "./debian/control"
    depends="$(dpkg-shlibdeps $library_dir -O "$tempdir/usr/bin/$PROJECT_NAME" 2> /dev/null | sed 's/^shlibs:Depends=//')"
    rm -rf "./debian"

    install -Dm644 README.md "$tempdir/usr/share/doc/$dpkgname/README.md"
    cat > "$tempdir/usr/share/doc/$dpkgname/copyright" << EOF
Format: http://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: $PROJECT_NAME
Source: $homepage

Files: *
Copyright: $copyright_years $maintainer
License: MIT

License: MIT
 Permission is hereby granted, free of charge, to any
 person obtaining a copy of this software and associated
 documentation files (the "Software"), to deal in the
 Software without restriction, including without
 limitation the rights to use, copy, modify, merge,
 publish, distribute, sublicense, and/or sell copies of
 the Software, and to permit persons to whom the Software
 is furnished to do so, subject to the following
 conditions:
 .
 The above copyright notice and this permission notice
 shall be included in all copies or substantial portions
 of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
 ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
 SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 DEALINGS IN THE SOFTWARE.
EOF
    chmod 644 "$tempdir/usr/share/doc/$dpkgname/copyright"

    mkdir "$tempdir/DEBIAN"
    cat > "$tempdir/DEBIAN/control" << EOF
Package: $dpkgname
Version: $version
Section: utils
Priority: optional
Maintainer: Shigure Kurosaki <shigure@hqsy.net>
Architecture: $architecture
Depends: $depends
Conflicts: $conflictname
Description: CLI tool for generating RSS feeds.
EOF

    fakeroot dpkg-deb -Zxz --build "$tempdir" "${dpkgname}_${version}_${architecture}.deb"
}

main() {
    pack
    if [[ $TARGET == *linux* ]]; then
        make_deb
    fi
}

main
