FROM rust:1.50

WORKDIR /diamond-square-app
COPY ./diamond_square_lib ./diamond_square_lib
COPY ./test_app_diamond_square ./test_app_diamond_square
WORKDIR /diamond-square-app/diamond_square_lib
RUN cargo build
WORKDIR /diamond-square-app/test_app_diamond_square
RUN cargo build

CMD ["cargo", "run"]