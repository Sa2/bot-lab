FROM gcr.io/distroless/base-debian10
COPY ./scripts/start.sh /root
COPY ./rasis/credentials/bot-lab/env.sh /root
COPY ./rasis/target/release/rasis /root
CMD ["/root/start.sh"]
