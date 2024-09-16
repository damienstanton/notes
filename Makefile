default: fs
.PHONY: book fs

book:
	@mdbook serve --open

fs:
	@dotnet fsi lab/${lab}.fsx