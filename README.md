findrun.exe
===========

On Windows, interpreters typically do **not** search `%PATH%` for scripts, so you normally have to specify the script's full path.  
`findrun` is a wrapper that automatically resolves the script's full path and executes it with the specified interpreter.

Usage
-----

```
findrun [-d] INTERPRETER [OPTIONS] SCRIPT [ARGS...]
```

The first argument that meets the following conditions is considered the script:

- Does **not** contain path separators (`/` or `\`) — i.e., it is just a file name
- Contains a dot (has a file extension)

With the `-d` option, the command line that would be executed is printed instead of actually running it.

### Example

```
findrun gawk -f foo.awk A B C
```

* Here, `foo.awk` is recognized as the script.
* The actual command executed will be:

```
gawk -f C:\path\to\foo.awk A B C
```
