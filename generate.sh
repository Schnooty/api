alias openapi-generator=/usr/sbin/run-in-docker.sh

openapi-generator-cli generate -i api.yaml -g rust-server -o rust
openapi-generator-cli generate -i api.yaml -g typescript-fetch -o javascript
openapi-generator-cli generate -i api.yaml -g markdown
