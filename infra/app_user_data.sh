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

# make
apt-get install make -y

# aws cli
apt-get install awscli -y

# jq
apt-get install jq -y

# ifconfig
apt-get install net-tools -y

# clone codenames
cd /home/ubuntu
git clone https://github.com/matthewtgilbride/codenames.git