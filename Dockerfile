FROM rust:1.62 as build
WORKDIR /wren
COPY . .
RUN cargo build --release

FROM debian:buster-slim as runtime
RUN apt-get -y update && apt-get -y install libssl-dev ca-certificates
COPY --from=build /wren/.env .
COPY --from=build /wren/target/release/wren .

