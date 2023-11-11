package main

import (
	"bib0x.github.com/schema"
)

[
	schema.#Knowledge & {
		description: "show routes"
		data: [
			"netstat -rWn",
		]
		tags: [
			"netstat",
		]
	},
	schema.#Knowledge & {
		description: "show rules"
		data: [
			"pfctl -sr",
		]
		tags: [
			"pfctl",
			"firewall",
		]
	},
]
