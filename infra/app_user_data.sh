# apt-get update -y
# apt-get install ca-certificates y
# apt-get install libssl-dev -y

# nats
curl -L https://github.com/nats-io/nats-server/releases/download/v2.8.4/nats-server-v2.8.4-amd64.deb -o nats-server.deb
apt-get install ./nats-server.deb -y

# wasmcloud host
wget https://github.com/wasmCloud/wasmcloud-otp/releases/download/v0.56.0/x86_64-linux.tar.gz
mkdir -p wasmcloud
tar -xvf x86_64-linux.tar.gz -C wasmcloud

# wash
curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | bash
apt-get install wash -y

# start nats
nohup nats-server -js -l nats.log &

# start host
nohup wasmcloud/bin/wasmcloud_host start

sleep 30

# link
wash ctl link put \
  MBBVDZJTLOX3O5XUHZB2BLG3GKGU4AOWVOVKWN5CH4AUPYHDSSABC454 \
  VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M \
  wasmcloud:httpserver \
  ADDRESS=0.0.0.0:8080 \
  allowed_origins="https://codenames.mattgilbride.com"

wash ctl link put \
  MBBVDZJTLOX3O5XUHZB2BLG3GKGU4AOWVOVKWN5CH4AUPYHDSSABC454 \
  VAXC6ARD2NH5TQ3TQ3ZE6U2EOBE4QWD4NQWW6CH26VFLY27VZYM4UJA3 \
  aws:kvdynamodb \
  config_json='{ "table_name": "codenames", "key_attribute": "key", "value_attribute": "game" }'

# start providers and actor
wash ctl start provider ghcr.io/matthewtgilbride/kvdynamodb_provider_x86:0.1.0
wash ctl start provider wasmcloud.azurecr.io/httpserver:0.16.2
wash ctl start actor ghcr.io/matthewtgilbride/codenames:0.1.0
