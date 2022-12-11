apt-get update
apt-get install ca-certificates curl gnupg lsb-release make cmake g++ python3 node-gyp build-essential libssl-dev git -y
curl -fsSL https://get.docker.com | sh
curl -fsSL https://deb.nodesource.com/setup_19.x | bash - && apt-get install -y nodejs
git config pull.rebase false
corepack enable