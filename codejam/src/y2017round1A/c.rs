use super::super::util::input::read_int_line;
use std::cmp;
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
        //Hd, Ad, Hk, Ak, B, and D;
        let input: Vec<i64> = read_int_line();

        children.push(thread::spawn(move || -> String {
            solve(
                case, input[0], input[1], input[2], input[3], input[4], input[5],
            )
        }));
    }

    for child in children
    {
        print!("{}", child.join().unwrap());
    }
}

const R: i64 = 100;

#[allow(non_snake_case)]
fn solve(case_no: u32, Hd: i64, Ad: i64, Hk: i64, Ak: i64, B: i64, D: i64) -> String
{
    let MAX_TURN = 10i64.pow(15);
    let mut Ak = Ak;
    let b;
    if B == 0
    {
        b = 0;
    }
    else
    {
        let z: f64 = (((B * Hk) as f64).sqrt() - Ad as f64) / B as f64;
        if z < 0f64
        {
            b = 0;
        }
        else
        {
            let b1 = z as i64;
            let b2 = b1 + 1;
            if (Hk + b1 * B + Ad - 1) / (b1 * B + Ad) + b1
                <= (Hk + b2 * B + Ad - 1) / (b2 * B + Ad) + b2
            {
                b = b1;
            }
            else
            {
                b = b2;
            }
        }
    }
    let a = (Hk + b * B + Ad - 1) / (b * B + Ad);
    let ab = a + b;

    let mut minturn = MAX_TURN;
    if ab <= (Hd - 1) / Ak + 1
    {
        minturn = ab;
    }
    else if D == 0
    {
        let u = (Hd - Ak - 1) / Ak;
        if u <= 0 && Hk > Ad
        {
            minturn = MAX_TURN;
        }
        else
        {
            let w = ab - (Hd - 1) / Ak;
            minturn = (Hd - 1) / Ak + w + (w + u - 2) / u;
        }
    }
    else if Hk > Ad && Ak - D >= Hd
    {
        minturn = MAX_TURN;
    }
    else
    {
        let mut z = 0;
        let mut last_was_heal = false;
        let mut Hi = Hd;
        let mut lastu: i64 = -1;
        while Ak > 0
        {
            let u = (Hd - Ak - 1) / Ak;
            if ab <= (Hi - 1) / Ak + 1
            {
                minturn = cmp::min(minturn, z + ab);
            }
            else if u > 0 && !last_was_heal && u as i64 != lastu
            {
                let w = ab - (Hi - 1) / Ak;
                minturn = cmp::min(minturn, z + (Hi - 1) / Ak + w + (w + u - 2) / u);
            }
            if Hi <= Ak - D
            {
                if last_was_heal
                {
                    break;
                }
                z += 1;
                last_was_heal = true;
                Hi = Hd - Ak;
                if Ak >= R * D && u == (Hd - (Ak - R * D) - 1) / (Ak - R * D)
                {
                    let tt = R / u;
                    z += tt * (u + 1);
                    Ak -= tt * u * D;
                    Hi = Hd - Ak;
                }
            }
            else
            {
                last_was_heal = false;
                z += 1;
                Ak -= D;
                Hi -= Ak;
            }
            lastu = u as i64;
        }
        if Ak <= 0
        {
            minturn = cmp::min(minturn, z + ab);
        }
    }

    if minturn == MAX_TURN
    {
        format!("Case #{}: {}\n", case_no, "IMPOSSIBLE")
    }
    else
    {
        format!("Case #{}: {}\n", case_no, minturn)
    }
}
