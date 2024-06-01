%.html: %.md
	pandoc --to=revealjs --incremental --embed-resources --standalone --slide-level=2 --output=$@ $<

.PHONY: fuzz
watch:
	@echo "Watching for changes..."
	@cargo watch --watch presentation.md --shell "make presentation.html"

.PHONY: fuzz
fuzz:
	cargo fuzz run binary_search
