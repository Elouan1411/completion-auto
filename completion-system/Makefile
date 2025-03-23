USERNAME := $(shell whoami)
APP_NAME := completion-system
BIN_PATH := /usr/local/bin/$(APP_NAME)
ICON_PATH := /usr/local/share/icons/$(APP_NAME).png
DESKTOP_FILE := ~/.local/share/applications/$(APP_NAME).desktop

install: setup udev build move_binary install_desktop

setup:
	@echo "Updating system and installing dependencies..."
	@sudo apt update
	@sudo apt install -y cargo libudev-dev pkg-config python3 python3-tk
	@sudo usermod -aG input $(USERNAME)

udev:
	@echo "Configuring udev rules..."
	@if ! grep -q 'KERNEL=="uinput", OWNER="$(USERNAME)", MODE="600"' /etc/udev/rules.d/99-uinput-completion-system.rules; then \
		echo 'KERNEL=="uinput", OWNER="$(USERNAME)", MODE="600"' | sudo tee -a /etc/udev/rules.d/99-uinput-completion-system.rules; \
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
	@if [ -f ./target/release/$(APP_NAME) ]; then \
		./target/release/$(APP_NAME); \
	else \
		echo "Executable not found"; \
		exit 1; \
	fi

move_binary:
	@echo "Moving binary to /usr/local/bin..."
	@if [ -f ./target/release/$(APP_NAME) ]; then \
		if [ -f $(BIN_PATH) ]; then \
			echo "Executable already exists in /usr/local/bin/"; \
		else \
			sudo mv target/release/$(APP_NAME) $(BIN_PATH); \
			echo "Executable moved to /usr/local/bin/"; \
		fi \
	else \
		echo "Executable not found"; \
		exit 1; \
	fi

install_desktop:
	@echo "Installing desktop entry and icon..."
	@if [ ! -f $(DESKTOP_FILE) ]; then \
		echo "[Desktop Entry]" > $(DESKTOP_FILE); \
		echo "Type=Application" >> $(DESKTOP_FILE); \
		echo "Name=Completion System" >> $(DESKTOP_FILE); \
		echo "Exec=$(BIN_PATH)" >> $(DESKTOP_FILE); \
		echo "Icon=$(ICON_PATH)" >> $(DESKTOP_FILE); \
		echo "Terminal=false" >> $(DESKTOP_FILE); \
		echo "Categories=Utility;" >> $(DESKTOP_FILE); \
		echo "Desktop entry created."; \
	else \
		echo "Desktop entry already exists."; \
	fi
	@if [ -f completion-system.png ] && [ ! -f $(ICON_PATH) ]; then \
		sudo install -D completion-system.png $(ICON_PATH); \
		echo "Icon installed."; \
	else \
		echo "Icon already exists or source missing."; \
	fi

clean:
	@echo "Cleaning up build files..."
	@rm -rf target
	@echo "Build files removed."

uninstall:
	@echo "Uninstalling binary..."
	@if [ -f $(BIN_PATH) ]; then \
		sudo rm $(BIN_PATH); \
		echo "Executable removed from /usr/local/bin/"; \
	else \
		echo "Executable not found in /usr/local/bin/"; \
	fi

	@echo "Uninstalling udev rules..."
	@if grep -q 'KERNEL=="uinput", OWNER="$(USERNAME)", MODE="600"' /etc/udev/rules.d/99-uinput-completion-system.rules; then \
		sudo rm /etc/udev/rules.d/99-uinput-completion-system.rules; \
		sudo udevadm control --reload-rules && sudo udevadm trigger; \
		echo "uinput rules removed."; \
	else \
		echo "uinput rules do not exist."; \
	fi

	@echo "Removing desktop entry and icon..."
	@if [ -f $(DESKTOP_FILE) ]; then \
		rm $(DESKTOP_FILE); \
		echo "Desktop entry removed."; \
	else \
		echo "Desktop entry not found."; \
	fi
	@if [ -f $(ICON_PATH) ]; then \
		sudo rm $(ICON_PATH); \
		echo "Icon removed."; \
	else \
		echo "Icon not found."; \
	fi
