alias openapi-generator=/usr/sbin/run-in-docker.sh

openapi-generator generate -i api.yaml -g rust-server -o rust
openapi-generator generate -i api.yaml -g typescript-fetch -o javascript
