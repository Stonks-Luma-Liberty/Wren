FROM rust:1.62 as build
WORKDIR /wren
COPY . .
RUN cargo build --release

FROM rust:1.62-slim-bullseye as runtime
RUN apt-get -y update && apt-get -y install libssl-dev ca-certificates
COPY --from=build /wren/.env .
COPY --from=build /wren/target/release/wren .

