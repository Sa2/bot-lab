# common


```
$ docker build -t builder-test:test ./Dockerfiles/rust-builder-aarch64
$ docker build -t builder-test:test ./Dockerfiles/rust-builder-x86_64
$ docker run --privileged  -i -t --rm builder-test:test /bin/sh
```