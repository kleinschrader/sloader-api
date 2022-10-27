FROM alpine:3.16.2 AS builder

WORKDIR /usr/app/

COPY . .

RUN apk add --no-cache cargo openssl openssl-dev

RUN cargo build --release && \
    strip /usr/app/target/release/sloader-api

FROM alpine:3.16.2
 
COPY --from=builder /usr/app/target/release/sloader-api /usr/app/sloader-api

RUN apk add --no-cache openssl libgcc

ENTRYPOINT ["/usr/app/sloader-api"]