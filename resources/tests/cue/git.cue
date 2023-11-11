package main

import (
	"bib0x.github.com/schema"
)

[
	schema.#Knowledge & {
		description: "quick show branch and file changes"
		data: [
			"git status -s -b",
		]
		tags: [
			"git-status",
		]
	},
	schema.#Knowledge & {
		description: "quick show submodules status"
		data: [
			"git submodule status",
		]
		tags: [
			"git-submodule",
		]
	},
	schema.#Knowledge & {
		description: "search for terms/string in commit message history"
		data: [
			"git log --all --grep='<message>'",
			"git log --grep='<message>'",
			"git log --author='<username>'",
		]
		tags: []
	},
]
