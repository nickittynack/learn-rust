FROM rust:latest

RUN apt-get update
RUN apt-get install inotify-tools -y

CMD ["cargo", "check"]
