fn main(){
    // ベクトルを用意する。
    let mut vec = vec![1,2,3];
    
    // //ベクトルの要素への参照を取り出す。
    // ベクトルをイミュータブルに参照する。
    for i in vec.iter(){
        // しかし、すでにイミュータブルに参照されているので、
        // ここでベクトルを変更しようとすると、エラー。
        vec.push(i * 2);
    }
}