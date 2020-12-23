use prettytable::{Table, Row, Cell};

const TEST_DATA: &str = "1001798
7,13,x,x,59,x,31,19";

#[derive(Debug)]
struct SchedTimes {
    times: Vec<i32>
}

impl SchedTimes {
    fn closest_to(&self, n: i32) -> (i32, i32) {
        self
            .times
            .iter()
            .map(|bid| {
                let mut current_iter = 1;

                loop {
                    let amt = bid * current_iter;

                    if amt >= n {
                        return (*bid, amt);
                    }

                    current_iter += 1;
                }
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
    }

    fn print_debug(&self, n: i32) {
        let mut table = Table::new();

        let mut headers = self.times
            .iter()
            .map(|n| Cell::new(&format!("bus {}", n)))
            .collect::<Vec<Cell>>();

        headers.push(Cell::new("time"));
        table.add_row(Row::new(headers));

        let columns = self
            .times
            .iter()
            .map(|bid| {
                let mut current_iter = 1;

                let mut nums = Vec::new();

                loop {
                    let amt = bid * current_iter;

                    nums.push(amt);

                    if amt >= n {
                        break
                    }

                    current_iter += 1;
                }

                nums
            })
            .collect::<Vec<Vec<i32>>>();

        for time in n - 10..n + 100 {
            let mut cells =
                columns
                    .iter()
                    .enumerate()
                    .map(|(i, arr)| {
                        return if arr.contains(&time) {
                            Cell::new("D")
                        } else if time == n {
                            Cell::new("-----")
                        } else {
                            Cell::new(".")
                        }
                    })
                    .collect::<Vec<Cell>>();

            cells.push(Cell::new(&format!("{}", time)));
            table.add_row(Row::new(cells));
        }

        table.printstd();
    }
}

#[derive(Debug)]
struct SchedTimesB {
    times: Vec<(usize, Option<i32>)>
}

impl SchedTimesB {
    fn find_sequential(&self) {
        let mut n = 1;

        loop {
            let mut invalid = false;

            for i in 0..self.times.len() {
                let (bus_idx, bus_id) = self.times[i];

                if bus_id.is_some() && (n + bus_idx) as i32 % bus_id.unwrap() != 0 {
                    invalid = true;
                    break
                }
            }

            if !invalid {
                println!("FOUND {}", n);
                return
            }

            n += 1;
        }
    }
}

fn day_13_a(input: &str) {
    let lines: Vec<&str> = input.split("\n")
        .collect();

    let arrival: i32 = lines[0].parse().unwrap();

    let sched_times = SchedTimes{
        times: lines[1]
            .split(",")
            .filter_map(|bus| match bus {
                "x" => None,
                id => Some(id.parse().unwrap())
            })
            .collect::<Vec<i32>>()
    };

    let (bus_id, minute) = sched_times.closest_to(arrival);

    println!("{:?} {:?}", arrival, (minute - arrival) * bus_id);
}

fn day_13_b(input: &str) {
    let lines: Vec<&str> = input.split("\n")
        .collect();

    let sched_times = SchedTimesB{
        times: lines[1]
            .split(",")
            .enumerate()
            .map(|(i, bus)| match bus {
                "x" => (i, None),
                id => (i, Some(id.parse().unwrap()))
            })
            .collect()
    };

    sched_times.find_sequential();

    println!("{:?}", sched_times);
}


#[test]
fn test_day_13_a() {
    day_13_a(TEST_DATA)
}

// #[test]
pub fn test_day_13_b() {
    day_13_b(TEST_DATA)
}