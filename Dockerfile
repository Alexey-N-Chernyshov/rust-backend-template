FROM rust:1.80 AS builder

# cache project dependencies
RUN USER=root cargo new my-project-name
WORKDIR ./my-project-name/
COPY Cargo.toml Cargo.lock build.rs ./
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/my-project-name*
RUN rm ./target/release/deps/my_project_name*

# build
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./.git ./.git
RUN cargo build --release

# install docker and copy test
FROM builder AS prepared_to_run_tests
# install docker for integration tests
RUN curl -fsSL https://get.docker.com -o get-docker.sh
RUN sh get-docker.sh
COPY ./tests ./tests

# test
FROM prepared_to_run_tests AS tester
RUN cargo test --no-run
ENTRYPOINT ["cargo" , "test"]

# generates coverage report
FROM rust:1.80 AS coverage
RUN cargo install grcov
RUN rustup component add llvm-tools-preview
# install docker for integration tests
RUN curl -fsSL https://get.docker.com -o get-docker.sh
RUN sh get-docker.sh
WORKDIR ./my-project-name/
COPY Cargo.toml Cargo.lock build.rs ./
COPY ./.git ./.git
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./tests ./tests
COPY coverage.sh ./
RUN chmod +x coverage.sh
ENTRYPOINT ["bash", "coverage.sh"]

# release
FROM debian:bookworm-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y libpq-dev \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ARG APP_USER=appuser
ENV TZ=Etc/UTC

RUN groupadd "$APP_USER" \
    && useradd -g "$APP_USER" "$APP_USER" \
    && mkdir -p ${APP}

COPY --from=builder /my-project-name/target/release/my-project-name ${APP}/my-project-name

RUN chown -R "$APP_USER":"$APP_USER" ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./my-project-name"]

COPY healthcheck.sh .
HEALTHCHECK --interval=5s --retries=5 CMD ./healthcheck.sh
