# RCLI

rcli is a rust cli tool.

## 作业一
阅读 chacha20poly1305 文档，了解其使用方法并构建 CLI 对输入文本进行加密 / 解密
要求：
- rcli text encrypt --key "xxx"> 加密并输出 base64
```shell
> rcli text encrypt --key "fixtures/chacha20.key"
hello world!^D
ciphertext: QCD7y8O2jtjtr3U3tn0-l1L_V54xtFnTFJhSSQm3-KseQVQ8d843oA
```

- rcli text decrypt --key "XXX" >base64 > binary> 解密文本
```shell
> rcli text decrypt --key "fixtures/chacha20.key"
QCD7y8O2jtjtr3U3tn0-l1L_V54xtFnTFJhSSQm3-KseQVQ8d843oA
plaintext: hello world!
```

## 作业二
json web token(jwt) 在用户验证领域经常被用到。请构建一个 CLI 来为给定 sub/aud/exp/… 生成一个 jwt。要求生成的 jwt 可以通过 jwt.io 的验证。

CLI：
- rcli jwt sign --sub acme --aud device1 --exp 14d

```shell
》 rcli jwt sign --sub acme --aud device1 --exp 14d
eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJydXN0LWJvb3RjYW1wIiwic3ViIjoiYWNtZSIsImF1ZCI6ImRldmljZTEiLCJleHAiOjE3MjE3MjcxODQsIm5iZiI6MTcyMDUxNzU4NCwiaWF0IjoxNzIwNTE3NTg0LCJqdGkiOiJiMTMxZDRkZC0yMTdkLTRkZWItODQzMi00Zjk5ZTFhNjQ4NzgiLCJyb2xlIjoidGVzdCIsInVzZXJfaWQiOjQyfQ.NgsN6MyLXh_QyoL_pf-rmTqJjDUO1EDM5oDX5YPzJ-o
```

- rcli jwt verify -t

```shell
> rcli jwt verify -t eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJydXN0LWJvb3RjYW1wIiwic3ViIjoiYWNtZSIsImF1ZCI6ImRldmljZTEiLCJleHAiOjE3MjE3MjcxODQsIm5iZiI6MTcyMDUxNzU4NCwiaWF0IjoxNzIwNTE3NTg0LCJqdGkiOiJiMTMxZDRkZC0yMTdkLTRkZWItODQzMi00Zjk5ZTFhNjQ4NzgiLCJyb2xlIjoidGVzdCIsInVzZXJfaWQiOjQyfQ.NgsN6MyLXh_QyoL_pf-rmTqJjDUO1EDM5oDX5YPzJ-o
Claims { reg_claims: RegisteredClaims { iss: "rust-bootcamp", sub: "acme", aud: "device1", exp: 1721727184, nbf: 1720517584, iat: 1720517584, jti: "b131d4dd-217d-4deb-8432-4f99e1a64878" }, pub_claims: PublicClaims { role: "test" }, priv_claims: PrivateClaims { user_id: 42 } }
```

- jwt.io

![jwt.io decode](fixtures/jwt-io-decode.png)

## 作业三
给课程里的 HTTP 文件服务器添加对 directory index 的支持。
![directory index](fixtures/directory-index.png)
