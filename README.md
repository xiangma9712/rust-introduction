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

CLIインターフェースで、値を受け取り、最大公約数を計算するプログラムを作成する。
[Code](./10X/hello/src/main.rs)

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

## 102

Actix Webを使用して、HTTPサーバーを作成する。
[Code](./10X/actix-gcd/src/main.rs)

### 文法サマリー

- modキーワードは、gcd_libという名前のモジュールを定義するために使用されます。
- useキーワードは、特定のRust言語の機能または関数をスコープに入れるために使用します。ここでは、actix_webクレート、serdeクレート、別のファイルで定義されたgcd_libモジュールなど、プログラムに必要なさまざまな依存関係を取り込むためにuseキーワードが使用されています。
- post_gcd関数では、if文を使用して、入力パラメータnまたはmのいずれかが0に等しいかどうかをチェックします。もしそうなら、エラーメッセージとともにHTTP 400 Bad Requestエラーを返します。
- content_type() および body() メソッドは、それぞれ HTTP レスポンスのコンテンツタイプおよびボディを設定するために使用されます。
- web::get().to() メソッドは、HTTP GET リクエストを get_index 関数にルーティングするために使用されます。
- web::post().to() メソッドは、HTTP POST リクエストを post_gcd 関数へルーティングするために使用される。
- struct キーワードを使用して、post_gcd 関数への入力パラメータを表す GcdParameters という新しいデータ構造を定義します。
- #[derive(Deserialize)]属性は、GcdParameters構造体のDeserialize特性を自動的に実装するために使用される。
- format!()マクロは、gcd_libモジュールで定義されたgcd()関数の呼び出し結果を含む文字列を構築するために使用されます。

## 103

crossbeamを使用して、並列処理を行う。
[Code](./10X/mandelbrot/src/main.rs)

```sh
cd 10X/mandelbrot
cargo build --release
target/release/mandelbrot mandelbrot.png 4000x3000 -1.20,0.35 -1,0.20
```

## 104

コマンドラインツールを作成する。
[Code](./10X/quickreplace/src/main.rs)

### 文法サマリー

- このコードでは、「use」キーワードを使用して、「std」ライブラリと「regex」ライブラリから、それぞれ「fs」モジュールと「Regex」モジュールをインポートしています。
- Arguments構造体は、4つのフィールドを持つ： 「pattern"、"replacement"、"filename"、"output "です。
- このコードでは、「fs::read_to_string」、「replace」、「fs::write」関数が返す可能性のある成功値やエラー値を処理するために「match」キーワードを使っています。
- env::args関数を呼び出してコマンドライン引数のリストを取得し、「parse_arguments」関数を使用して解析する。
- std::process::exit関数を使用して、エラーが発生した場合、0以外の終了コードでプログラムを終了します。
- cloneメソッドを使用して、既存のインスタンスから「String」型の新しいインスタンスを作成します。
- skipメソッドを使用して、コマンドライン引数のリストの最初の引数（プログラム自体の名前）をスキップするコードです。
