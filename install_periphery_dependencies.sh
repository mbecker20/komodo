apt-get update
apt-get install ca-certificates curl gnupg lsb-release -y
curl -fsSL https://get.docker.com | sh
apt-get update
apt-get install git -y
git config pull.rebase false
curl -fsSL https://deb.nodesource.com/setup_19.x | bash - && apt-get install -y nodejs
corepack enable