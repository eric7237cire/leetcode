use std::io::stdin;

pub fn solve_case()
{
    //handle input / output
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    debug!("Read {}", s);
    let nums: Vec<u64> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();

    let ans = solve(nums[0], nums[1]);
    println!("{} {}", ans.0, ans.1);
}

fn solve(n: u64, k: u64) -> (u64, u64)
{
    if n == k {
        return (0, 0);
    }

    let mut tree_height = 0;

    // Find how big a balanced binary tree we need
    // Tree of height 1 has 1 node
    // h=2 = 3 nodes
    // h=3 = 7 nodes == 2^3 - 1
    for s in 1..=k + 1 {
        if 2u64.pow(s as u32) - 1 >= k {
            tree_height = s;
            break;
        }
    }

    // Find out how many holes taken by the tree up to the last row

    //holes_left = n - 2**(tree_size-1) + 1

    // Place k-1 folks
    let holes_left = n - (k - 1);
    let width_tree = 2u64.pow(tree_height as u32 - 1);

    // How many empty stalls?
    let min_hole_size = holes_left / width_tree;

    debug!("n = {} k = {} tree_height = {}", n, k, tree_height);

    // If there is a remainder, add 1
    let an_extra = holes_left % width_tree > 0;

    let hole_size = if an_extra {
        min_hole_size + 1
    } else {
        min_hole_size
    };

    // We get placed in the middle
    let min_dist = (hole_size - 1) / 2;

    // If its odd, we have an even min/max distance
    if hole_size % 2 == 1 {
        return (min_dist, min_dist);
    } else {
        // Otherwise max is 1 more
        return (min_dist + 1, min_dist);
    }
}
