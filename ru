DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
docker build -q --tag rust-watcher:latest "$DIR"/watcher/
docker run -it --rm --user "$(id -u)":"$(id -g)" -e USER=$LOGNAME -v "$PWD":/usr/src/myapp -v "$PWD"/registry:/usr/local/cargo/registry -w /usr/src/myapp rust-watcher:latest "$@"
