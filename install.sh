#!/bin/bash
set -e

REPO="aladac/thumbsdown"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="thumbsdown"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}==>${NC} $1"; }
warn() { echo -e "${YELLOW}==>${NC} $1"; }
error() { echo -e "${RED}Error:${NC} $1"; exit 1; }

# Detect platform
detect_platform() {
    local os arch

    case "$(uname -s)" in
        Linux*)  os="linux" ;;
        Darwin*) os="macos" ;;
        *)       error "Unsupported OS: $(uname -s)" ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)  arch="amd64" ;;
        aarch64|arm64) arch="arm64" ;;
        *)             error "Unsupported architecture: $(uname -m)" ;;
    esac

    # macOS Intel not supported
    if [[ "$os" == "macos" && "$arch" == "amd64" ]]; then
        error "macOS Intel (x86_64) is not supported. Use 'cargo install thumbsdown' instead."
    fi

    echo "${os}-${arch}"
}

# Get latest release tag
get_latest_version() {
    curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4
}

main() {
    info "Detecting platform..."
    local platform=$(detect_platform)
    info "Platform: $platform"

    info "Fetching latest version..."
    local version=$(get_latest_version)
    [[ -z "$version" ]] && error "Failed to fetch latest version"
    info "Version: $version"

    local asset="thumbsdown-${platform}"
    local url="https://github.com/${REPO}/releases/download/${version}/${asset}"

    info "Downloading ${asset}..."
    mkdir -p "$INSTALL_DIR"

    if ! curl -sL "$url" -o "${INSTALL_DIR}/${BINARY_NAME}"; then
        error "Failed to download from $url"
    fi

    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    info "Installed to ${INSTALL_DIR}/${BINARY_NAME}"

    # Check if INSTALL_DIR is in PATH
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        warn "${INSTALL_DIR} is not in your PATH"
        echo ""
        echo "Add it to your shell profile:"
        echo ""
        echo "  # For bash (~/.bashrc or ~/.bash_profile)"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
        echo "  # For zsh (~/.zshrc)"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
        echo "Then restart your shell or run: source ~/.bashrc"
    else
        info "Run 'thumbsdown --help' to get started"
    fi
}

main
