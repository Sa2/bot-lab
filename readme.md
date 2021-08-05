
# botアカウント の管理の仕方

webでログインして、application page へ遷移するする

https://discord.com/developers/applications


# ローカルで動かす

```
$ docker run -it --rm -p 5001:5001 --name bot-lab-rasis io.raspberry.local:5000/bot-lab-rasis:latest
```

# デプロイ

```
$ kubectl apply -f k8s-deployment.yaml
```


containerd で pull する

```
$ crictl pull io.raspberry.local:5000/bot-lab-rasis:latest
```