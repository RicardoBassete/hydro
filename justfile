# List all recipes
list:
	just --list --unsorted

# Runs the program with the correct path to the main.hy file
run:
	@cargo run -- main.hy