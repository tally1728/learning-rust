// randクレート（乱数生成器）を導入
use rand::Rng;
// ２つの値の比較結果（列挙型）を導入
use std::cmp::Ordering;
// 標準ライブラリの入出力ライブラリを導入
use std::io;

// メイン関数
fn main() {
    // 文字列 "Guess the number!" を表示するマクロ
    println!("Guess the number!");

    // 1. 不変変数 secret_number を定義
    // 2. rand::thread_rng() 関数を呼び出して、
    // ThreadRng オブジェクト（スレッドに固有な擬似乱数生成器）を生成
    // Function rand::thread_rng() -> ThreadRng
    // 3. Trait rand::Rng で定義された ThreadRng オブジェクトの
    // gen_range メソッドを呼び出して、1〜100の擬似乱数を生成
    // fn gen_range<T: PartialOrd + SampleUniform>(&mut self, low: T, high: T) -> T
    // 4. 生成された値を secret_number へ代入
    let secret_number = rand::thread_rng().gen_range(1..101);

    // 文字列 "The secret number is: " と secret_number の値を表示するマクロ
    // {} がプレースホルダーとなっている。
    println!("The secret number is: {}", secret_number);

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

    // guess の cmp メソッドを呼び出して、secret_number との大小関係を比較する。
    // 比較結果は Ordering オブジェクト（列挙型）として出力される。
    // 例えば guess が secret_number より大きい場合は、
    // 比較結果として値 Ordering::Greater を返す。
    // pub fn cmp(&self, other: &u32) -> Ordering
    // match 式で比較結果に該当する処理を実行する。
    match guess.cmp(&secret_number) {
        // 比較結果が Ordering::Less の場合、
        // つまり guess が secret_number より小さい場合、文字列 "Too small!" を出力
        Ordering::Less => println!("Too small!"),
        // 比較結果が Ordering::Greater の場合、
        // つまり guess が secret_number より大きい場合、文字列 "Too big!" を出力
        Ordering::Greater => println!("Too big!"),
        // 比較結果が Ordering::Equal の場合、
        // つまり guess と secret_number が等しい場合、文字列 "You win!" を出力
        Ordering::Equal => println!("You win!"),
    }
}
