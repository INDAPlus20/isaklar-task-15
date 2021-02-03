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

        let mut tree: Vec<(usize, usize, usize)> = Vec::with_capacity(n);

        // fill with pointers
        for i in 0..(n) {
            tree.push((i, i + 1, 1));
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
                        // make the longest branch the root
                        if tree[p_root].2 <= tree[q_root].2 {
                            // change parent pointer of the set containing q
                            tree[p_root].0 = q_root;
                            // update sum for root node
                            tree[q_root].1 += tree[p_root].1;
                            // update count for root node
                            tree[q_root].2 += tree[p_root].2;
                        } else {
                            // change parent pointer of the set containing q
                            tree[q_root].0 = p_root;
                            // update sum for root node
                            tree[p_root].1 += tree[q_root].1;
                            // update count for root node
                            tree[p_root].2 += tree[q_root].2;
                        }
                    }
                }
                2 => {
                    let p = command.next().unwrap().parse::<usize>().unwrap() - 1;
                    let q = command.next().unwrap().parse::<usize>().unwrap() - 1;
                    let p_root = find_root(&mut tree, p);
                    let q_root = find_root(&mut tree, q);
                    if p_root != q_root {
                        // move p to q
                        // save parent
                        let p_parent = tree[p].0;
                        // set new parent
                        tree[p].0 = q_root;
                        if p != p_parent {
                            // find any child of p and change their parent pointers to p_parent
                            let mut found = 0;
                            let children = tree[p].2 - 1;
                            for i in 0..tree.len() {
                                if i != p {
                                    // skip p
                                    if tree[i].0 == p {
                                        tree[i].0 = p_parent;

                                        found += 1;
                                        if found >= children {
                                            break;
                                        }
                                    }
                                }
                            }
                            // update previous parent sum and count
                            tree[p_parent].1 -= (p + 1);
                            tree[p_parent].2 -= 1;
                        } else if p == p_parent && tree[p].2 != 1 {
                            // if its a root node and non-singular
                            // find any child of p and determine new root node
                            let mut new_root = p;
                            let mut found = 0;
                            let children = tree[p].2 - 1;
                            for i in 0..tree.len() {
                                if i != p {
                                    // skip p
                                    if tree[i].0 == p {
                                        if new_root == p {
                                            // make this new root
                                            tree[i].0 = i;
                                            new_root = i;
                                        } else {
                                            // connect to new
                                            tree[i].0 = new_root;
                                            // update sum for root node
                                            tree[new_root].1 += tree[i].1;
                                            // update count for root node
                                            tree[new_root].2 += tree[i].2;
                                        }
                                        found += 1;
                                        if found >= children {
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            // if its a singular node
                        }

                        // reset sum and count
                        tree[p] = (q_root, p + 1, 1);

                        // update new parent sum and count
                        tree[q_root].1 += tree[p].1;
                        tree[q_root].2 += tree[p].2;
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
