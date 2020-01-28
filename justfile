all: tunka build-docker-openvpn

tunka:
	cargo build --release

build-docker-openvpn:
	docker build --tag tunka-openvpn docker/openvpn
