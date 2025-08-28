# The example of mTLS in Rust.

## Environments

- OS: Ubuntu 22.04.5 LTS
- OpenSSL: OpenSSL 3.0.2 15 Mar 2022 (Library: OpenSSL 3.0.2 15 Mar 2022)
- Rust: cargo 1.88.0 (873a06493 2025-05-10)

## Generate CA

```
$ make generate-ca
mkdir -p certs
openssl genrsa -out certs/ca.key 4096
openssl req -x509 -new -nodes -key certs/ca.key -sha256 -days 365 -out certs/ca.crt -subj "/CN=ca" -config certs/openssl.ext -extensions v3_ca
openssl genrsa -out certs/server.key 4096
openssl req -new -key certs/server.key -out certs/server.csr -subj "/CN=localhost"
openssl x509 -req -in certs/server.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/server.crt -days 365 -sha256 -extfile certs/openssl.ext -extensions server_ext
Certificate request self-signature ok
subject=CN = localhost
openssl genrsa -out certs/client_1.key 4096
openssl req -new -key certs/client_1.key -out certs/client_1.csr -subj "/CN=client_1"
openssl x509 -req -in certs/client_1.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/client_1.crt -days 365 -sha256 -extfile certs/openssl.ext -extensions client_ext
Certificate request self-signature ok
subject=CN = client_1
openssl genrsa -out certs/client_2.key 4096
openssl req -new -key certs/client_2.key -out certs/client_2.csr -subj "/CN=client_2"
openssl x509 -req -in certs/client_2.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/client_2.crt -days 365 -sha256 -extfile certs/openssl.ext -extensions client_ext
Certificate request self-signature ok
subject=CN = client_2
```

## Start server

```
$ make start-server
cargo run --bin mtls-server
   Compiling mtls-example v0.1.0 (/hpc/mtr_scrap/users/klausm/workspace/src/github.com/k82cn/mtls-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.52s
     Running `target/debug/mtls-server`

```

## Start client

```
$ make start-client
cargo run --bin mtls-client
   Compiling mtls-example v0.1.0 (/hpc/mtr_scrap/users/klausm/workspace/src/github.com/k82cn/mtls-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.29s
     Running `target/debug/mtls-client`
RESPONSE=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Thu, 28 Aug 2025 02:10:06 GMT", "grpc-status": "0"} }, message: HelloReply { message: "Hello Tonic!" }, extensions: Extensions }
```


## Check server logs

```
[2025-08-28T02:10:06Z INFO  mtls_server] Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.14.1"} }, message: HelloRequest { name: "Tonic" }, extensions: Extensions }
[2025-08-28T02:10:06Z INFO  mtls_server] Peer certs: 1
[2025-08-28T02:10:06Z INFO  mtls_server] Peer cert subject: CN=client_2
[2025-08-28T02:10:06Z INFO  mtls_server] Peer cert issuer: CN=ca
[2025-08-28T02:10:06Z INFO  mtls_server] Peer cert extensions: [X509Extension { oid: OID(2.5.29.19), critical: false, value: [48, 0], parsed_extension: BasicConstraints(BasicConstraints { ca: false, path_len_constraint: None }) }, X509Extension { oid: OID(2.5.29.15), critical: true, value: [3, 2, 5, 160], parsed_extension: KeyUsage(KeyUsage { flags: 5 }) }, X509Extension { oid: OID(2.5.29.37), critical: false, value: [48, 10, 6, 8, 43, 6, 1, 5, 5, 7, 3, 2], parsed_extension: ExtendedKeyUsage(ExtendedKeyUsage { any: false, server_auth: false, client_auth: true, code_signing: false, email_protection: false, time_stamping: false, ocsp_signing: false, other: [] }) }, X509Extension { oid: OID(2.5.29.14), critical: false, value: [4, 20, 196, 28, 23, 102, 208, 84, 200, 246, 57, 162, 98, 34, 31, 161, 243, 39, 90, 63, 245, 151], parsed_extension: SubjectKeyIdentifier(KeyIdentifier([196, 28, 23, 102, 208, 84, 200, 246, 57, 162, 98, 34, 31, 161, 243, 39, 90, 63, 245, 151])) }, X509Extension { oid: OID(2.5.29.35), critical: false, value: [48, 22, 128, 20, 70, 225, 62, 33, 94, 18, 157, 110, 39, 78, 193, 97, 157, 41, 25, 149, 94, 253, 94, 155], parsed_extension: AuthorityKeyIdentifier(AuthorityKeyIdentifier { key_identifier: Some(KeyIdentifier([70, 225, 62, 33, 94, 18, 157, 110, 39, 78, 193, 97, 157, 41, 25, 149, 94, 253, 94, 155])), authority_cert_issuer: None, authority_cert_serial: None }) }]
[2025-08-28T02:10:06Z INFO  mtls_server] Peer cert raw serial: [46, 161, 109, 176, 27, 156, 225, 152, 8, 47, 148, 89, 81, 230, 132, 88, 37, 233, 86, 152]
[2025-08-28T02:10:06Z INFO  mtls_server] Peer cert version: X509Version(2)

```