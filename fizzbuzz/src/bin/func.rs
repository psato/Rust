/* 
関数も、第一級の値として扱うことができる。
関数型プログラミングで関数を関数に渡すなどしてよく用いられる。
map()はリストの各要素に関数を適用した結果のリストを返す関数。
*/ 

fn add(x: isize, y:isize) -> isize {
    x + y
}

fn main(){
    // 関数は`名前（引数）`で呼び出せる。
    println!("{}", add(1, 2)); // 3

    // 関数を変数に束縛できる。
    let f: fn(isize, isize) -> isize = add;

    // 変数に束縛した関数を`名前（引数）`呼び出せる。
    let a = f(1, 2);
    println!("{}", a) // 3
}