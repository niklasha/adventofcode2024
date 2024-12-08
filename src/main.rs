use std::collections::{HashMap, HashSet};
use std::io;
use std::io::Read;
use std::ops::ControlFlow;

fn _main_01_1() {
    let input = io::stdin();
    let lines = input.lines().map(Result::unwrap);
    let numbers = lines.flat_map(|line| {
        line.split_whitespace()
            .map(|token| token.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });
    let (a, b): (Vec<_>, Vec<_>) = numbers.enumerate().partition(|(i, _)| i % 2 == 0);
    let mut a: Vec<_> = a.into_iter().map(|(_, n)| n).collect();
    a.sort();
    let mut b: Vec<_> = b.into_iter().map(|(_, n)| n).collect();
    b.sort();
    let distance = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| i64::abs(a - b))
        .sum::<i64>();
    println!("{}", distance);
}

fn _main_01_2() {
    let input = io::stdin();
    let lines = input.lines().map(Result::unwrap);
    let numbers = lines.flat_map(|line| {
        line.split_whitespace()
            .map(|token| token.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });
    let (a, b): (Vec<_>, Vec<_>) = numbers.enumerate().partition(|(i, _)| i % 2 == 0);
    let a = a
        .into_iter()
        .map(|(_, n)| n)
        .fold(HashMap::new(), |mut map, n| {
            map.entry(n).and_modify(|c| *c += 1).or_insert(1i64);
            map
        });
    let b = b
        .into_iter()
        .map(|(_, n)| n)
        .fold(HashMap::new(), |mut map, n| {
            map.entry(n).and_modify(|c| *c += 1).or_insert(1i64);
            map
        });
    let score = a
        .keys()
        .into_iter()
        .map(|n| n * a.get(n).unwrap() * b.get(n).unwrap_or(&0))
        .sum::<i64>();
    println!("{}", score);
}

fn _main_02_1() {
    let input = io::stdin();
    let lines = input.lines().map(Result::unwrap);
    let safe = lines
        .filter(|line| {
            let (diffs, _) = line
                .split_whitespace()
                .map(|token| token.parse::<i64>().unwrap())
                .fold((Vec::new(), None), |(mut v, last), n| {
                    if let Some(last) = last {
                        v.push(n - last);
                    }
                    (v, Some(n))
                });
            diffs.iter().all(|&d| d != 0 && d.abs() <= 3) && {
                let mut i = diffs.iter();
                let sgn = i.next().unwrap().signum();
                i.all(|n| sgn == n.signum())
            }
        })
        .count();
    println!("{}", safe);
}

fn _main_02_2() {
    let input = io::stdin();
    let lines = input.lines().map(Result::unwrap);
    let safe = lines
        .filter(|line| {
            let map = line
                .split_whitespace()
                .map(|token| token.parse::<i64>().unwrap());
            let v = map.collect::<Vec<_>>();
            (0..v.len()).any(|i| {
                let mut v = v.clone();
                v.remove(i);
                let (diffs, _) = v.iter().fold((Vec::new(), None), |(mut v, last), n| {
                    if let Some(last) = last {
                        v.push(n - last);
                    }
                    (v, Some(n))
                });
                diffs.iter().all(|&d| d != 0 && d.abs() <= 3) && {
                    let mut i = diffs.iter();
                    let sgn = i.next().unwrap().signum();
                    i.all(|n| sgn == n.signum())
                }
            })
        })
        .count();
    println!("{}", safe);
}

fn _main_03_1() {
    let input = io::stdin();
    let s = String::from_iter(input.bytes().map(|b| b.unwrap() as char));
    let (sum, _) = (0..)
        .try_fold((0, &s[..]), |(sum, s), _| {
            let mut i = s.splitn(2, "mul(");
            if i.next().is_none() {
                ControlFlow::Break((sum, ""))
            } else if let Some(s) = i.next() {
                let mut i = s.splitn(2, ',');
                if let Some(a) = i.next() {
                    if a.len() < 1 || a.len() > 3 {
                        ControlFlow::Continue((sum, s))
                    } else if let Ok(a) = a.parse::<i64>() {
                        if let Some(s) = i.next() {
                            let mut i = s.splitn(2, ')');
                            if let Some(b) = i.next() {
                                if b.len() < 1 || b.len() > 3 {
                                    ControlFlow::Continue((sum, s))
                                } else if let Ok(b) = b.parse::<i64>() {
                                    ControlFlow::Continue((sum + a * b, i.next().unwrap()))
                                } else {
                                    ControlFlow::Continue((sum, s))
                                }
                            } else {
                                ControlFlow::Break((sum, ""))
                            }
                        } else {
                            ControlFlow::Continue((sum, s))
                        }
                    } else {
                        ControlFlow::Continue((sum, s))
                    }
                } else {
                    ControlFlow::Break((sum, ""))
                }
            } else {
                ControlFlow::Break((sum, ""))
            }
        })
        .break_value()
        .unwrap();
    println!("{}", sum);
}

fn _main_03_2() {
    let input = io::stdin();
    let s = String::from_iter(input.bytes().map(|b| b.unwrap() as char));
    let (sum, _, _) = (0..)
        .try_fold((0, &s[..], false), |(sum, s, dont), _| {
            let mut i = s.splitn(2, "mul(");
            if let Some(s) = i.next() {
                let dont = s
                    .rfind("don't()")
                    .map_or_else(|| dont && !s.contains("do()"), |p| !s[p..].contains("do()"));
                if let Some(s) = i.next() {
                    if dont {
                        ControlFlow::Continue((sum, s, dont))
                    } else {
                        let mut i = s.splitn(2, ',');
                        if let Some(a) = i.next() {
                            if a.len() < 1 || a.len() > 3 {
                                ControlFlow::Continue((sum, s, dont))
                            } else if let Ok(a) = a.parse::<i64>() {
                                if let Some(s) = i.next() {
                                    let mut i = s.splitn(2, ')');
                                    if let Some(b) = i.next() {
                                        if b.len() < 1 || b.len() > 3 {
                                            ControlFlow::Continue((sum, s, dont))
                                        } else if let Ok(b) = b.parse::<i64>() {
                                            ControlFlow::Continue((
                                                sum + a * b,
                                                i.next().unwrap(),
                                                dont,
                                            ))
                                        } else {
                                            ControlFlow::Continue((sum, s, dont))
                                        }
                                    } else {
                                        ControlFlow::Break((sum, "", dont))
                                    }
                                } else {
                                    ControlFlow::Continue((sum, s, dont))
                                }
                            } else {
                                ControlFlow::Continue((sum, s, dont))
                            }
                        } else {
                            ControlFlow::Break((sum, "", dont))
                        }
                    }
                } else {
                    ControlFlow::Break((sum, "", dont))
                }
            } else {
                ControlFlow::Break((sum, "", dont))
            }
        })
        .break_value()
        .unwrap();
    println!("{}", sum);
}

fn _main_04_1() {
    const XMAS: &str = "XMAS";
    const XMAS_LEN: usize = XMAS.len();
    let input = io::stdin();
    let grid = String::from_iter(input.bytes().map(|b| b.unwrap() as char));
    let w = grid.find('\n').unwrap();
    let d = w + 1;
    let h = grid.len() / d;
    let count_in_row = |r: &str| {
        let (_, cnt) = (0..)
            .try_fold((r, 0), |(s, cnt), _| {
                if let Some(p) = s.find(XMAS) {
                    ControlFlow::Continue((&s[p + XMAS_LEN..], cnt + 1))
                } else {
                    ControlFlow::Break(("", cnt))
                }
            })
            .break_value()
            .unwrap();
        cnt
    };
    let reverse = |s: &str| s.chars().rev().collect::<String>();
    let count_in_grid_forward = |g: &str| g.split('\n').map(|r| count_in_row(r)).sum::<usize>();
    let count_in_grid_backward = |g: &str| {
        g.split('\n')
            .map(|r| count_in_row(&reverse(r)))
            .sum::<usize>()
    };
    let transpose = |g: &str| {
        (0..h)
            .fold(vec![String::new(); w], |mut v, r| {
                v.iter_mut()
                    .enumerate()
                    .for_each(|(c, s)| s.push(g.as_bytes()[r * d + (w - c - 1)] as char));
                v
            })
            .join("\n")
    };
    let tilt = |g: &str| {
        (0..(h + w - 1))
            .fold(vec![String::new(); h + w - 1], |mut v, i| {
                let (mut r, mut c) = if i < h {
                    (i as isize, 0)
                } else {
                    ((h - 1) as isize, i - h + 1)
                };
                while r >= 0 && c < w {
                    v[i].push(g.as_bytes()[(r as usize * d + c) as usize] as char);
                    (r, c) = (r - 1, c + 1);
                }
                v
            })
            .join("\n")
    };
    let we = count_in_grid_forward(&grid);
    let ew = count_in_grid_backward(&grid);
    let transposed_grid = transpose(&grid);
    let ns = count_in_grid_forward(&transposed_grid);
    let sn = count_in_grid_backward(&transposed_grid);
    let tilted_grid = tilt(&grid);
    let swne = count_in_grid_forward(&tilted_grid);
    let nesw = count_in_grid_backward(&tilted_grid);
    let tilted_transposed_grid = tilt(&transposed_grid);
    let nwse = count_in_grid_forward(&tilted_transposed_grid);
    let senw = count_in_grid_backward(&tilted_transposed_grid);
    println!("{}", we + ew + ns + sn + swne + nesw + nwse + senw);
}

fn _main_04_2() {
    let input = io::stdin();
    let grid = String::from_iter(input.bytes().map(|b| b.unwrap() as char));
    let w = grid.find('\n').unwrap();
    let d = w + 1;
    let cnt = grid
        .chars()
        .enumerate()
        .filter(|(i, c)| {
            *c == 'A' && *i >= d && *i < grid.len() - d && *i % d > 0 && *i % d < w - 1 && {
                let nw = grid.as_bytes()[*i - d - 1] as char;
                let ne = grid.as_bytes()[*i - d + 1] as char;
                let sw = grid.as_bytes()[*i + d - 1] as char;
                let se = grid.as_bytes()[*i + d + 1] as char;
                ((nw == 'M' && se == 'S') || (nw == 'S' && se == 'M'))
                    && ((ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M'))
            }
        })
        .count();
    println!("{}", cnt);
}

fn _main_05_1() {
    let input = io::stdin();
    let mut lines = input.lines().map(Result::unwrap);
    let (before, after) = lines
        .try_fold(
            (HashMap::new(), HashMap::new()),
            |(mut before, mut after), l| {
                if l.is_empty() {
                    ControlFlow::Break((before, after))
                } else {
                    let [a, b] = l
                        .splitn(2, '|')
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()[..]
                    else {
                        panic!("");
                    };
                    before
                        .entry(a)
                        .and_modify(|set: &mut HashSet<_>| {
                            (*set).insert(b);
                        })
                        .or_insert(HashSet::from([b]));
                    after
                        .entry(b)
                        .and_modify(|set: &mut HashSet<_>| {
                            (*set).insert(a);
                        })
                        .or_insert(HashSet::from([a]));
                    ControlFlow::Continue((before, after))
                }
            },
        )
        .break_value()
        .unwrap();
    let sum = lines
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .filter(|v| {
            (0..v.len()).all(|i| {
                (0..i).all(|j| after.get(&v[i]).unwrap().contains(&v[j]))
                    && ((i + 1)..v.len())
                        .all(|j| before.get(&v[i]).map_or(true, |set| set.contains(&v[j])))
            })
        })
        .map(|v| v[v.len() / 2])
        .sum::<u64>();
    println!("{}", sum);
}

fn _main_05_2() {
    let input = io::stdin();
    let mut lines = input.lines().map(Result::unwrap);
    let (before, after) = lines
        .try_fold(
            (HashMap::new(), HashMap::new()),
            |(mut before, mut after), l| {
                if l.is_empty() {
                    ControlFlow::Break((before, after))
                } else {
                    let [a, b] = l
                        .splitn(2, '|')
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()[..]
                    else {
                        panic!("");
                    };
                    before
                        .entry(a)
                        .and_modify(|set: &mut HashSet<_>| {
                            (*set).insert(b);
                        })
                        .or_insert(HashSet::from([b]));
                    after
                        .entry(b)
                        .and_modify(|set: &mut HashSet<_>| {
                            (*set).insert(a);
                        })
                        .or_insert(HashSet::from([a]));
                    ControlFlow::Continue((before, after))
                }
            },
        )
        .break_value()
        .unwrap();
    let mut incorrect = lines
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .filter(|v| {
            !(0..v.len()).all(|i| {
                (0..i).all(|j| after.get(&v[i]).unwrap().contains(&v[j]))
                    && ((i + 1)..v.len())
                        .all(|j| before.get(&v[i]).map_or(true, |set| set.contains(&v[j])))
            })
        })
        .collect::<Vec<_>>();
    let mut corrected = vec![];
    for v in &mut incorrect {
        let mut nv = vec![];
        let mut after = after.clone();
        while !v.is_empty() {
            let (i, n) = v
                .iter()
                .enumerate()
                .find(|(_, n)| {
                    after
                        .get(n)
                        .map_or(true, |set| set.iter().all(|n| !v.contains(n)))
                })
                .unwrap();
            nv.push(n.clone());
            after.remove(n);
            for (_, set) in after.iter_mut() {
                set.remove(n);
            }
            after.retain(|_, set| !set.is_empty());
            v.remove(i);
        }
        corrected.push(nv);
    }
    let sum = corrected.into_iter().map(|v| v[v.len() / 2]).sum::<u64>();
    println!("{}", sum);
}

fn _main_06_1() {
    let mut input = io::stdin().bytes().map(Result::unwrap);
    let (_, my, mx, grid, Some((mut x, mut y)), Some((mut dx, mut dy))) = (0..)
        .try_fold(
            (0, 0, 0, HashSet::new(), None, None),
            |(x, y, mx, mut grid, origin, dir), _| {
                let b = input.next();
                match b {
                    None => ControlFlow::Break((x, y, mx, grid, origin, dir)),
                    Some(b'\n') => ControlFlow::Continue((0, y + 1, x, grid, origin, dir)),
                    Some(b'#') => {
                        grid.insert((x, y));
                        ControlFlow::Continue((x + 1, y, mx, grid, origin, dir))
                    }
                    Some(b'^') | Some(b'>') | Some(b'v') | Some(b'<') => ControlFlow::Continue((
                        x + 1,
                        y,
                        mx,
                        grid,
                        Some((x, y)),
                        Some(match b {
                            Some(b'^') => (0, -1),
                            Some(b'>') => (1, 0),
                            Some(b'v') => (0, 1),
                            Some(b'<') => (-1, 0),
                            _ => panic!(),
                        }),
                    )),
                    Some(b'.') => ControlFlow::Continue((x + 1, y, mx, grid, origin, dir)),
                    _ => panic!(),
                }
            },
        )
        .break_value()
        .unwrap()
    else {
        panic!()
    };
    let set = (0..)
        .try_fold(HashSet::new(), |mut set, _| {
            set.insert((x, y));
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || nx >= mx || ny < 0 || ny >= my {
                ControlFlow::Break(set)
            } else {
                if grid.contains(&(nx, ny)) {
                    (dx, dy) = (-dy, dx);
                } else {
                    (x, y) = (nx, ny);
                }
                ControlFlow::Continue(set)
            }
        })
        .break_value()
        .unwrap();
    println!("{}", set.len());
}

fn _main_06_2() {
    let mut input = io::stdin().bytes().map(Result::unwrap);
    let (_, my, mx, grid, Some((x, y)), Some((dx, dy))) = (0..)
        .try_fold(
            (0, 0, 0, HashSet::new(), None, None),
            |(x, y, mx, mut grid, origin, dir), _| {
                let b = input.next();
                match b {
                    None => ControlFlow::Break((x, y, mx, grid, origin, dir)),
                    Some(b'\n') => ControlFlow::Continue((0, y + 1, x, grid, origin, dir)),
                    Some(b'#') => {
                        grid.insert((x, y));
                        ControlFlow::Continue((x + 1, y, mx, grid, origin, dir))
                    }
                    Some(b'^') | Some(b'>') | Some(b'v') | Some(b'<') => ControlFlow::Continue((
                        x + 1,
                        y,
                        mx,
                        grid,
                        Some((x, y)),
                        Some(match b {
                            Some(b'^') => (0, -1),
                            Some(b'>') => (1, 0),
                            Some(b'v') => (0, 1),
                            Some(b'<') => (-1, 0),
                            _ => panic!(),
                        }),
                    )),
                    Some(b'.') => ControlFlow::Continue((x + 1, y, mx, grid, origin, dir)),
                    _ => panic!(),
                }
            },
        )
        .break_value()
        .unwrap()
    else {
        panic!()
    };
    let obstacles = (0..mx).fold(HashSet::new(), |set, sx| {
        (0..my).fold(set, |mut set, sy| {
            if (sx, sy) == (x, y) || grid.contains(&(sx, sy)) {
                set
            } else {
                set.insert((sx, sy));
                set
            }
        })
    });
    let cnt = obstacles.into_iter().fold(0, |cnt, (ox, oy)| {
        let (mut x, mut y, mut dx, mut dy) = (x, y, dx, dy);
        let (_, is_loop) = (0..)
            .try_fold((HashSet::new(), false), |(mut set, _), _| {
                if set.contains(&(x, y, dx, dy)) {
                    ControlFlow::Break((set, true))
                } else {
                    set.insert((x, y, dx, dy));
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0 || nx >= mx || ny < 0 || ny >= my {
                        ControlFlow::Break((set, false))
                    } else {
                        if grid.contains(&(nx, ny)) || (nx, ny) == (ox, oy) {
                            (dx, dy) = (-dy, dx);
                        } else {
                            (x, y) = (nx, ny);
                        }
                        ControlFlow::Continue((set, false))
                    }
                }
            })
            .break_value()
            .unwrap();
        if is_loop {
            cnt + 1
        } else {
            cnt
        }
    });
    println!("{}", cnt);
}

fn _main_07_1() {
    let input = io::stdin();
    let lines = input.lines().map(Result::unwrap);
    let eqs = lines
        .map(|l| {
            let mut tokens = l.split_whitespace();
            let result = tokens
                .next()
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let operands = tokens
                .map(|t| t.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (result, operands)
        })
        .collect::<Vec<_>>();
    let sum = eqs
        .into_iter()
        .filter(|(res, opers)| {
            let n = opers.len() - 1;
            let mut it = opers.iter();
            let acc = *it.next().unwrap();
            (0..(1 << n)).any(|ops| {
                *res == (0..n).zip(it.clone()).fold(acc, |acc, (i, &oper)| {
                    if (ops >> i) & 1 == 0 {
                        acc + oper
                    } else {
                        acc * oper
                    }
                })
            })
        })
        .map(|(res, _)| res)
        .sum::<i64>();
    println!("{}", sum);
}

fn _main_07_2() {
    let input = io::stdin();
    let lines = input.lines().map(Result::unwrap);
    let eqs = lines
        .map(|l| {
            let mut tokens = l.split_whitespace();
            let result = tokens
                .next()
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let operands = tokens
                .map(|t| t.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (result, operands)
        })
        .collect::<Vec<_>>();
    enum Op {
        Add,
        Mul,
        Concat,
    }
    let to_op_vec = |mut ops: u32, n: usize| {
        let mut v = vec![];
        for _i in 0..n {
            v.push(match ops % 3 {
                0 => Op::Add,
                1 => Op::Mul,
                2 => Op::Concat,
                _ => panic!(),
            });
            ops /= 3;
        }
        v
    };
    let sum = eqs
        .into_iter()
        .filter(|(res, opers)| {
            let n = opers.len() - 1;
            let mut it = opers.iter();
            let acc = *it.next().unwrap();
            (0..3_u32.pow(n as u32)).any(|ops| {
                *res == to_op_vec(ops, n)
                    .iter()
                    .zip(it.clone())
                    .fold(acc, |acc, (op, &oper)| match op {
                        Op::Add => acc + oper,
                        Op::Mul => acc * oper,
                        Op::Concat => (acc.to_string() + &oper.to_string()).parse().unwrap(),
                    })
            })
        })
        .map(|(res, _)| res)
        .sum::<i64>();
    println!("{}", sum);
}

fn _main_08_1() {
    let mut input = io::stdin().bytes().map(Result::unwrap);
    let (_, my, mx, grid) = (0..)
        .try_fold(
            (0, 0, 0, HashMap::<u8, Vec<(i64, i64)>>::new()),
            |(x, y, mx, mut grid), _| {
                let b = input.next();
                match b {
                    None => ControlFlow::Break((x, y, mx, grid)),
                    Some(b'\n') => ControlFlow::Continue((0, y + 1, x, grid)),
                    Some(b'.') => ControlFlow::Continue((x + 1, y, mx, grid)),
                    Some(b) => {
                        grid.entry(b)
                            .and_modify(|e| e.push((x, y)))
                            .or_insert(vec![(x, y)]);
                        ControlFlow::Continue((x + 1, y, mx, grid))
                    }
                }
            },
        )
        .break_value()
        .unwrap();
    let antinodes = grid.iter().fold(HashSet::new(), |mut set, (_, v)| {
        v.iter()
            .flat_map(|(x0, y0)| v.iter().map(|(x1, y1)| ((*x0, *y0), (*x1, *y1))))
            .filter(|((x0, y0), (x1, y1))| x0 != x1 || y0 != y1)
            .for_each(|((x0, y0), (x1, y1))| {
                let (dx, dy) = (x0 - x1, y0 - y1);
                vec![(x0 + dx, y0 + dy), (x1 - dx, y1 - dy)]
                    .iter()
                    .filter(|(xa, ya)| *xa >= 0 && *xa < mx && *ya >= 0 && *ya < my)
                    .for_each(|loc| {
                        set.insert(*loc);
                    });
            });
        set
    });
    println!("{}", antinodes.len());
}

fn _main_08_2() {
    let mut input = io::stdin().bytes().map(Result::unwrap);
    let (_, my, mx, grid) = (0..)
        .try_fold(
            (0, 0, 0, HashMap::<u8, Vec<(i64, i64)>>::new()),
            |(x, y, mx, mut grid), _| {
                let b = input.next();
                match b {
                    None => ControlFlow::Break((x, y, mx, grid)),
                    Some(b'\n') => ControlFlow::Continue((0, y + 1, x, grid)),
                    Some(b'.') | Some(b'#') => ControlFlow::Continue((x + 1, y, mx, grid)),
                    Some(b) => {
                        grid.entry(b)
                            .and_modify(|e| e.push((x, y)))
                            .or_insert(vec![(x, y)]);
                        ControlFlow::Continue((x + 1, y, mx, grid))
                    }
                }
            },
        )
        .break_value()
        .unwrap();
    let antinodes = grid.iter().fold(HashSet::new(), |mut set, (_, v)| {
        v.iter()
            .flat_map(|(x0, y0)| v.iter().map(|(x1, y1)| ((*x0, *y0), (*x1, *y1))))
            .filter(|((x0, y0), (x1, y1))| x0 <= x1 && (x0 != x1 || y0 != y1))
            .for_each(|((x0, y0), (x1, y1))| {
                let (dx, dy) = (x0 - x1, y0 - y1);
                let f = |c: i64, d: i64, m: i64| {
                    let min = c % d;
                    let cnt = (m - min - 1) / d;
                    (cnt.abs() + 1, cnt.signum(), c / d)
                };
                let (cnt, sgn, off) = if dx != 0 {
                    f(x0, dx, mx)
                } else {
                    f(y0, dy, my)
                };
                for i in 0..cnt {
                    let (x, y) = (x0 + dx * ((off + i) * sgn), y0 + dy * ((off + i) * sgn));
                    if y >= 0 && y < my {
                        set.insert((x, y));
                    }
                }
            });
        set
    });
    println!("{}", antinodes.len());
}

fn main() {
    _main_08_2();
}
