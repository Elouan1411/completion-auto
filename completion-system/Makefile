install: setup udev build move_binary

setup:
	@echo "Updating system and installing dependencies..."
	@sudo apt update
	@sudo apt install -y cargo libudev-dev pkg-config python3 python3-tk
	@sudo usermod -aG input elouan

udev:
	@echo "Configuring udev rules..."
	@if ! grep -q 'KERNEL=="uinput", OWNER="elouan", MODE="600"' /etc/udev/rules.d/99-uinput-completion-system.rules; then \
		echo 'KERNEL=="uinput", OWNER="elouan", MODE="600"' | sudo tee -a /etc/udev/rules.d/99-uinput-completion-system.rules; \
		sudo udevadm control --reload-rules && sudo udevadm trigger; \
		echo "uinput rules added."; \
	else \
		echo "uinput rules already exist."; \
	fi

build:
	@echo "Building project..."
	@cargo build --release || { echo "Build failed"; exit 1; }

run:
	@echo "Running the executable..."
	@if [ -f ./target/release/$(shell basename $(CURDIR)) ]; then \
		./target/release/$(shell basename $(CURDIR)); \
	else \
		echo "Executable not found"; \
		exit 1; \
	fi

move_binary:
	@echo "Moving binary to /usr/local/bin..."
	@if [ -f ./target/release/$(shell basename $(CURDIR)) ]; then \
		if [ -f /usr/local/bin/$(shell basename $(CURDIR)) ]; then \
			echo "Executable already exists in /usr/local/bin/"; \
		else \
			sudo mv target/release/$(shell basename $(CURDIR)) /usr/local/bin/; \
			echo "Executable moved to /usr/local/bin/"; \
		fi \
	else \
		echo "Executable not found"; \
		exit 1; \
	fi

clean:
	@echo "Cleaning up build files..."
	@rm -rf target
	@echo "Build files removed."

uninstall:
	@echo "Uninstalling binary..."
	@if [ -f /usr/local/bin/$(shell basename $(CURDIR)) ]; then \
		sudo rm /usr/local/bin/$(shell basename $(CURDIR)); \
		echo "Executable removed from /usr/local/bin/"; \
	else \
		echo "Executable not found in /usr/local/bin/"; \
		exit 1; \
	fi

	@echo "Uninstalling udev rules..."
	@if grep -q 'KERNEL=="uinput", OWNER="elouan", MODE="600"' /etc/udev/rules.d/99-uinput-completion-system.rules; then \
		sudo rm /etc/udev/rules.d/99-uinput-completion-system.rules; \
		sudo udevadm control --reload-rules && sudo udevadm trigger; \
		echo "uinput rules removed."; \
	else \
		echo "uinput rules do not exist."; \
	fi