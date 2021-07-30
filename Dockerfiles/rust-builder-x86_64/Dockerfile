FROM rust:1.53-buster

ENV PATH=$PATH:$HOME/bin:/sbin:/usr/sbin:/usr/local/bin

RUN apt-get update -y && apt-get install -y apt-transport-https ca-certificates curl gnupg lsb-release && \
    curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg && \
    echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null && \
    apt-get update -y && \
    apt-get upgrade -y && \
    apt-get install -y docker-ce docker-ce-cli containerd.io
#CMD [ "service docker start" ]