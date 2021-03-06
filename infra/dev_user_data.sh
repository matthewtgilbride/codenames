apt-get update -y

# docker
apt-get install \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release -y

curl -fsSL https://download.docker.com/linux/ubuntu/gpg | \
  gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

echo \
  "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

apt-get update -y

apt-get install docker-ce docker-ce-cli containerd.io -y

groupadd docker

usermod -aG docker ubuntu

systemctl enable docker.service
systemctl enable containerd.service

# docker-compose
curl -L "https://github.com/docker/compose/releases/download/1.28.5/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# rust
su - ubuntu -c \
  'curl https://sh.rustup.rs -sSf | sh -s -- -y'
su - ubuntu -c \
  'rustup target add wasm32-unknown-unknown'

# make
apt-get install make -y

# aws cli
apt-get install awscli -y

# jq
apt-get install jq -y

# ifconfig
apt-get install net-tools -y

# node
apt-get install nodejs -y
apt-get install npm -y
npm install --global yarn

# wash and wasmcloud
curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | bash
apt-get install wasmcloud wash -y

# clone codenames
cd /home/ubuntu
git clone https://github.com/matthewtgilbride/codenames.git
