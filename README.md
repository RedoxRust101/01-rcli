# RCLI

rcli is a rust cli tool.

## 作业一

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
- rcli jwt sign --sub acme --aud device1 --exp 14d

```shell
》 rcli jwt sign --sub acme --aud device1 --exp 14d
eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJydXN0LWJvb3RjYW1wIiwic3ViIjoiYWNtZSIsImF1ZCI6ImRldmljZTEiLCJleHAiOjE3MjE3MjcxODQsIm5iZiI6MTcyMDUxNzU4NCwiaWF0IjoxNzIwNTE3NTg0LCJqdGkiOiJiMTMxZDRkZC0yMTdkLTRkZWItODQzMi00Zjk5ZTFhNjQ4NzgiLCJyb2xlIjoidGVzdCIsInVzZXJfaWQiOjQyfQ.NgsN6MyLXh_QyoL_pf-rmTqJjDUO1EDM5oDX5YPzJ-o
cm@localhost [
```

- rcli jwt verify -t

```shell
> rcli jwt verify -t eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJydXN0LWJvb3RjYW1wIiwic3ViIjoiYWNtZSIsImF1ZCI6ImRldmljZTEiLCJleHAiOjE3MjE3MjcxODQsIm5iZiI6MTcyMDUxNzU4NCwiaWF0IjoxNzIwNTE3NTg0LCJqdGkiOiJiMTMxZDRkZC0yMTdkLTRkZWItODQzMi00Zjk5ZTFhNjQ4NzgiLCJyb2xlIjoidGVzdCIsInVzZXJfaWQiOjQyfQ.NgsN6MyLXh_QyoL_pf-rmTqJjDUO1EDM5oDX5YPzJ-o
Claims { reg_claims: RegisteredClaims { iss: "rust-bootcamp", sub: "acme", aud: "device1", exp: 1721727184, nbf: 1720517584, iat: 1720517584, jti: "b131d4dd-217d-4deb-8432-4f99e1a64878" }, pub_claims: PublicClaims { role: "test" }, priv_claims: PrivateClaims { user_id: 42 } }
```

- jwt.io

![jwt.io decode](fixtures/jwt-io-decode.png)
