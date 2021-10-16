alias openapi-generator=/usr/sbin/run-in-docker.sh

openapi-generator-cli generate -i api.yaml -g rust-server -o rust
