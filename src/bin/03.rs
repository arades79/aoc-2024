advent_of_code::solution!(3);

use winnow::{ascii::dec_int, combinator::{alt, delimited, opt, preceded, repeat, separated, separated_pair, terminated}, prelude::*, token::{any, take, take_till, take_until}};

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
struct Mul(i32,i32);

impl Mul {
    fn eval(self) -> i64 {
        self.0 as i64 * self.1 as i64
    }
}

fn mul(input: &mut &str) -> PResult<Mul> {
   let (v1,v2) = preceded("mul", delimited('(', separated_pair(dec_int,',',dec_int), ')')).parse_next(input)?;
   Ok(Mul(v1,v2))
}

fn mul_garbo(input: &mut &str) -> PResult<Mul> {
    let (v1,v2) = preceded(('m',opt(any),'u',opt(any),'l',opt(any)), delimited((opt(any),'(',opt(any)), separated_pair(dec_int,(opt(any),',',opt(any)),dec_int), (opt(any),')',opt(any)))).parse_next(input)?;
    Ok(Mul(v1,v2))
 }

fn mul_through_garbage(input:&mut &str) -> PResult<Vec<Mul>> {
    let mut mulls = Vec::new();
    while input.len() > 0 {
       match mul.parse_next(input) {
        Ok(m) => mulls.push(m),
        Err(_) => {take::<_,_,()>(1u32).parse_next(input).ok();}
       }
    }
    Ok(mulls)
}

pub fn part_one(input: &str) -> Option<i64> {
    let muls = mul_through_garbage.parse(input).ok()?;
    let sum = muls.into_iter().map(Mul::eval).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
       let m = mul.parse("mul(1,2)").unwrap();
       assert_eq!(m, Mul(1,2));
       assert!(mul.parse("notmul").is_err())
    }

    #[test]
    fn test_garbo() {
        let m = mul_through_garbage.parse("fsfsdmul(1,2)fdsifdj").expect("shuold get mul");
        assert_eq!(m, vec![Mul(1,2)]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
