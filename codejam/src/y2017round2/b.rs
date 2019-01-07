use super::super::util::input::*;
//use std::cmp::max;

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (N, C, M) = reader.read_tuple_3::<u16, u16, u16>();
        //P B
        let tickets: Vec<_> = (0..M).map(|_| reader.read_tuple_2::<u16,u16>()).collect();

        print!("{}", solve(case, N, C, &tickets));
    }
}

fn solve(case_no: u32, N: u16, C:u16, tickets: &Vec<(u16,u16)>) -> String
{
    debug!("Solving case {}", case_no);

    //first determine if a customer has multiple tickeets
    let max_tickets_per_customer:u16 = *tickets.iter().fold::<&mut Vec<u16>, _>(
        &mut vec![0; N as usize], | acc , &(_P,B)| {
            {
                acc[B as usize - 1] += 1;
            }
            acc
        }).iter().max().unwrap();
    debug!("Max tickets per customer: {}", max_tickets_per_customer);

    format!("Case #{}: {}\n", case_no, max_tickets_per_customer)
}
