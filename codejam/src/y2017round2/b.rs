use super::super::util::input::*;
use super::super::util::math::*;
use std::cmp::max;

/*
custom algorithm / proof
minimization / optimization
*/

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (N, C, M) = reader.read_tuple_3::<u16, u16, u16>();
        //P B
        let tickets: Vec<_> = (0..M).map(|_| reader.read_tuple_2::<u16, u16>()).collect();

        print!("{}", solve(case, N, C, &tickets));
    }
}

fn solve(case_no: u32, N: u16, C: u16, tickets: &Vec<(u16, u16)>) -> String
{
    debug!("Solving case {}", case_no);

    //first determine if a customer has multiple tickeets
    let max_tickets_per_customer: u16 = *tickets
        .iter()
        .fold(&mut vec![0; C as usize], |acc, &(_P, B)| {
            {
                acc[B as usize - 1] += 1;
            }
            acc
        })
        .iter()
        .max()
        .unwrap();
    debug!("Max tickets per customer: {}", max_tickets_per_customer);

    let mut tickets_per_position: Vec<u16> = vec![0; N as usize];

    tickets
        .iter()
        .fold(&mut tickets_per_position, |acc, &(P, _B)| {
            {
                acc[P as usize - 1] += 1;
            }
            acc
        });

    //List of position: # of tickets in that position
    let ticket_pos_list: Vec<(u16, u16)> = tickets_per_position
        .iter()
        .enumerate()
        .filter(|(_P, Count)| **Count > 0)
        .map(|(a, b)| (a as u16, *b))
        .collect();

    debug!("Position, Count pairs: {:?}", ticket_pos_list);

    let mut promote_space = 0;
    let mut last_pos: i16 = -1;
    let mut min_rides_needed = 1;
    let mut count_cumul = 0;
    //promote those closest to front, keeping track of how many spaces we didn't use
    for &(pos, count) in ticket_pos_list.iter() {
        promote_space += int_sub_us(pos, last_pos);
        count_cumul += count;
        min_rides_needed = max(min_rides_needed, int_div_ceil(count_cumul, promote_space));

        last_pos = pos as i16;
    }

    let rides_needed = max(min_rides_needed, max_tickets_per_customer);

    let mut promotions_needed = 0;

    for (_pos, count) in ticket_pos_list {
        promotions_needed += max(0i16, int_sub(count, rides_needed));
    }

    format!(
        "Case #{}: {} {}\n",
        case_no, rides_needed, promotions_needed
    )
}
