#!make
include server/.env
.DEFAULT_GOAL := default

default:
	@echo "Please specify a target to make."

run:
	@echo "Running..."
	cd server && cargo run
