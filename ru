docker run -i --rm --user "$(id -u)":"$(id -g)" -e USER=$LOGNAME -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest "$@"
