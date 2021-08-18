# bolic-rs

[『Rubyで作る奇妙なプログラミング言語』](http://esolang-book.route477.net/)に掲載されているesolang、BolicのRust実装  

## Usage

```bash
$ cargo run -- [<Bolic code file path>]
```

## Overview

- Bolicコードのトークンは、UTF-8に登録されている絵文字を使用する
- 以下に示す以外の文字は処理系によって無視される

    - ``⓪`` - ``⑩`` : 0から10の数値リテラル
    - ``＋``, ``−``, ``×``, ``÷``, : 四則演算
    - ``✪``, ``✷``, ``✲``, ``✩`` : 変数として使用可能なシンボル
    - ``☜`` : 変数に式の結果を代入
    - ``✈``, ``☺``, ``☹``, ``☻`` : ``if``, ``then``, ``else``, ``end``
    - ``♺``, ``☞``, ``♘`` : ``while``, ``do``, ``end``
    - ``✍`` : 式の結果（64bit整数）を数字出力
    - ``♪`` : 式の結果（64bit整数）をASCIIコードと解釈して文字出力

- さらなる詳細は本書を参照されたし

## LICENSE

-  ``examples``ディレクトリ以下は著者の [原悠](https://github.com/yhara/esolang-book-sources) 氏に帰属
- それ以外はMIT
