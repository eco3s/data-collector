# cargo executable
cargo := 'cargo'

# debug or release mode which will used for contexts like build or run etc.
# and this is the error message for that
mode_error := '"mode" must be either "debug" or "release"'

# default cache directory
download_dir := './downloads'

# default export directory
export_dir := './out'

# build the binary crate in given mode
build mode = 'debug':
	# TODO: remove repetition
	{{cargo}} build {{ if mode == 'debug' { '' } else if mode =~ '^r' { '--release' } else { error(mode_error) } }}

alias b := build

# alias for recipes which has parameters
br: (build 'release')

# build and then run the executable
run mode = 'debug':
	{{cargo}} run {{ if mode == 'debug' { '' } else if mode =~ '^r' { '--release' } else { error(mode_error) } }}

alias r := run

# alias for recipes which has parameters
rr: (run 'release')

# run all tests without capturing
test:
	{{cargo}} test -- --nocapture

alias t := test

# delete all build artifacts
clean:
	{{cargo}} clean

alias c := clean

# format the entire codebase
format:
	{{cargo}} fmt

alias f := format

# checks for code qualities
lint:
	{{cargo}} clippy

alias l := lint

# delete all downloaded cache and parsed output
delete:
	@if [ -d {{download_dir}} ]; then \
		rm -r {{download_dir}}; \
		echo "successfully deleted download directory located at {{download_dir}}"; \
	else \
		echo "'{{download_dir}}' already does not exists"; \
	fi

alias d := delete

# run pre-hook manually
pre:
	pre-commit run --all-files

alias p := pre

# generate changelog file
cliff file = './CHANGELOG.md':
	git cliff --output {{file}} --verbose
