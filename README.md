# 開発メモ

とりあえず CDK 無しで、aws-lambda-rust-runtime を触ってみる。

Build: `cargo lambda build --release --arm64`

↑これでLambdaデプロイできる。M1 Mac でビルドしても動いてくれた。

`target/lambda/aws-cost-notifier/bootstrap`にバイナリが生成される。

`cargo lambda deploy`でいい感じにzipをデプロイしてくれるっぽい。
この部分をCDKで実行できればOKな気がする。