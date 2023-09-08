#!/bin/zsh

trunk build --release --public-url /alphabet -v
scp -r -v dist/* root@oneofone.dev:/srv/http-main/alphabet
