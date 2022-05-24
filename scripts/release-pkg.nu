#!/usr/bin/env nu

# Author: hustcer
# Created: 2022/05/24 17:05:20
# Description:
#   A script to do the github release task, need nushell to be installed.
#

let bin = 'musd'
let os = $env.OS
let target = $env.TARGET
# Repo source dir like `/home/runner/work/musd/musd`
let src = $env.GITHUB_WORKSPACE
let dist = $'($env.GITHUB_WORKSPACE)/dist'
let version = (open Cargo.toml | get package.version)

$'Packaging ($bin) v($version) for ($target) in ($src)...'
if not ('Cargo.lock' | path exists) {
    cargo generate-lockfile
}

$'Building ($bin)...'

# Fix OpenSSL related issues on Ubuntu
if $os == 'ubuntu-latest' {
    # musl-tools to fix 'Failed to find tool. Is `musl-gcc` installed?'
    sudo apt install musl-tools -y

    cd /usr/share
    wget https://www.openssl.org/source/openssl-1.1.1o.tar.gz
    tar xzf openssl-1.1.1o.tar.gz; cd openssl-1.1.1o

    let-env OPENSSL_LIB_DIR = '/usr/share/openssl-1.1.1o/'
    let-env OPENSSL_INCLUDE_DIR = '/usr/share/openssl-1.1.1o/include'
    if $target == 'aarch64-unknown-linux-gnu' {
        sudo apt-get install gcc-aarch64-linux-gnu -y
        let configure = (./config shared no-asm no-async --cross-compile-prefix=aarch64-linux-gnu- | complete)
        print ($configure | get stderr)
        # Remove `-m64` string in Makefile
        sed '/-m64/d' Makefile | save Makefile.bk; mv Makefile.bk Makefile
        let make = (make | complete); print ($make | get stderr)
        # This is very important here, Otherwise will cause `error adding symbols: file in wrong format`
        let-env CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = 'aarch64-linux-gnu-gcc'
        cd $src; cargo rustc --bin $bin --target $target --release

    } else if $target == 'armv7-unknown-linux-gnueabihf' {
        sudo apt-get install pkg-config gcc-arm-linux-gnueabihf -y
        let configure = (./config shared no-asm no-async --cross-compile-prefix=arm-linux-gnueabihf- | complete)
        print ($configure | get stderr)
        sed '/-m64/d' Makefile | save Makefile.bk; mv Makefile.bk Makefile
        let make = (make | complete); print ($make | get stderr)
        let-env CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER = 'arm-linux-gnueabihf-gcc'
        cd $src; cargo rustc --bin $bin --target $target --release

    } else {
        let configure = (./config shared | complete); print ($configure | get stderr)
        let make = (make | complete); print ($make | get stderr)
        cd $src; cargo rustc --bin $bin --target $target --release
    }
}

if $os == 'windows-latest' {
    cd $src; cargo rustc --bin $bin --target $target --release -- -C target-feature="+crt-static"
}

let suffix = if $os == 'windows-latest' { '.exe' } else { '' }
let executable = $'target/($target)/release/($bin)($suffix)'

$'Current executable file: ($executable)'
$'Copying release files...'
cd $src; mkdir $dist
echo [LICENSE README.md Cargo.lock Cargo.toml CHANGELOG.md README.zh-CN.md $executable] | each {|it| cp $it $dist }
cd $dist; $'Creating release archive...'

if $os in ['ubuntu-latest', 'macos-latest'] {
    let archive = $'($dist)/($bin)-($version)-($target).tar.gz'
    tar czf $archive *
    print $'archive: ---> ($archive)'; ls $archive
    # It's weird that `^echo $'::set-output name=archive::($archive)'` doesn't work
    echo $archive
} else if $os == 'windows-latest' {
    let pwd = (pwd -W)
    let archive = $'($dist)/($bin)-($version)-($target).zip'
    7z a $archive *
    print $'archive: ---> ($archive)'; ls $archive
    echo $'($pwd)/($bin)-($version)-($target).zip'
}
