### tunka-openvpn

FROM xrelkd/tunelo:v0.1.6-alpine

RUN apk add --no-cache --update openvpn

ADD entrypoint.sh /entrypoint.sh
RUN chmod a+x /entrypoint.sh

ENTRYPOINT [ "/entrypoint.sh" ]
