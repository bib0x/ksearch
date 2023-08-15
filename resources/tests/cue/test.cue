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
