#!/bin/bash

source .env

function docker {
cat << EOF > ci/docker/.env
NEO4J_AUTH_USER=$NEO4J_AUTH_USER
NEO4J_AUTH_PASSWORD=$NEO4J_AUTH_PASSWORD
EOF
}

function backend {
cat << EOF > backend/.env
SERVER_PORT=$SERVER_PORT
SERVER_HOST=$SERVER_HOST

RUST_LOG=warn

NEO4J_AUTH_HOST=$NEO4J_AUTH_HOST
NEO4J_AUTH_PORT=$NEO4J_AUTH_PORT
NEO4J_AUTH_USER=$NEO4J_AUTH_USER
NEO4J_AUTH_PASSWORD=$NEO4J_AUTH_PASSWORD
EOF
}

docker
backend

# Автоматическое обновление .env.example
if [ $CURRENT_ENV = "local" ]; then\
    cat .env > .env.example ;\
fi