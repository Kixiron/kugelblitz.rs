FROM rust:latest
RUN git clone https://github.com/kixiron/kugelblitz.git ./kugelblitz
RUN cd ./kugelblitz
RUN cargo run --release
