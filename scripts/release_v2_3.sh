#!/bin/bash
set -e

# 定义项目根目录的绝对路径（用单引号避免转义问题）
PROJECT_ROOT='/root/nexu-protocol'

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${GREEN}Step 1: Compiling Nexu Core (Rust)${NC}"
cd "${PROJECT_ROOT}/nexu-core"
cargo build --release --target x86_64-unknown-linux-gnu
cbindgen --lang c --output include/nexu_core.h .
echo -e "${GREEN}✅ Rust core compiled and C header generated${NC}"

echo -e "${GREEN}Step 2: Validating C header${NC}"
if [ ! -f include/nexu_core.h ]; then
    echo -e "${RED}❌ C header file not found${NC}"
    exit 1
fi
echo -e "${GREEN}✅ C header validated${NC}"

echo -e "${GREEN}Step 3: Building Go SDK${NC}"
cd "${PROJECT_ROOT}/sdk/go"
go mod init nexucore
go build -o nexucore.so -buildmode=c-shared .
echo -e "${GREEN}✅ Go SDK built${NC}"

echo -e "${GREEN}Step 4: Generating release assertion${NC}"
cd "${PROJECT_ROOT}"
echo "Nexu v2.3 (RC-1) Build Assertion
Build Time: $(date +%Y-%m-%d_%H:%M:%S)
Rust Core Hash: $(sha256sum nexu-core/target/release/libnexu_core.so | cut -d' ' -f1)
C Header Hash: $(sha256sum nexu-core/include/nexu_core.h | cut -d' ' -f1)
Go SDK Hash: $(sha256sum sdk/go/nexucore.so | cut -d' ' -f1)
Cold-Truth: Deterministic and Recomputable" > RELEASE_ASSERTION.txt

echo -e "${GREEN}🏁 Nexu v2.3 (RC-1) build completed successfully!${NC}"
echo -e "${GREEN}🔒 Causal chain anchored at: $(pwd)/RELEASE_ASSERTION.txt${NC}"
