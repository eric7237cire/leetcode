
#[test]
fn test_helper1()
{
    //init_log();
    let mut sol: Vec<Colors> = vec![Red, Yellow, Blue, Red, Yellow];
    let mut counts: Counts = Counts::new();
    counts.adj_count(Blue, 1);
    let r = primary_color_sol(&mut sol, &mut counts);
    assert!(r, "snht");
}

#[test]
fn test_helper2()
{
    //init_log();
    let mut sol: Vec<Colors> = vec![Red, Red];
    let mut counts = Counts::new();
    counts.adj_count(Blue,  1);
    counts.adj_count(Yellow, 1);
    let r = primary_color_sol(&mut sol, &mut counts);
    assert!(r, "sam");
}
#[test]
fn test_helper3()
{
    //init_log();
    let mut sol: Vec<Colors> = vec![];
    let mut counts = Counts::new();
    counts.adj_count(Blue,  2);
    counts.adj_count(Yellow, 4);
    counts.adj_count(Red, 2);
    let r = primary_color_sol(&mut sol, &mut counts);
    assert!(r, "bob");
    assert_eq!(8, sol.len());
}

fn helper(sol: &mut Vec<Colors>, counts: &mut Counts, level: usize) -> bool
{
    return greedy_helper(sol, counts, level);

    let r_val = match counts.total
    {
        0 => true,
        1 =>
        {
            let max_color = counts.max_color();
            //check both ends
            if sol.first().unwrap().is_ok(max_color) && sol.last().unwrap().is_ok(max_color)
            {
                sol.push(max_color);
                counts.adj_count(max_color, -1);
                true
            }
            else
            {
                false
            }
        }
        _ =>
        {
            if counts.get_count(Red)
                > 1 + counts.get_count(Yellow) + counts.get_count(Blue)
            {
                false
            }
            else if counts.get_count(Yellow)
                > 1 + counts.get_count(Red) + counts.get_count(Blue)
            {
                false
            }
            else if counts.get_count(Blue)
                > 1 + counts.get_count(Yellow) + counts.get_count(Red)
            {
                false
            }
            else
            {
                let mut found = false;
                for idx in 0..6
                {
                    let color = COLORS[idx];
                    if counts.get_count(color) == 0
                    {
                        continue;
                    }
                    if !sol.is_empty() && !sol.last().unwrap().is_ok(color)
                    {
                        continue;
                    }
                    sol.push(color);
                    counts.adj_count(color, -1);
                    let ok = helper(sol, counts, level + 1);
                    if ok
                    {
                        found = true;
                        break;
                    }
                    else
                    {
                        sol.remove(sol.len() - 1);
                        counts.adj_count(color, 1);
                    }
                }

                found
            }
        }
    };

    if counts.total > 0 && sol.len() > 0
    {
        debug!(
            "Level {} Helper sol: {:?}-{:?} size:{} n: {} counts: {:?} ret={}",
            //" ".repeat(level * 2),
            level,
            sol.first().unwrap(),
            sol.last().unwrap(),
            sol.len(),
            counts.total,
            counts.count
                .iter()
                .zip(COLORS.iter())
                .map(|(cnt, col)| format!("{:?}: {}", col, cnt))
                .collect::<Vec<String>>()
                .join("; "),
            r_val
        );
    }

    r_val
}
