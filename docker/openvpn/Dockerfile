### tunka-openvpn

FROM xrelkd/tunelo:v0.1.3-slim

RUN apt-get update && apt-get -y install openvpn

ADD entrypoint.sh /entrypoint.sh
RUN chmod a+x /entrypoint.sh

ENTRYPOINT [ "/entrypoint.sh" ]
