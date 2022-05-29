apt-get update -y
# apt-get install ca-certificates y

# nats
curl -L https://github.com/nats-io/nats-server/releases/download/v2.8.4/nats-server-v2.8.4-amd64.deb -o nats-server.deb
apt-get install ./nats-server.deb -y

# wash
curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | bash
apt-get install wash -y

# wasmcloud host
wget https://github.com/wasmCloud/wasmcloud-otp/releases/download/v0.54.6/x86_64-linux.tar.gz
mkdir -p wasmcloud
tar -xvf x86_64-linux.tar.gz -C wasmcloud

# start nats
nohup nats-server &

# start host
nohup wasmcloud/bin/wasmcloud_host start

# start providers and actor
wash ctl start provider ghcr.io/matthewtgilbride/kvdynamodb_provider:0.1.0
wash ctl start provider wasmcloud.azurecr.io/httpserver:0.15.0
wash ctl start actor ghcr.io/matthewtgilbride/codenames:0.1.0

# link
wash ctl link put \
	  MBBVDZJTLOX3O5XUHZB2BLG3GKGU4AOWVOVKWN5CH4AUPYHDSSABC454 \
	  VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M \
	  wasmcloud:httpserver \
	  PORT=8080 \
	  allowed_origins="https://codenames.mattgilbride.com"

wash ctl link put \
  MBBVDZJTLOX3O5XUHZB2BLG3GKGU4AOWVOVKWN5CH4AUPYHDSSABC454 \
  VBM7ATBGFQ2ZWDECPWUTLRIIWSBPABHBL5UL5CD7LDNR5I4NKFXZJ5EA \
  aws:kvdynamodb \
  config_json='{ "table_name": "codenames", "key_attribute": "key", "value_attribute": "game" }'
