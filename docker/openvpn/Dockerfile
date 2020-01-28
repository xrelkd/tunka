### brook builder
FROM golang:1.13-alpine as builder

RUN apk add --update \
    alpine-sdk

WORKDIR /go/src/github.com/txthinking

RUN git clone --branch v20200102 https://github.com/txthinking/brook && \
    cd brook/ && \
    go get -t -v && \
    cd cli/brook && \
    go get -t -v . && \
    CGO_ENABLED=0 GOOS=linux go build -a -ldflags '-extldflags "-static"' -o brook && \
    strip --verbose brook && \
    file brook && \
    cp brook /usr/bin/brook

### tunka-openvpn
FROM alpine:3.11

RUN apk add --update \
    --no-cache \
    openvpn

COPY --from=builder /usr/bin/brook /usr/bin/brook

ADD entrypoint.sh /entrypoint.sh
RUN chmod a+x /entrypoint.sh

ENTRYPOINT [ "/entrypoint.sh" ]
