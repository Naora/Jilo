TMP_SERVER_PID=/tmp/jilo-dev-server.pid

.PHONY: build, run, watch, install

install:
	sudo apt install inotify-tools

build: | public
	cd app && npm run build
	cargo build

public:
	ln -s app/public .

run: build public
	cargo run

watch:
	while true; do \
		make run & echo $$! > $(TMP_SERVER_PID); \
		inotifywait -qre close_write web/src app/src; \
		kill `cat $(TMP_SERVER_PID)`; \
	done