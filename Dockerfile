FROM rust:alpine3.22 AS build

# ENV HTTPS_PROXY=http://us-west-2-proxy.lendingcloud.us:8080 
# ENV HTTP_PROXY=http://us-west-2-proxy.lendingcloud.us:8080

WORKDIR /app

RUN apk add --no-cache musl-dev vim curl

COPY . .
RUN cargo build

CMD ["cargo", "run"]
# CMD ["sleep", "1d"]

#TODO - Add the final layer with just the binary