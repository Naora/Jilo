TMP_SERVER_PID=/tmp/jilo-dev-server.pid

.PHONY: build, dev, run, watch, install

install:
	sudo apt install inotify-tools

build: 
	cd app && npm run build
	cargo build --release

dev: | public
	cd app && npm run dev
	cargo build

public:
	ln -s app/public .

run: dev public
	cargo run

watch:
	while true; do \
		make run & echo $$! > $(TMP_SERVER_PID); \
		inotifywait -qre close_write web/src app/src app/sass; \
		kill `cat $(TMP_SERVER_PID)`; \
	done