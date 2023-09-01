# change .vscode/settings.json to switch "rust-analyzer.cargo.target" to aarch64 or riscv64 or x86_64

ARCH="aarch64-unknown-none"

if [ "$1" = "aarch64" ]; then
    ARCH="aarch64-unknown-none"
elif [ "$1" = "riscv64" ]; then
    ARCH="riscv64gc-unknown-none-elf"
elif [ "$1" = "x86_64" ]; then
    ARCH="x86_64-unknown-none"
fi

# edit .vscode/settings.json
sed -i '' -e "s/\"rust-analyzer.cargo.target\":.*/\"rust-analyzer.cargo.target\": \"$ARCH\"/" .vscode/settings.json
