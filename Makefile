# SPDX-License-Identifier: MIT

all:
	cargo test
	cargo clippy

publish:
	cargo publish -p "tagged-types-derive"
	cargo publish -p "tagged-types"
