TMP_SERVER_PID=/tmp/jilo-dev-server.pid

.PHONY: build, front, back, dev

build: 
	cd app && npm run build
	cargo build --release

front:
	cd app && npm run dev

back:	
	cargo watch -x run

dev:
	make -j 2 back front