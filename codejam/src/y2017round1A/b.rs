use super::super::util::input::read_int_line;
use std::io::stdin;
use std::thread;

pub fn solve_all_cases()
{
    let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();

    for case in 1..=t
    {
        //handle input / output
        let n_and_p: Vec<u8> = read_int_line();
        let (n, p) = (n_and_p[0], n_and_p[1]);

        let r: Vec<u32> = read_int_line();

        let mut q: Vec<Vec<u32>> = Vec::new();
        for _ in 0..n
        {
            q.push(read_int_line());
        }
        children.push(thread::spawn(move || -> String {
            solve(case, n, p, &r, &q)
        }));
    }

    for child in children
    {
        print!("{}", child.join().unwrap());
    }
}

#[allow(non_snake_case)]
fn solve(case_no: u32, N: u8, P: u8, R: &Vec<u32>, Q: &Vec<Vec<u32>>) -> String
{
    debug!("\nStarting solve");
    let mut events: Vec<_> = Vec::new();
    for i in 0..N as usize
    {
        let required_amount = R[i];

        for p in 0..P as usize
        {
            let package_size = Q[i][p];

            // problem is floating point
            //min_servings = math.ceil(package_size / (1.1 * required_amount) )
            // max_servings = math.floor( package_size / (.9 * required_amount) )

            let max_servings = (10 * package_size) / (9 * required_amount);
            let mut min_servings =
                (10 * package_size + 11 * required_amount - 1) / (11 * required_amount);

            debug!(
                "For ingredient {i}, package # {p}. \
                 Required per serving = {required_amount} \
                 Package size = {package_size} \
                 Min = {min_servings} Max = {max_servings}",
                i = i,
                p = p,
                required_amount = required_amount,
                package_size = package_size,
                min_servings = min_servings,
                max_servings = max_servings
            );

            if min_servings == 0
            {
                min_servings = 1;
            }

            if min_servings > max_servings
            {
                continue;
            }

            events.push((min_servings, false, i, package_size));
            events.push((max_servings, true, i, package_size));
        }
    }

    // Code based on https://www.go-hero.net/jam/17/name/Nore
    events.sort();
    let mut cnt = 0;
    let mut counts = vec![Vec::new(); N.into()];
    let mut remv = vec![0; N as usize];
    for (boundary, is_upper_bound, ingredient_index, package_size) in events
    {
        debug!(
            "Saw event Boundary={} {} ingredient={} package={}",
            boundary, is_upper_bound, ingredient_index, package_size
        );

        debug!("Counts={:?}, remv={:?}", counts, remv);
        if is_upper_bound
        {
            if remv[ingredient_index] > 0
            {
                remv[ingredient_index] -= 1;
            }
            // elif yy in counts[i]:
            else
            {
                let index = counts[ingredient_index]
                    .iter()
                    .position(|x| *x == package_size)
                    .unwrap();
                counts[ingredient_index].remove(index);
            }
        }
        else
        {
            counts[ingredient_index].push(package_size);
            let min_count_len = counts.iter().map(|c| c.len()).min().unwrap();
            if min_count_len > 0
            {
                cnt += 1;
                for ii in 0..N as usize
                {
                    let min_index = counts[ii]
                        .iter()
                        .enumerate()
                        .min_by_key(|&(_, item)| item)
                        .unwrap()
                        .0;
                    counts[ii].remove(min_index);
                    remv[ii] += 1;
                }
            }
        }
    }

    format!("Case #{}: {}\n", case_no, cnt)
}
