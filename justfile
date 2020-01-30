all: tunka build-docker-openvpn

tunka:
	cargo build --release

build-docker-openvpn:
	docker build --tag tunka-openvpn docker/openvpn

build-docker-tor:
	docker build --tag tunka-tor docker/tor
