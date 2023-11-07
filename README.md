# ksearch

Ksearch stands for `knowledge search`. I am using different kind of place to store knowledge (zettelkasten, Firefox bookmarks, Gist...). The issue is that I tend to forget where I've stored all of this... 

To solve this memory issue, I use this tool to index where I store my stuff.

Cheatsheets are defined using CUE language to generate JSON 
files that could be parsed next by this tool.

## Usage

```
CLI to search knowlege from JSON cheatsheets

Usage: ksearch [OPTIONS]

Options:
  -s, --search <search>  Term to search
  -t, --topic <topic>    Targeted search topic
  -f, --filter <filter>  Search filters such as tags
  -e, --environment      Show environment variable
  -p, --path             Show topic path if exist
  -G, --generate         Generate CUE notes as JSON file
  -h, --help             Print help
```

## Example

```
$ ksearch -s <term> -t <topic> -f <tags>
$ ksearch -t <topic> -p
$ ksearch -e
$ ksearch --generate
```

## Create a resources directory

```
$ pwd
/home/user/dev/git/ksearch_resources/

# Create directories
$ mkdir -p resources/{cue,json}
$ cd resources/cue

# Create cue module with `Cheats` datastructure
$ cue mod init bib0x.github.com
$ mkdir schema
$ cat > schema/cheats.cue <<EOF
package schema

#Cheat: {
    description: string
    data: [...string]
    tags: [...string]
}
EOF

# Create a cheatsheet
$ cat > pfsense.cue <<EOF
package main

import (
  "bib0x.github.com/schema"
)

[
  schema.#Cheat & {
    description: "show routes",
    data: [
      "netstat -rWn",
    ]
    tags: [
      "netstat",
    ]
  },
  schema.#Cheat & {
    description: "show rules",
    data: [
      "pfctl -sr",
    ],
    tags: [
      "pfctl",
      "firewall"
    ]
  },
]
EOF

# Tests: Generate JSON file from CUE
$ KSEARCH_PATH=`pwd`/resources/ ksearch -G
$ ls `pwd`/resources/json/
pfsense.json
```
