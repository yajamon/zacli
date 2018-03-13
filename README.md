# zacli

[Zaif Web api](http://techbureau-api-document.readthedocs.io/ja/latest/index.html)をコマンドラインから叩けるツールです。

[Rust製のAPI Wrapper](https://github.com/yajamon/zaif-api-rust)を作ったので、シンプルなクライアントも必要ですよね、ということで。
チャートは公式サイトで確認しつつ売買注文はAPIから、というのもありかと。

## インストール方法

```sh
cargo install zacli
```

## 使い方

以下はコマンドの一部です。`zacli help`ですべてのコマンドが確認できます。

```sh
zacli currencies <NAME>
zacli currency_pairs <CURRENCY_PAIR>
zacli depth <CURRENCY_PAIR>
zacli trade <CURRENCY_PAIR> <ACTION> <AMOUNT> <PRICE>
```

### API Keyをzacliに教える

`trade`など、一部のコマンドにはAPIアクセスキー、シークレットが必要です。
zacliがこれらを知る手段として設定ファイルを採用しています。

設定ファイルは次のコマンドで生成できます。

`zacli config --init`

ホームディレクトリに`.zacli.toml`が生成されているので、APIアクセスキー、シークレットを差し替えてください。

### 成行注文をする

`zaif_jpy`の成行の買い注文をするとします。
板の情報は`zacli depth`で確認することができます。

```sh
zacli depth zaif_jpy | grep ask | head -1
# ask     1.500   100.0
```

depthの結果は`type`, `price`, `amount`の順で表示されます。
ここでは`type`が`ask`のもの（売り注文）をgrepで絞り込んでいます。

*NOTE: 売り注文はpriceが低いもの、買い注文はpriceが高いものから順に表示されます*

絞り込んだ結果から1行だけ取り出し、今売りだされている一番安い価格を確認します。（1.500 JPY)
これより高い価格で買い注文を出せば、成行注文が成立します。

```sh
# 2.0 JPY で数 10 の買い注文を発行します
zacli trade zaif_jpy bid 2.0 10
```
