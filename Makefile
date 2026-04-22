# human-os/Makefile

make all: kill-port
	tauri dev

clean: kill-port
	rm -rf src-tauri/target

kill-port:
	@echo "Killing port 1420..."
	@PID=$$(lsof -ti :1420); \
	if [ -n "$$PID" ]; then kill -9 $$PID; fi