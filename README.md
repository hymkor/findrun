findrun.exe
===========

On Windows, interpreters typically do **not** search `%PATH%` (or similar environment variables) for scripts, so you normally have to specify the script's full path.  
`findrun` is a small wrapper that automatically resolves a script's full path and executes it with the specified interpreter.

By default, `findrun` searches for scripts in the directories listed in `%PATH%`, but you can also specify a different environment variable (such as `AWKPATH`).

Usage
-----

```

findrun [-v ENVNAME] [-d] INTERPRETER [OPTIONS] SCRIPT [ARGS...]

```

The first argument that meets **all** of the following conditions is treated as the script:

- Does **not** contain path separators (`/` or `\`) — i.e. it is just a file name
- Contains a dot (has a file extension)

If the script is found in one of the directories listed in the selected environment variable, it is replaced with its full path before execution.

Options
-------

- `-v ENVNAME`  
  Use the specified environment variable as the search path instead of `%PATH%`.

- `-d`  
  Print the command line that would be executed, without actually running it.

Examples
--------

Search for the script in `%PATH%` (default behavior):

```

findrun gawk -f foo.awk A B C

```

Here, `foo.awk` is recognized as the script, and the actual command executed will be:

```

gawk -f C:\path\to\foo.awk A B C

```

Use a different environment variable (for example, `AWKPATH`) as the search path:

```

findrun -v AWKPATH goawk -f foo.awk A B C

```

In this case, `foo.awk` is searched for in the directories listed in `%AWKPATH%`.
