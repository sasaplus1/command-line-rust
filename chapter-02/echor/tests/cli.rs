use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

/*
 * コンピュータのメモリには大きく分けて「スタック」と「ヒープ」という二つの領域があります。
 * それぞれの領域には特性と使い方があり、それぞれの特性を理解することで、RustのBoxがなぜヒープにデータを格納するのかを理解することができます。
 *
 * 1. スタック：スタックは高速にアクセスできるメモリ領域で、関数の呼び出しとともに領域が確保され、関数の終了とともに領域が解放されます。
 * そのため、サイズが事前に決まっていて、生存期間が短いデータに適しています。
 * しかし、スタックのサイズは限られており、大量のデータを扱うことは難しいです。
 *
 * 2. ヒープ：ヒープは動的にサイズを変更できるメモリ領域で、必要に応じてデータを格納したり削除したりすることができます。
 * そのため、サイズが大きいデータや生存期間が長いデータに適しています。
 * しかし、ヒープはスタックに比べてアクセス速度が遅く、またメモリ管理（確保と解放）をプログラマが行う必要があります。
 *
 * RustのBoxは、ヒープにデータを格納するためのスマートポインタです。
 * Boxを使うと、コンパイル時にサイズが決まらないデータや、大きなデータをスタックではなくヒープに格納することができます。
 * また、Boxはデータがスコープを抜けると自動的にヒープからデータを削除するため、メモリリークを防ぐことができます。
 *
 * つまり、RustのBoxを使うと、ヒープのメモリ管理を簡単に行いつつ、大きなデータや長寿命のデータを効率的に扱うことができる、というわけです。
 */
/*
 * このコードにおける`Box<dyn std::error::Error>は、「任意の種類のエラー」を表現できる型です。
 * dyn std::error::Error`は「`Error`トレイトを実装している任意の型」を意味します。
 *
 * Rustでは、具体的な型のサイズがコンパイル時に決まらないとスタックに格納することができません。
 * `dyn std::error::Error`は任意の型を表すため、そのサイズはコンパイル時に決まりません。
 * したがって、ヒープに格納する必要があります。
 *
 * この場合、`Box`を使うことで、具体的な型が何であるかに関わらず、エラーをヒープに格納し、一貫した方法で扱うことができます。
 * また、`Result<(), Box<dyn std::error::Error>>`の形式は、関数が成功した場合には何も返さず、エラーが発生した場合にはエラー情報を返す、という一般的なエラーハンドリングのパターンを表現しています。
 */
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();

    cmd.arg("hello").assert().success();
}

#[test]
fn dies_no_args() -> TestResult {
    /*
     * Rustのコードにおける `?` と `Ok(())` について説明します。
     *
     * まず、`?` についてですが、これはエラーハンドリングを行うための演算子です。
     * `?` がついている式が Result や Option 型を返し、その値が Err や None だった場合、その時点で関数の実行を中止し、そのエラー値を関数の呼び出し元に返します。
     * つまり、エラーの伝搬を行います。
     * これにより、エラーハンドリングのための冗長なコードを大幅に減らすことができます。
     *
     * 次に、`Ok(())` についてですが、これは Result 型の成功を表す値を作成しています。
     * `Result` 型は基本的に Result<T, E> という形で、`T` が成功時の値、`E` がエラー時の値を表します。
     * ここで、`()` はRustにおけるユニット型（unit type）で、何も値を持たないことを表します。
     * つまり、`Ok(())` は「成功したが、特に返すべき値はない」という意味になります。
     *
     * そのため、あなたが書いたコードは「`Command::cargo_bin("echor")? でエラーが起きたらすぐにテストを終了し、エラーを返す。
     * エラーが起きなければ、最後に Ok(())` を返してテスト成功を示す」という挙動を持ちます。
     */
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    /*
     * Rustでは関数の最後の式は自動的に戻り値となります。
     * セミコロンをつけると、その式は文となり、値を返さないようになります。
     * つまり、セミコロンをつけずに式を終えると、その式の値が関数の戻り値となります。
     */
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
