set -e

# keep track of the last executed command
trap 'last_command=$current_command; current_command=$BASH_COMMAND' DEBUG
# echo an error message before exiting
trap 'echo "\"${last_command}\" command failed with exit code $?."' ERR

# rust
echo "patchlogue CI"
echo "Running formatter"
cargo fmt --check --quiet
echo "Running linter"
cargo clippy
echo "Running tests"
cargo test
# www
echo "Running formatting for www"
( cd www ; npx prettier --check *.html )
( cd www ; npx prettier --check *.css )
( cd www ; npx prettier --check *.js )
echo "done"
