# ksearch

This program is used to search in JSON generated cheatsheets.

Cheatsheets are defined using CUE language to generate JSON 
files that could be parsed next by this tool.

## Usage

```
$ ksearch -s <term> -t <topic> -f <tags> -m
$ ksearch -t <topic> -p
$ ksearch -e
```

## Todo

- [ ] Tests
- [ ] add comment to cheatsheet ?
- [x] manage CUE/JSON export via CLI
- [ ] manage alias link
