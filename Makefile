# SPDX-License-Identifier: MIT

all:
	cargo test
	cargo clippy
	cargo doc

publish: all
	cargo publish -p "tagged-types-derive"
	cargo publish -p "tagged-types"
