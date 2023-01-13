apt-get update
apt-get install -y ca-certificates curl gnupg lsb-release make cmake g++ python3 node-gyp build-essential libssl-dev git
git config --global pull.rebase false

# install docker cli
# mkdir -p /etc/apt/keyrings
# curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
# echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
# chmod a+r /etc/apt/keyrings/docker.gpg
# apt-get update
# apt-get install -y docker-ce docker-ce-cli containerd.io
curl -fsSL https://get.docker.com | sh

# install nodejs and enable yarn
curl -fsSL https://deb.nodesource.com/setup_19.x | bash - && apt-get install -y nodejs
corepack enable