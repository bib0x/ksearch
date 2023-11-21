package main

import (
	"bib0x.github.com/schema"
)

[
	schema.#Knowledge & {
		description: "show routes"
		data: [
			"// small comment",
			"netstat -rWn",
			"// small comment after",
		]
		tags: [
			"netstat",
		]
	},
	schema.#Knowledge & {
		description: "show rules"
		data: [
			"// comment test to see how it goes",
			"// another comment",
			"pfctl -sr",
		]
		tags: [
			"pfctl",
			"firewall",
		]
	},
]
