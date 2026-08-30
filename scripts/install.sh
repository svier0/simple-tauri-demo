#!/bin/bash

PNPM_HOME=$(pwd)/pnpm
PNPM_BIN=$PNPM_HOME/pnpm.exe
export PATH="$PNPM_HOME/bin:$(pwd)/node.exe:$PATH"

pnpm install @deepseek-ai/dsh || true
pnpm approve-builds -all
echo '#!/bin/bash
export DSH_HOME="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/.dsh"
export PATH="./node_modules/.bin:$PATH"
dsh web --no-open --port 34333 --trusted-host 127.0.0.1

#dsh plugin --profile web add github:xiaozhe7772222/dsh-opencode-zen
#dsh plugin --profile web list
' > start.sh
chmod 755 ./start.sh