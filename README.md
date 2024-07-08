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
