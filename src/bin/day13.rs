use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day13>()
}

#[test]
fn test_day13_solution() {
    aoc2022::test_solution::<Day13>()
}

struct Day13;
impl Solution for Day13 {
    const DAY: aoc2022::Day = match Day::number(13) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let lines = &mut input.lines();
        let mut i = 1;
        let mut part1_sum = 0;
        let divider1 = Packet {
            list: vec![Value::List(vec![Value::Int(2)])],
        };
        let divider2 = Packet {
            list: vec![Value::List(vec![Value::Int(6)])],
        };

        let mut all_packets = vec![divider1.clone(), divider2.clone()];
        while let Some(line) = lines.next() {
            let a = packet_parser::packet(line)?;
            let b = packet_parser::packet(lines.next().unwrap())?;
            lines.next();
            if a <= b {
                part1_sum += i;
            }
            i += 1;
            all_packets.extend_from_slice(&[a, b]);
        }
        all_packets.sort();
        let mut part2_product = 0;
        for (i, packet) in all_packets.into_iter().enumerate() {
            if packet == divider1 {
                part2_product = i + 1;
            } else if packet == divider2 {
                part2_product *= i + 1;
                break;
            }
        }
        Ok((part1_sum, part2_product))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Packet {
    list: Vec<Value>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Int(u8),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Int(a), Value::List(b)) => vec![Value::Int(*a)].cmp(b),
            (Value::List(a), Value::Int(b)) => a.cmp(&vec![Value::Int(*b)]),
            (Value::List(a), Value::List(b)) => a.cmp(b),
        }
    }
}

peg::parser! {
  grammar packet_parser() for str {
    pub rule packet() -> Packet
        = list:list() { Packet { list }}

    rule value() -> Value
        = list:list() { Value::List(list) } / int:int() { Value::Int(int) }

    rule int() -> u8
        = n:$(['0'..='9']+) {? n.parse().or(Err("u8")) }

    rule list() -> Vec<Value>
        = "[" list:(value() ** ",") "]" { list }
  }
}
