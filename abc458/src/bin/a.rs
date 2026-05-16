use proconio::input;

fn main() {
    input! {
        s: String,
        n: usize,
    }

    let chars: Vec<char> = s.chars().collect();

    for i in n..chars.len() - n {
        print!("{}", chars[i]);
    }

    println!();

}

// printで一つずつ表示後、末尾でprintlnをするよりも
// 事前にlet mut answer = String::new();で変数を用意し、answer.push(chars[i])とした方がよい


// 別解

/**
 * sからchars()で一つずつ取り出し、skipでn飛ばし、takeでsの全体の長さから、先頭と末尾nを引いた分を取得し、collectで実体化
 * let ans: String = s.chars().skip(n).take(s.len - 2 * n).collect()
 */
