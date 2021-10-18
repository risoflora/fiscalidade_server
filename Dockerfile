######################################################################
# Copyright (c) 2021 Silvio Clecio (silvioprog) <silvioprog@gmail.com>
#
# SPDX-License-Identifier: MIT
######################################################################

# docker build -t fiscalidade_server .
# docker run -p 3223:3223 -v $HOME/.fiscalidade_server:/home/fiscalidade_server/.fiscalidade_server --rm -idt fiscalidade_server

FROM clux/muslrust as builder
ENV DEBCONF_NOWARNINGS="yes"
RUN apt-get update && apt-get install musl-tools -y
WORKDIR /app
COPY . /app
RUN cargo build --release --target x86_64-unknown-linux-musl && strip /app/target/x86_64-unknown-linux-musl/release/fiscalidade_server

LABEL Maintainer="Silvio Clecio (silvioprog) <silvioprog@gmail.com>"
LABEL Name="fiscalidade_server"
LABEL Version="1.0.0"

FROM alpine
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/fiscalidade_server .
RUN addgroup -S fiscalidade_server && adduser -S fiscalidade_server -G fiscalidade_server
USER fiscalidade_server
EXPOSE 3223
CMD ["./fiscalidade_server"]
