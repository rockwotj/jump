# Jump

A small program to move around the filesystem.

Inspired by [teleport](https://github.com/bollu/teleport)

## Install

Install this by `cargo install` in the project directory. You'll need rust nightly installed to compile this. If you're on MacOS, then you can download the binaries from the releases page.

Once you've installed the binaries on your $PATH, you'll want to add the following function to your `bash_alias` or profile.

```bash
function j() {
  OUTPUT=`jump $@`
  if [ $? -eq 0 ]
    then cd "$OUTPUT"
    else echo "$OUTPUT"
  fi
}
```

You can then use the `jump-cfg` binary to set your workspace and jump markers, then the `j` function to go to those markers or directories in your workspace.
