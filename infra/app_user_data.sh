apt-get update -y
apt-get install ca-certificates y

# clone codenames
cd /home/ubuntu
git clone https://github.com/matthewtgilbride/codenames.git

cd /home/ubuntu/codenames/service

export ALLOWED_ORIGINS=https://codenames.mattgilbride.com

nohup ./codenames-actix &
