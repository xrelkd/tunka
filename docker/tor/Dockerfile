FROM alpine:latest

RUN apk add --no-cache tor && \
    sed "1s/^/SocksPort 0.0.0.0:9050\n/" /etc/tor/torrc.sample > /etc/tor/torrc

EXPOSE 9050

VOLUME [ "/var/lib/tor" ]

USER tor

CMD [ "/usr/bin/tor" ]
