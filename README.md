# Semver CLI

This is a clone of the (semver tool in node)[https://docs.npmjs.com/misc/semver] using the
(Rust semver lib by Steve Klabnik)[https://github.com/steveklabnik/semver].

## Goal

I do not want to rely on installed node to use the semver tool. Because I use that tool to
select a node version to use.

I only implemented the needed API for my needs: matching versions against a range.

```
-$ semver-cli -r '>=1.0.1, <1.2.0' 1.0.1 1.0.2 1.1.1 1.3.0
1.0.1
1.0.2
1.1.1
-$ semver-cli -r '~1.0.1' 1.0.1 1.0.2 1.1.1 1.3.0
1.0.1
1.0.2
```

## Usage

```
semver-cli 0.1.0
Julien Durillon <julien@durillon.xyz>
Prints valid versions sorted by SemVer precedence

USAGE:
    semver-cli [OPTIONS] [VERSION]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --range <range>

ARGS:
    <VERSION>...
```

As you can see, I did not implement any fancy `--increment` stuff here. Because I don't
need it. If you want this feature, PR welcome!
