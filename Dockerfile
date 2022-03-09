FROM rust:1.59.0 as build
WORKDIR /app
COPY . /app
RUN cargo build --release 


FROM gcr.io/distroless/cc
COPY --from=build /app/target/release/rapi-template /
EXPOSE 8080
USER nonroot
CMD ["./rapi-template"]