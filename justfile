# cargo executable
cargo := 'cargo'

# default cache directory
download_dir := './downloads'

# default export directory
export_dir := './out'

# debug or release profile which will be used for contexts like build or run etc.
# and this is the error message for that
profile_error := '"profile" must be either "debug" or "release"'

# default package in which cargo run commands
p := 'cli' # alias
package := p

# generate option from variable
package_option := if package == '' {
	'--workspace'
} else {
	'--package ' + package
}

# build the binary crate in given profile
build profile = 'debug':
	@# TODO: remove repetition
	{{cargo}} build {{package_option}} {{ if profile =~ '^d' { '' } else if profile =~ '^r' { '--release' } else { error(profile_error) } }}

alias b := build

# alias for recipes which has parameters
br: (build 'release')

# build and then run the executable
run profile = 'debug':
	{{cargo}} run {{package_option}} {{ if profile =~ '^d' { '' } else if profile =~ '^r' { '--release' } else { error(profile_error) } }}

alias r := run

# alias for recipes which has parameters
rr: (run 'release')

# run all tests without capturing
test:
	{{cargo}} test {{package_option}} -- --nocapture

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
		rm {{download_dir / '*'}} \
		&& echo "successfully deleted all contents of {{download_dir}}" \
		|| true; \
	else \
		echo "'{{download_dir}}' already does not exists"; \
	fi
	@if [ -d {{export_dir}} ]; then \
		rm {{export_dir / '*'}} && \
		echo "successfully deleted all contents of {{export_dir}}" \
		|| true; \
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
