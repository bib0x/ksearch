import "list"

#Cheat: {
    description: string
    data: [...string]
    tags: [...string]
}

[
  #Cheat & { 
    description: "show routes", 
    data: [ 
      "netstat -rWn", 
    ] 
    tags: [
      "netstat", 
    ] 
  },
  #Cheat & { 
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
