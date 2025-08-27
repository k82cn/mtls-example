generate-ca:
	openssl genrsa -out certs/ca.key 4096
	openssl req -x509 -new -nodes -key certs/ca.key -sha256 -days 365 -out certs/ca.crt -subj "/CN=ca"
	openssl genrsa -out certs/server.key 4096
	openssl req -new -key certs/server.key -out certs/server.csr -subj "/CN=server"
	openssl x509 -req -in certs/server.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/server.crt -days 365 -sha256
	openssl genrsa -out certs/client_1.key 4096
	openssl req -new -key certs/client_1.key -out certs/client_1.csr -subj "/CN=client_1"
	openssl x509 -req -in certs/client_1.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/client_1.crt -days 365 -sha256
	openssl genrsa -out certs/client_2.key 4096
	openssl req -new -key certs/client_2.key -out certs/client_2.csr -subj "/CN=client_2"
	openssl x509 -req -in certs/client_2.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/client_2.crt -days 365 -sha256

start-server:
	cargo run --bin mtls-server

start-client:
	cargo run --bin mtls-client
