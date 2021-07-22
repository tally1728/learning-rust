// 標準ライブラリの入出力ライブラリを導入
use std::io;

// メイン関数
fn main() {
    // 文字列 "Guess the number!" を表示するマクロ
    println!("Guess the number!");

    // 文字列 "Please input your guess." を表示するマクロ
    println!("Please input your guess.");

    // 1. 可変変数 guess を定義
    // 2. String 型の関連関数（スタティックメソッド）new を呼び出して、
    // 空の String オブジェクト（空の文字列）を生成
    // pub const fn new() -> String
    // 3. その String オブジェクトを guess へ代入
    let mut guess = String::new();

    // IOモジュール（入出力ライブラリ）の stdin 関数を呼び出して、
    // 標準入力へのハンドルを表す Stdin オブジェクトを生成
    // pub fn stdin() -> Stdin
    io::stdin()
        // Stdin オブジェクトの read_line メソッドを呼び出して、
        // ユーザからの入力を受け付ける。
        // 引数として変数 guess への可変な参照を渡すことで、
        // ユーザ入力が変数 guess に追記される。
        // 処理の成功または失敗を表す Result オブジェクトを返す。
        // pub fn read_line(&self, buf: &mut String) -> Result<usize>
        .read_line(&mut guess)
        // Result オブジェクトの expect メソッドを呼び出す。
        // もし read_line の処理が失敗した場合、
        // 文字列 "Failed to read line" とエラー内容を含む
        // パニックメッセージを表示して、プログラムを終了させる。
        // もし read_line の処理が成功した場合、
        // Ok 列挙子が保持する値、つまりユーザが入力したデータのバイト数を返す。
        // pub fn expect(self, msg: &str) -> T
        .expect("Failed to read line");

    // 文字列 "You guessed: " と guess の値を表示するマクロ
    // {} がプレースホルダーとなっている。
    println!("You guessed: {}", guess);
}
