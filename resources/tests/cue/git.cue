package main

import (
	"bib0x.github.com/schema"
)

[
	schema.#Cheat & {
		description: "quick show branch and file changes"
		data: [
			"git status -s -b",
		]
		tags: [
			"git-status",
		]
	},
	schema.#Cheat & {
		description: "quick show submodules status"
		data: [
			"git submodule status",
		]
		tags: [
			"git-submodule",
		]
	},
]
