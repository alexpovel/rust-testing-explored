%.html: %.md
	pandoc --to=revealjs --incremental --embed-resources --standalone --slide-level=2 --output=$@ $<

watch:
	@echo "Watching for changes..."
	@cargo watch --watch presentation.md --shell "make presentation.html"