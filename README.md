# zacli

[Zaif Web api](http://techbureau-api-document.readthedocs.io/ja/latest/index.html)をコマンドラインから叩けるツールです。

[Rust製のAPI Wrapper](https://github.com/yajamon/zaif-api-rust)を作ったので、シンプルなクライアントも必要ですよね、ということで。

## 使い方

```sh
zacli currencies <NAME>
zacli currency_pairs <CURRENCY_PAIR>
zacli depth <CURRENCY_PAIR>
zacli trade <CURRENCY_PAIR> <ACTION> <AMOUNT> <PRICE>
```
