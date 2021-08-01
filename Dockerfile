FROM gcr.io/distroless/base-debian10
COPY ./scripts/start.sh /root
COPY ./rasis/target/release/rasis /root
COPY ./rasis/credentials/bot-lab/env.sh /root
CMD ["/root/start.sh"]
