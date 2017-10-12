use std::io::BufRead;

const SIZE:usize = 10;

fn add(arr: &mut [i32], pos: i32, amount: i32) {
    if pos > SIZE as i32 {
        return;
    }

    arr[pos as usize] += amount;

    add(arr, pos + (pos & -pos), amount);
}

fn sum(arr: &[i32], goal: i32) -> i32 {
    if goal == 0 {
        return 0;
    }

    return arr[goal as usize] + sum(arr, goal - (goal & -goal));
}

fn query(arr: &[i32], from: i32, to: i32) -> i32 {
    return sum(arr, to) - sum(arr, from);
}

fn main() {
    let stdin = std::io::stdin();
    let numcases:i32 = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();
    let mut arr = [0; SIZE+1];

    for _i in 0..numcases {
        let case:Vec<i32> = stdin.lock().lines().next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

        if case[0] == 0 {
            add(&mut arr, case[1], case[2]);
        } else {
            println!("{}", query(&arr, case[1]-1, case[2]));
        }

        // println!("{:?}", arr);
    }
}
