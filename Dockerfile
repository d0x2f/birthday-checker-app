FROM clux/muslrust:stable AS build

WORKDIR /build

COPY . .

RUN cargo build --release

RUN apt-get update && apt-get install -y ca-certificates
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-static-muslc-amd64 /tini
RUN chmod +x /tini

FROM scratch AS run

ENV PORT 8000

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/birthday-checker /birthday-checker
COPY --from=build /build/log4rs.yml /log4rs.yml
COPY --from=build /tini /tini

EXPOSE $PORT

ENTRYPOINT ["/tini", "--"]
CMD ["/birthday-checker"]