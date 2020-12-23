const TEST_DATA: &str = "LLLLLL.LL.LLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLLLL.LLLL.L.LLL.LLLLLLLL.LLLLLLLL.LLLLL
LLLLLL.L.LL.LLLL.L.LLLL.LLLLLLL.LLLLL.LLLLLLLL.LLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLL
LLLLLL.LLLL.LLLL.LLLLLLLLLLLLLL.LLLLL.LLLL.LLLLLLLLLLL.L.LLLL.LLLLL.LLL..LLL.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLLLLLL.L.LLLLLLLLLLLLLLLLLLL.L.LLLLLLL.L.L.LL.LLLLL..LLLLLLL.LLLLLLLL.LL.LL
LLLLLL.LLLL.LLLL.LLLLLLLLLLLLL..LLLLLLLLLLLLLL.LLLL.LL.LLLLLL.LLLLL.LLL.LLLL.LLLLLLLL.LLLLL
LLLL.L.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLL.L..LLLLL.LLLLLLLL.L.LLLLLL.LLLLL
LLLLLL..LLL.LLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLL.L.LLLL.LLLLLL.LL.LLLLL.LLLLLLLLLLLLLLLLL.LLLLL
LLLLLL.LLLLLLLLLLLL.LLLL.LLLLLL.LLLLL.LL.LLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLLL..LLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLL.LLLLL
.L.L......LL..LL.....L..L...L...L.L...L.LL.....LL..L..L....L...L....L......L.L.L...L.LLLL..
LLLLLLLLL.L.LLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLLLLLLLLLLLL.LLLLLLL.LLLL.L.LLLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLL..LLLLLLLL.LLLLLLLL.LLLLL
LL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LL.LLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLLL
.LLLLL.LLLL.LLLL..LLLLLL.L.LLLLLL.LLL.LLLLL.LL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLL
LLLLLL.LLLL.LL.LLLLLLLL.LLLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLL.LLL.LLLLL
LLLLL.LLLLLLL.LL.LLLLLL..LLLLLL..LLLL.LLLLLLLL.L.LLL.L.LLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLL
LLLLLLLLLLL.LLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLL.LLLLLLLLLLL.LLLLL
.LLLLL.LLLL.L.LL..LLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLL.L.LLLLLLLLLLLLLLLLLLLLLLL.LLLLL
LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLL.L.LLLL.LLLLL.LLLLLLLL..LLLLLLL.LLLLL
.L...LL.LLL....LL..LL...L..LLL.LL...L...L........L....LL..L...L.....LL......L...L..LL....L.
LL.LLL.LLLLLLLLL.LLLLLLL.LLLLLL.L.LLL.LLLLLLLL.LLLLLL..LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL
LLLLLLLLLLL.LLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLLLLLLLLLLL.LLLLL.LLLLLLL..LLLLLLL...LLLL
LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLLL..LLLLLLL.LLLLLLLL.LLLLL
LLL.LL.LLLLLLL.LLL.LLLLL.LLLLLLLLLLLL.LL.LLLLL..LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLL
LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLLLLLL.LLLLLLLLL.LLLLL
L..LLL.LLLL.LLLLLLLLLLLL.LL.LLLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL
LLLLLL.LLLL.LLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLL.LLL.LLLL.
LLLLLL.LLL..LLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLL.LLLLLLL.LLLLLL.LLLLL.LLLL.LLL.LLLLLLLL.LLLLL
..LL...LLL....L....L.LLL.L.L...L.LLL..L...L..L..L........L.LL.L.L.LLLLLL...L.L.LL.L....LL..
LLLLLL.LLLLLLLLL.LL.L.LL.LLLLLL.L.LLL..LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLL..LL.LLLLLLLL.LLLL.LLLLLL.LLLLL.L.LLL.LL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.L..LL
LLLLLL.LLLLLLLLL.LL.LLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLL.L.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLL.LLLLL
LLLLLL.LLLL.LLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLLLLL.LL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLL.LL...LLLL.LLLLLLL.L.LLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLLLL
L.L.LL.....L.......L.....LLL..L......LL.LLL....L.L.L..........L.LL..LLL..L....LL..L..L..L.L
LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLL.LLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLLLLLLLL..LLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LL.LL.LLLLLLLLLLLLLLLLL.LL.LL
LLLLLLLLLLL.LLLL.L.LLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LL.LLLLLLLLLL..LLLLLLLL.LLLLL
LLLLLLLLLLLLLLLL.LL.LLLLLLLLLLL..LLLL.LLLLLL.L.LLLLLLL.LLLLL..LLLLLLLLLL.LLL.LLLL.LLLLLLLLL
LLLLLL.LLLL.LLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLL.L
LLLLL.LLLLLLLLLLLLLLL.LL.LLLL.L.LLLLL.LLLLLLLLLLLLLLLL.LLLLLL.L.LLLLLLLLLLLL.LLLLLLLL.LLLLL
..LLLL.L...LLL.L......L....L.L..L......LL..L.LL.LL.LL................LL.LL....L.........L..
LLLLLL.LLLL.LLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLL.LLLLLLLLL..LLLL
LLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL..LLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.L.LL.LLL.LLL.L
LLLLLL.LLLL.LLLL.LLLLLLL.LLLLLL.LLLLLLL.LLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLL
LLLLLL.LLLL.LL.L.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.L.L.L.LLLLLLLL.LL.LLLLLLLL.LL
LLL.LL.LLLL.LLLL.LLLLLLL.LLLLL..LLLLL.LLLL.LLL.LLLLLLLLLLLLL..LLLLLLLLLLLLLLLLLLLLLLL.LLLLL
LLLLLLLLLLL.LLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLL..LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
L.LLLL.LLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLL.L..LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLL
..L..L.L.LLLL.....LLL.LLL....L..LL.L..L......L.............LL....LL..L.L..L...L.....L.L...L
LLLLLL.LLLL.L.LL.L.LLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLL.LLLLLLLLLLLL
LLLLLL..LLL.LLLL.LLLLLLL.LLLLLL.LLLLL.LLLL.LLL.LLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL
LLLLLL.LLLL..LLLLLLLLLLL.LLLLLL.L.LLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LL.LLL.L.LLLLLLLL.LLLLL
LLLLLL.LLLLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLL.L.LLL.LLLLLLLLLLLLLLL.LLLLLLLL.LLLLL
LLLLLL.LL.L.LL.L.LLLLLLLLL.LLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL
LLLLLLLLL.L.LLLLLLLLLL.L.LLLLL.LLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLL.LLLLLLLLLLLL
LLLLLL.LLLLLL.LL.LLLLLLLL.LLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLL.
LLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLL.LLLLLLLL.LLLL
....L..L..LL....L.L.LLL.L.L...L...L.L.L..L..L...LLL.......L.......LL..L...LL..L.L....LL.LL.
LLLLLL.LLLL.LL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLL.LLLLL
LLLLLL.LLLL.LLLL..LLLLLL.LLLL.L.LLLLL.LLL.LLLL.LLLLLLL.LLLLLLLLLLLL.L.LLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.L.LLLLLL..LLLL
LLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLL.L.LLLL.LLLLL.LLLLL.LL.LLLLLLLL.LLLLL
.LLLLL.LLLL.LLLLLLLLLLLL.L.LLLLLLLLLL.LLLLLLLL.LLLLLLLLLLL.LL.L.LLLLLLLLLLLL.LLLLL.LL.LLLLL
LL.LLL.LLLL.LLLL.LLLLLLLLLLLLLL.LLLLL.LLLL.LLL.L.L.LLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLL.L.LLLLL
LLLLLLLLLLLLLL.L.LLLLLLL.LLLLLL.LLLLL.LLL.LLLLLLLLLLLL.L.LLLL.LLLLL.LLLLLLLLLLLLLLLLL.LLLL.
LLLLLL.L.LL.LLLLLLLLLLLL.LL.LLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.L.LLL.LLLLLLLL.LLLLLLL.LLLLLL
LLLLLLLLLLL.LLLLLLLLLLLL.L.L.LL.LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLL.LLL.LLLLL.LLLLLLLLLLL.LLLLL
.L...LL..LL....L...L.....LLLL..L....L..LL...L...LLL...L.LLL..L.LL.L.L.LL.L..LL.....LLLL.LL.
LLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLL..LLLLLLLL.LLLLLLLLLLLLLL
LLLLLLLLLLL.L.LLLLLL.LLLLLLLLLL.L.LLL.LLLL.LLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLLL.LLLLLLL.LLLLL
LLLLLL.LLLL.LLLL.LLLLLLL.LL.LLLLLLLLL.LLLLLLL..LLLLLLL.LLL.LL.LLLLL.LLLLL.LL.LLLLLL.L.LLLLL
LLLLLL.LLLL.LLLL.LLLLLLLLLLL.LL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLLLLLLL.LLLLLLLLLLLL.LLLLLL.LLLLL..LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL..LLLLLLL.LLLLL
LLL.LLLLLLL.LLLL.LLLLLLL.LLLLLLLLL.LL.LLLLLLLL.LLLLLLL..LLLLL.LLLLLLLLLLL.LL.LLLLLLLLLLLLL.
LLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLL.LLL.LLLLLLLLLLLLL
...L...L.L......L..L..L.LL...L..........L..L.L.L........L..L..L.L.LL..LL..LL.........L.....
LLLL.L.LLLL.LLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLL
LLLLLLLLLLL.LLLLL.LLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLL.LLLLLLLLLLLLLLLLLL
LLLLLL.LLLLLLLLLLLLLLLLL.LLLLLL.L.LLL.LL.LLLLLLLLLLLL..LLLLL..LLLLL.LLLLLLL..LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLL.LLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLL.LLLLL
.LLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLL.LLLLL
L.LL.LLLLLL.LL.L..LLLLLL.LLLLLL.LLLLLLLLLLLLLL.LL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLL.LLL.LLLLL
L..L...L..L...LL...L.L.L..LLL.LL...L......LLL...L.L....L.......L...L.LL.L....L....L...L....
LLLLLL.LLLL.LLLL.L.LLLLL.LLL.LL.LLLLL.LLLL.LLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLL
LLL.LL.LLLL.LLL...L.LLL..LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLL.LLLLL
LLLLLL.LLLL.LLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLLL
LLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLL.LLL.LLLLLLLLLLLLL.LL.LLLLL.LLLLLLL.LLLLL
LLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLLLLLLL.LLLL.LLLLLLL.LLLLLL..LLLL.LLLLLLLL.LLLLLLLLL.LLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLL.
LLLLLL.LLLLLLLLLLLLLL.LL.L.LLLLLLLLLL.LLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLL.L
LLLLLL.LLLLLLLLL.LLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLLLLLLLLLLLLL.LLLLL.LLLLLL.L.LLLLLLLLLLLLLL
LLLLLL.LLLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLLLL.LLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLL";

#[derive(Debug, Copy, Clone, PartialEq)]
enum Seat {
    Empty(),
    Occupied(),
    Floor(),
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl Direction {
    const fn value(&self) -> (i32, i32) {
        match *self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::NorthEast => (1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
            Direction::NorthWest => (-1, -1)
        }
    }

    fn all() -> Vec<Direction> {
        vec![Direction::North,
             Direction::South,
             Direction::East,
             Direction::West,
             Direction::NorthEast,
             Direction::SouthEast,
             Direction::SouthWest,
             Direction::NorthWest]
    }
}

fn parse(input: &str) -> Vec<Vec<Seat>> {
    input
        .split("\n")
        .map(|ln| ln.chars()
            .map(|space| match space {
                'L' => Seat::Empty(),
                '#' => Seat::Occupied(),
                '.' => Seat::Floor(),
                _ => Seat::Floor()
            }).collect::<Vec<Seat>>()
        ).collect::<Vec<Vec<Seat>>>()
}

fn print_grid(grid: &Vec<Vec<Seat>>) {
    println!("--------------");

    grid.iter().for_each(|ln| {
        ln
            .iter()
            .for_each(|c| print!("{}", match c {
                Seat::Empty() => 'L',
                Seat::Occupied() => '#',
                Seat::Floor() => '.',
            }));

        print!("\n");
    });
}

fn print_grid_dbg(grid: &Vec<Vec<Seat>>, x: i32, y: i32) {
    println!("--------------");

    let seats = occupied_neighbors_dbg(grid, x, y);

    grid
        .iter()
        .enumerate()
        .for_each(|(y, ln)| {
            ln
                .iter()
                .enumerate()
                .for_each(|(x, s)| {
                    if seats.contains(&(x as i32, y as i32)) {
                        print!("O");
                    } else {
                        print!("{}", match *s {
                            Seat::Empty() => 'L',
                            Seat::Occupied() => '#',
                            Seat::Floor() => '.',
                        });
                    }
                });

            print!("\n");
        });
}

fn safe_seat_check(grid: &Vec<Vec<Seat>>, x: i32, y: i32) -> Option<Seat> {
    if y < 0 || grid.len() <= y as usize {
        return None;
    }

    if x < 0 || grid[y as usize].len() <= x as usize {
        return None;
    }

    let row = grid.get(y as usize).unwrap();
    let seat = row.get(x as usize).unwrap();

    return Some(*seat);
}

fn first_seat(grid: &Vec<Vec<Seat>>, x: i32, y: i32, direction: Direction) -> Option<(Seat, i32, i32)> {
    let mut l_x = x + direction.value().0;
    let mut l_y = y + direction.value().1;

    loop {
        let seat = safe_seat_check(grid, l_x, l_y);

        match seat {
            // Loop again if floor
            Some(Seat::Floor()) => {}

            // Else return
            None => return None,
            _ => return Some((seat.unwrap(), l_x, l_y))
        }

        l_x += direction.value().0;
        l_y += direction.value().1;
    }
}

fn occupied_neighbors(grid: &Vec<Vec<Seat>>, x: i32, y: i32) -> i32 {
    Direction::all().iter().map(|d| {
        let seat = safe_seat_check(grid, x + d.value().0, y + d.value().1);

        match seat {
            Some(Seat::Occupied()) => 1,
            _ => 0
        }
    }).sum()
}

fn occupied_neighbors_b(grid: &Vec<Vec<Seat>>, x: i32, y: i32) -> i32 {
    Direction::all()
        .iter()
        .map(|d| {
            let seat = first_seat(grid, x, y, *d);

            match seat {
                Some((Seat::Occupied(), _, _)) => 1,
                _ => 0
            }
        }).sum()
}

fn occupied_neighbors_dbg(grid: &Vec<Vec<Seat>>, x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut seats = Vec::new();

    Direction::all()
        .iter()
        .for_each(|d| {
            let seat = first_seat(grid, x, y, *d);

            match seat {
                Some((Seat::Occupied(), s_x, s_y)) => {
                    seats.push((s_x, s_y));
                },
                _ => {}
            };
        });

    seats
}

fn step(grid: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    grid
        .iter()
        .enumerate()
        .map(|(y, ln)| {
            ln
                .iter()
                .enumerate()
                .map(|(x, s)| {
                    let neighbors = occupied_neighbors(grid, x as i32, y as i32);

                    return if *s == Seat::Empty() && neighbors == 0 {
                        Seat::Occupied()
                    } else if *s == Seat::Occupied() && neighbors >= 4 {
                        Seat::Empty()
                    } else {
                        *s
                    };
                })
                .collect()
        })
        .collect()
}

fn step_b(grid: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    grid
        .iter()
        .enumerate()
        .map(|(y, ln)| {
            ln
                .iter()
                .enumerate()
                .map(|(x, s)| {
                    let neighbors = occupied_neighbors_b(grid, x as i32, y as i32);

                    return if *s == Seat::Empty() && neighbors == 0 {
                        Seat::Occupied()
                    } else if *s == Seat::Occupied() && neighbors >= 5 {
                        Seat::Empty()
                    } else {
                        *s
                    };
                })
                .collect()
        })
        .collect()
}

fn grids_eq(a: &Vec<Vec<Seat>>, b: &Vec<Vec<Seat>>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for y in 0..a.len() {
        if a[y].len() != b[y].len() {
            return false;
        }

        for x in 0..a[y].len() {
            if a[y][x] != b[y][x] {
                return false;
            }
        }
    }

    true
}

fn day_11_a(input: &str) -> i32 {
    let mut last_grid = parse(input);
    print_grid(&last_grid);

    loop {
        let new_grid = step(&last_grid);
        print_grid(&new_grid);

        if grids_eq(&last_grid, &new_grid) {
            break;
        }

        last_grid = new_grid
    }

    last_grid
        .iter()
        .map::<i32, _>(|ln| {
            ln
                .iter()
                .map::<i32, _>(|s| match *s {
                    Seat::Occupied() => 1,
                    _ => 0
                })
                .sum()
        })
        .sum()
}

fn day_11_b(input: &str) -> i32 {
    let mut last_grid = parse(input);
    // print_grid(&last_grid);

    loop {
        let new_grid = step_b(&last_grid);
        // print_grid(&new_grid);
        // print_grid_seats(&new_grid);

        if grids_eq(&last_grid, &new_grid) {
            break;
        }

        last_grid = new_grid
    }

    last_grid
        .iter()
        .map::<i32, _>(|ln| {
            ln
                .iter()
                .map::<i32, _>(|s| match *s {
                    Seat::Occupied() => 1,
                    _ => 0
                })
                .sum()
        })
        .sum()
}

#[test]
fn test_day_11_a() {
    println!("{}", day_11_a(TEST_DATA));
}

#[test]
fn test_day_11_b() {
    println!("{}", day_11_b(TEST_DATA));
}

#[test]
fn test_test() {
    let last_grid = step_b(&parse(TEST_DATA));

    print_grid(&last_grid);

    print_grid_dbg(&last_grid, 1, 1);

    println!("{:?}", occupied_neighbors_b(&last_grid, 1, 1));
}