# common


```
$ docker build -t builder-test:test ./Dockerfiles/rust-builder-arm
$ docker build -t builder-test:test ./Dockerfiles/rust-builder-x86_64
$ docker run -i -t --rm builder-test:test /bin/sh
```