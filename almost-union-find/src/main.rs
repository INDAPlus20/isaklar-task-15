use std::io::prelude::*;
use std::{io, usize};

fn main() {
    // allocate memory for the input buffer
    let mut buf: String = String::with_capacity(100);

    // get input lines as strings
    io::stdin().read_to_string(&mut buf).expect("err");

    // split by line
    let lines: Vec<&str> = buf.split('\n').collect();

    let mut l = 0;
    while l < lines.len() {
        if lines[l] == "" {
            l += 1;
            continue;
        }
        let mut first = lines[l].split_whitespace();

        let n: usize = first.next().unwrap().parse().unwrap();
        let m: usize = first.next().unwrap().parse().unwrap();
        //eprintln!("n = {} m = {}", n, m);

        // (parent, sum, count)
        let mut tree: Vec<(usize, usize, usize)> = vec![(0,0,0); 2*n ];

        // fill with pointers
        // i + n contains the sums and count
        for i in 0..n  {
            tree[i].0 = i + n;
            tree[i + n] = (i + n, i + 1, 1);
        }

        // parse commands
        for i in (l + 1)..(l + m + 1) {
            //eprintln!("{:?}", tree);

            let mut command = lines[i].split_whitespace();
            match command.next().unwrap().parse::<u32>().unwrap() {
                1 => {
                    let p = command.next().unwrap().parse::<usize>().unwrap() - 1;
                    let q = command.next().unwrap().parse::<usize>().unwrap() - 1;
                    let p_root = find_root(&mut tree, p);
                    let q_root = find_root(&mut tree, q);
                    if p_root != q_root {
                        // change parent pointer of the set containing q
                        tree[p_root].0 = q_root;
                        // update sum for root node
                        tree[q_root].1 += tree[p_root].1;
                        // update count for root node
                        tree[q_root].2 += tree[p_root].2;
                  
                    }
                }
                2 => {
                    let p = command.next().unwrap().parse::<usize>().unwrap() - 1;
                    let q = command.next().unwrap().parse::<usize>().unwrap() - 1;
                    let p_root = find_root(&mut tree, p);
                    let q_root = find_root(&mut tree, q);
                    if p_root != q_root {
                        // move p to q
                        tree[p].0 = q_root;
                        // update old parent sum and count
                        tree[p_root].1 -= (p + 1);
                        tree[p_root].2 -= 1;
                        // update new parent sum and count
                        tree[q_root].1 += (p + 1);
                        tree[q_root].2 += 1;
                    }
                }
                3 => {
                    let p = command.next().unwrap().parse::<usize>().unwrap() - 1;
                    let root = find_root(&mut tree, p);
                    let (sum, count) = sum_count(&tree, root);
                    println!("{:?} {:?}", count, sum);
                }
                _ => panic!(),
            }
        }

        l += m + 1;
        //eprintln!("l = {}", l);
    }
}

// finds the root and flattens the tree
fn find_root(tree: &mut Vec<(usize, usize, usize)>, e: usize) -> usize {
    if e != tree[e].0 {
        tree[e].0 = find_root(tree, tree[e].0);
    }

    return tree[e].0;
}

fn sum_count(tree: &Vec<(usize, usize, usize)>, root: usize) -> (usize, usize) {
    let (_, sum, count) = tree[root];

    return (sum, count);
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_count_correctness() {
        let mut tree = vec![0, 0, 4, 0, 4];
        let (sum, count) = sum_count(&tree, 4);
        println!("{:?} {:?}", count, sum);
    }
}
*/
