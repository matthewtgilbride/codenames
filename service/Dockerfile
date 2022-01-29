FROM ubuntu

ARG ARCH

# weird: https://github.com/roapi/roapi/issues/103
RUN apt-get update
RUN apt-get install ca-certificates -y

COPY target/${ARCH}-unknown-linux-musl/release/codenames-actix /

CMD ["/codenames-actix"]
