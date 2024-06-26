#!/bin/sh
#

# -------------------------------------------------------
# update env

export PATH="$PATH:$HOME/.acme.sh"
export PATH="$PATH:$HOME/.cargo/bin"

# -------------------------------------------------------
# setup dependencies

if [ ! -x /usr/bin/curl ]; then
    apk add openssl curl socat
fi

if [ ! -x ~/.acme.sh/acme.sh ]; then
    curl https://get.acme.sh | sh -s email=acme@rehi.org
    acme.sh --register-account --server letsencrypt_test
    acme.sh --set-default-ca --server letsencrypt_test
fi

# -------------------------------------------------------
# start dev server

cargo run
