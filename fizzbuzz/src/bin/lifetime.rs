/**
 * 参照先がない状態、`dangling pointer`はC言語ではプログラマが気をつけなければいけない。
 * ライフタイムにより、rustではそれがない。
 * 値を貸している間に参照先をムーブしようとすると、コンパイル時にエラーになる。
 */

fn main() {
    // 本来`s`のライフタイムはこの関数の最後まで。
    let s = "owned data".to_string();

    // `{}`で囲んだブロックはライフタイムを区切る。
    {
        // `s`はここで、ムーブしてしまうので、ここでライフタイム終わり
        // `t`のライフタイムはこのブロックの終わりまで。
        let t = s;
    }
    // この時点で、`t`にも`s`にもアクセスできない。

    {
        let s = "owned data".to_string();
        // ここで`s`への参照を作る。この参照はこのブロックの最後で死ぬが、
        // `s`のほうが長生きしないといけない。
        let ref_s = &s;
        // let t = s;
    }
}
