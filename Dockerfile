FROM gcr.io/distroless/base-debian10
RUN mkdir -p /app
WORKDIR /app
COPY ./scripts/start.sh /app
COPY ./rasis/credentials/bot-lab/env.sh /app
COPY ./rasis/target/release/rasis /app
WORKDIR /app
CMD ["start.sh"]
