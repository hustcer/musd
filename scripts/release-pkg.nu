#!/usr/bin/env nu

# Author: hustcer
# Created: 2022/05/24 17:05:20
# Description:
#   A script to do the github release task, need nushell to be installed.
# REF:
#   1. https://github.com/volks73/cargo-wix

use common.nu [hr-line, get-env]

# The binary file to be released
let bin = 'musd'
let os = $env.OS
let target = $env.TARGET
# Repo source dir like `/home/runner/work/musd/musd`
let src = $env.GITHUB_WORKSPACE
let flags = $env.TARGET_RUSTFLAGS
let dist = $'($env.GITHUB_WORKSPACE)/dist'
let version = (open Cargo.toml | get package.version)

print $'Packaging ($bin) v($version) for ($target) in ($src)...'; hr-line -b
if not ('Cargo.lock' | path exists) {
    cargo generate-lockfile
}

print $'Start building ($bin)...'; hr-line

# ---------------------------------------------------------------------------------
# Fix OpenSSL related issues on Ubuntu and then build the release binary
# ---------------------------------------------------------------------------------
if $os == 'ubuntu-latest' {
    if $target == 'aarch64-unknown-linux-gnu' {
        sudo apt-get install gcc-aarch64-linux-gnu -y
        # This is very important here, Otherwise will cause `error adding symbols: file in wrong format`
        let-env CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = 'aarch64-linux-gnu-gcc'
        cargo-build-musd $flags

    } else if $target == 'armv7-unknown-linux-gnueabihf' {
        sudo apt-get install pkg-config gcc-arm-linux-gnueabihf -y
        let-env CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER = 'arm-linux-gnueabihf-gcc'
        cargo-build-musd $flags

    } else {
        # musl-tools to fix 'Failed to find tool. Is `musl-gcc` installed?'
        sudo apt install musl-tools -y
        cargo-build-musd $flags
    }
}

# ---------------------------------------------------------------------------------
# Build for macOS
# ---------------------------------------------------------------------------------
if $os in ['macos-latest'] { cargo-build-musd $flags }

# ---------------------------------------------------------------------------------
# Build for Windows
# ---------------------------------------------------------------------------------
if $os in ['windows-latest'] { cargo rustc --bin $bin --target $target --release }

# ---------------------------------------------------------------------------------
# Prepare for the release archive
# ---------------------------------------------------------------------------------
let suffix = if $os == 'windows-latest' { '.exe' } else { '' }
let executable = $'target/($target)/release/($bin)($suffix)'
print $'Current executable file: ($executable)'
print $'Copying release files...'
cd $src; mkdir $dist
echo [LICENSE README* CHANGELOG.md $executable] | each {|it| cp -r $it $dist }

print $'(char nl)Dist directory contents:'; hr-line;
cd $dist; ls -f

print $'(char nl)Check binary release build detail:'; hr-line;
let info = if $os == 'windows-latest' {
    # Use `.\musd.exe` works, but `./musd.exe` doesn't
    (do -i { .\musd.exe -b }) | str join
} else {
    (do -i { ./musd -b }) | str join
}
if ($info | str trim | empty?) {
    print $'(ansi r)Incompatible nu binary...(ansi reset)'
} else { $info }

# ---------------------------------------------------------------------------------
# Create a release archive and send it to output for the following steps
# ---------------------------------------------------------------------------------
print $'(char nl)Creating release archive...'; hr-line
if $os in ['ubuntu-latest', 'macos-latest'] {

    let archive = $'($dist)/($bin)-($version)-($target).tar.gz'
    tar czf $archive *
    print $'archive: ---> ($archive)'; ls $archive
    echo $'::set-output name=archive::($archive)'

} else if $os == 'windows-latest' {

    let releaseStem = $'($bin)-($version)-($target)'

    if (get-env _EXTRA_) == 'msi' {
        # Create Windows msi release package
        print $'Start creating Windows msi package...'
        cd $src; hr-line -b
        mkdir target/release; cp $executable target/release/
        let wixRelease = $'($src)/target/wix/($releaseStem).msi'
        cargo install cargo-wix --version 0.3.2
        cargo wix init
        cargo wix --no-build --nocapture --output $wixRelease
        echo $'::set-output name=archive::($wixRelease)'

    } else {

        let archive = $'($dist)/($releaseStem).zip'
        7z a $archive *
        print $'archive: ---> ($archive)';
        let pkg = (ls -f $archive | get name)
        if not ($pkg | empty?) {
            echo $'::set-output name=archive::($pkg | get 0)'
        }
    }
}

def 'cargo-build-musd' [ options: string ] {
    if ($options | str trim | empty?) {
        cargo rustc --bin $bin --target $target --release --features=static-link-openssl
    } else {
        cargo rustc --bin $bin --target $target --release --features=static-link-openssl $options
    }
}
