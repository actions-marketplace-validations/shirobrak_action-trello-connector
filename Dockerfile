FROM alpine:latest

WORKDIR /app

COPY bin/x86_64-unknown-linux-musl/action-trello-connector action-trello-connector

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
