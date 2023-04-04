# Rust Introduction

## 環境構築

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
echo "export PATH="$HOME/.cargo/bin:$PATH" >> ~/.zshrc
```

```sh
cargo --version
rustc --version
rustdoc --version
```

## プロジェクト作成

```sh
cargo new hello_world
cd hello_world
cargo run
```

## 101

### 文法サマリー

- useキーワードは、Rust言語の特定の機能や関数をスコープ内に取り込むために使用します。ここでは、useキーワードを使用して、std（標準ライブラリ）からFromStr traitとenvモジュールを呼び出しています。
- fnキーワードは関数を定義するために使用され、mainはRustプログラムのエントリポイントになります。
- letキーワードは、新しい変数numbersを定義し、Vec::new()を使用して空のVecに初期化するために使用されます。
- forループは、env::args()関数を使ってプログラムに渡されるコマンドライン引数の各引数を繰り返し処理するために使用されます。skip(1)メソッドは、最初の引数（プログラム自体の名前）をスキップするために使用されます。
- push()メソッドは、u64::from_str()で符号なし64ビット整数に変換した後、各引数を数値ベクトルに追加するために使用されます。
- if文は、数値ベクトルの長さが0であるかどうかをチェックします。もしそうなら、標準エラーストリームにエラーメッセージを表示し、std::process::exit()を使ってプログラムを終了する。
- gcd()関数は、2つの符号なし64ビット整数を入力とし、それらの最大公約数を返します。GCDの計算にはユークリッドアルゴリズムが使用されます。
- test_gcd()関数は、assert_eq! マクロを使用してgcd()関数のユニットテストを定義し、期待される出力と実際の出力を比較します。

### リンク

```sh
rustup doc --std
```