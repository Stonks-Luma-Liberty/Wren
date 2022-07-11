FROM rust:1.62 as build
WORKDIR /wren
COPY . .
RUN cargo build --release

FROM rust:1.62-slim-bullseye as runtime
COPY --from=build /wren/.env .
COPY --from=build /wren/target/release/wren .

