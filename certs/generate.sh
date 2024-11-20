#! /bin/sh
openssl req -x509 -sha512 -days 3650 -newkey rsa:4096 -keyout ca.key -out ca.crt -nodes -subj "/C=CN/ST=ShangHai/L=ShangHai/O=K and A Ltd/OU=./CN=mmitsuha.xyz."
openssl genpkey -algorithm RSA -out end.key -pkeyopt rsa_keygen_bits:4096
openssl req -new -key end.key -out end.csr -subj "/C=CN/ST=ShangHai/L=ShangHai/O=K and A Ltd/OU=./CN=mmitsuha.xyz."
openssl x509 -req -in end.csr -CA ca.crt -CAkey ca.key -CAcreateserial -extfile cert.ext -out end.crt -days 3650 -sha256
openssl rsa -in ca.key -out ca.key.der -outform DER
openssl x509 -in ca.crt -out ca.crt.der -outform DER
openssl rsa -in end.key -out end.key.der -outform DER
openssl x509 -in end.crt -out end.crt.der -outform DER
