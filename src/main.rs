use std::collections::{BTreeMap, HashMap, HashSet};
use std::io;
use std::io::{BufRead, Read};
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

fn _main_09_1() {
    let mut input = String::new();
    let _ = io::stdin().lock().read_line(&mut input);
    let input = input.strip_suffix('\n').unwrap();
    let (_, _, chunks, _) = input.chars().fold(
        (false, 0_isize, BTreeMap::new(), 0_isize),
        |(is_free, p, mut chunks, id), sz| {
            let sz = sz.to_digit(10).unwrap() as isize;
            chunks.insert(p, (if is_free { -1_isize } else { id }, sz));
            (!is_free, p + sz, chunks, if is_free { id } else { id + 1 })
        },
    );
    //    println!("{:?}", chunks);
    let mut rev_blocks = chunks
        .iter()
        .filter(|(_, (id, _))| *id >= 0)
        .flat_map(|(_, (id, sz))| vec![*id; *sz as usize].into_iter())
        .rev();
    let sum = (0..chunks
        .iter()
        .filter(|(_, (fd, _))| *fd >= 0)
        .map(|(_, (_, sz))| *sz)
        .sum::<isize>())
        .fold(0, |sum, i| {
            sum + i * {
                let (_, (id, _)) = chunks
                    .iter()
                    .take_while(|(p, _)| i >= **p as isize)
                    .last()
                    .unwrap();
                let n = if *id >= 0 {
                    *id as isize
                } else {
                    rev_blocks.next().unwrap()
                };
                n
            }
        });
    println!("{}", sum);
}

fn _main_09_2() {
    let mut input = String::new();
    let _ = io::stdin().lock().read_line(&mut input);
    let input = input.strip_suffix('\n').unwrap();
    let (_, _, mut chunks, _) = input.chars().fold(
        (false, 0_isize, BTreeMap::new(), 0_isize),
        |(is_free, p, mut chunks, mut id), sz| {
            let sz = sz.to_digit(10).unwrap() as isize;
            chunks.insert(
                p,
                (
                    if is_free {
                        -1_isize
                    } else {
                        id += 1;
                        id - 1
                    },
                    sz,
                ),
            );
            (!is_free, p + sz, chunks, id)
        },
    );
    let mut rev_files = chunks
        .iter()
        .filter(|(_, (id, _))| *id >= 0)
        .map(|(p, (id, sz))| (p.clone(), (id.clone(), sz.clone())))
        .collect::<Vec<_>>();
    rev_files.reverse();
    let (sum, _) = (0..chunks.iter().map(|(_, (_, sz))| *sz).sum::<isize>()).fold(
        (0, vec![]),
        |(sum, mut f), i| {
            (
                sum + i * {
                    let (p, (id, sz)) = chunks
                        .iter()
                        .take_while(|(p, (_, sz))| i >= **p as isize || *sz == 0)
                        .last()
                        .unwrap();
                    let (p, id, sz) = (p.clone(), id.clone(), sz.clone());
                    let n = if id >= 0 {
                        id as isize
                    } else {
                        let o = i - p;
                        let residual = sz - o;
                        if f.len() > 0 {
                            f.pop().unwrap()
                        } else {
                            if let Some((off, (p, (id, sz)))) = rev_files
                                .iter()
                                .enumerate()
                                .find(|(_, (p, (_, sz)))| *p > i && *sz <= residual)
                            {
                                let (p, id, sz) = (p.clone(), id.clone(), sz.clone());
                                f = vec![id; sz as usize - 1];
                                let (_, sz) = chunks.remove(&p).unwrap();
                                rev_files.remove(off);
                                chunks.insert(p, (-1, sz));
                                id
                            } else {
                                0
                            }
                        }
                    };
                    n
                },
                f,
            )
        },
    );
    println!("{}", sum);
}

fn _main_10_1() {
    let mut g = HashMap::new();
    let (g, mr, mc) =
        io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .fold((&mut g, 0, 0), |(g, r, _), l| {
                let (g, mc) = l.chars().fold((g, 0), |(g, c), ch| {
                    g.insert((r, c), ch);
                    (g, c + 1)
                });
                (g, r + 1, mc)
            });
    let o = g
        .iter()
        .filter(|(_, ch)| **ch == '0')
        .map(|(p, _)| *p)
        .collect::<Vec<_>>();
    let next = |p: &(i32, i32)| {
        let h = (g.get(&p).unwrap().to_digit(10).unwrap() + 1)
            .to_string()
            .chars()
            .next()
            .unwrap();
        (-1_i32..=1)
            .flat_map(|r| (-1_i32..=1).map(move |c| (r, c)))
            .filter(|(r, c)| (*r).abs() != (*c).abs())
            .map(|(r, c)| (p.0 + r, p.1 + c))
            .filter(|(r, c)| {
                *r >= 0 && *r < mr && *c >= 0 && *c < mc && *g.get(&(*r, *c)).unwrap() == h
            })
            .collect::<HashSet<_>>()
    };
    let sum = o.iter().fold(0, |sum, o| {
        let mut ts = HashSet::new();
        ts.insert(vec![*o]);
        let ts = (0..9).fold(ts, |ts, _| {
            ts.iter()
                .flat_map(|t| {
                    let p1 = next(t.last().unwrap());
                    let ts = p1
                        .iter()
                        .map(|p| {
                            let mut nt = t.clone();
                            nt.push(*p);
                            nt
                        })
                        .collect::<HashSet<_>>();
                    ts
                })
                .collect::<HashSet<_>>()
        });
        let cnt = ts
            .iter()
            .map(|v| v.last().unwrap().clone())
            .collect::<HashSet<_>>()
            .len();
        sum + cnt
    });
    println!("{}", sum);
}

fn _main_10_2() {
    let mut g = HashMap::new();
    let (g, mr, mc) =
        io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .fold((&mut g, 0, 0), |(g, r, _mc), l| {
                let (g, mc) = l.chars().fold((g, 0), |(g, c), ch| {
                    g.insert((r, c), ch);
                    (g, c + 1)
                });
                (g, r + 1, mc)
            });
    let o = g
        .iter()
        .filter(|(_, ch)| **ch == '0')
        .map(|(p, _)| *p)
        .collect::<Vec<_>>();
    let next = |p: &(i32, i32)| {
        let h = (g.get(&p).unwrap().to_digit(10).unwrap() + 1)
            .to_string()
            .chars()
            .next()
            .unwrap();
        (-1_i32..=1)
            .flat_map(|r| (-1_i32..=1).map(move |c| (r, c)))
            .filter(|(r, c)| (*r).abs() != (*c).abs())
            .map(|(r, c)| (p.0 + r, p.1 + c))
            .filter(|(r, c)| {
                *r >= 0 && *r < mr && *c >= 0 && *c < mc && *g.get(&(*r, *c)).unwrap() == h
            })
            .collect::<HashSet<_>>()
    };
    let sum = o.iter().fold(0, |sum, o| {
        let mut ts = HashSet::new();
        ts.insert(vec![*o]);
        let ts = (0..9).fold(ts, |ts, _| {
            ts.iter()
                .flat_map(|t| {
                    let p1 = next(t.last().unwrap());
                    let ts = p1
                        .iter()
                        .map(|p| {
                            let mut nt = t.clone();
                            nt.push(*p);
                            nt
                        })
                        .collect::<HashSet<_>>();
                    ts
                })
                .collect::<HashSet<_>>()
        });
        let cnt = ts.len();
        sum + cnt
    });
    println!("{}", sum);
}

fn _main_11_1() {
    let mut numbers = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split(' ')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    for _ in 0../*25*/ 9 {
        numbers = numbers
            .iter()
            .flat_map(|n| {
                if *n == 0 {
                    vec![1]
                } else {
                    let s = (*n).to_string();
                    let l = s.len();
                    if l % 2 == 0 {
                        vec![s[..l / 2].parse().unwrap(), s[l / 2..].parse().unwrap()]
                    } else {
                        vec![*n * 2024]
                    }
                }
            })
            .collect::<Vec<_>>();
        println!("{}", numbers.len());
    }
}

fn _main_11_2() {
    let mut stones = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split(' ')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .map(|n| (n, 1u64))
        .collect::<HashMap<_, _>>();
    for _t in 0u64..75 {
        stones = stones
            .into_iter()
            .flat_map(|(n, cnt)| match n {
                0 => vec![(1, cnt)],
                n => {
                    let s = n.to_string();
                    let l = s.len();
                    if l % 2 == 0 {
                        vec![
                            (s[..l / 2].parse().unwrap(), cnt),
                            (s[l / 2..].parse().unwrap(), cnt),
                        ]
                    } else {
                        vec![(n * 2024, cnt)]
                    }
                }
            })
            .fold(HashMap::new(), |mut map, (n, cnt)| {
                map.entry(n).and_modify(|acc| *acc += cnt).or_insert(cnt);
                map
            });
    }
    println!("{}", stones.values().sum::<u64>());
}

fn _main_12_1() {
    let (mut g, mr, mc) = io::stdin().lock().lines().map(Result::unwrap).fold(
        (BTreeMap::new(), 0, 0),
        |(g, r, _), l| {
            let (g, mc) = l.chars().fold((g, 0), |(mut g, c), ch| {
                g.insert((r, c), ch);
                (g, c + 1)
            });
            (g, r + 1, mc)
        },
    );

    fn map_region(
        map: &mut BTreeMap<(i32, i32), char>,
        plant: char,
        rc: (i32, i32),
        set: &mut HashSet<(i32, i32)>,
    ) {
        set.insert(rc);
        for nrc in [
            (rc.0 - 1, rc.1),
            (rc.0, rc.1 - 1),
            (rc.0, rc.1 + 1),
            (rc.0 + 1, rc.1),
        ] {
            if !set.contains(&nrc) {
                if let Some(n) = map.get(&nrc) {
                    if *n == plant {
                        map.remove(&nrc);
                        map_region(map, plant, nrc, set);
                    }
                }
            }
        }
    }

    fn pop_region(map: &mut BTreeMap<(i32, i32), char>) -> HashSet<(i32, i32)> {
        let Some((rc, plant)) = map.pop_first() else {
            panic!();
        };
        let mut set = HashSet::new();
        // find neighbors not in set
        map_region(map, plant, rc, &mut set);
        set
    }

    fn scan(
        map: &BTreeMap<(i32, i32), usize>,
        mr: i32,
        mc: i32,
        regions: &Vec<HashSet<(i32, i32)>>,
    ) -> Vec<(usize, usize)> {
        let mut rv = vec![(0, 0); regions.len()];
        for r in 0..=mr {
            for c in 0..=mc {
                let rc = (r, c);
                let above_rc = (r - 1, c);
                let left_rc = (r, c - 1);
                let region = map.get(&rc);
                let above = map.get(&above_rc);
                let left = map.get(&left_rc);
                // is there another region above?
                if region != above {
                    // Add a fence to the lower region
                    if let Some(region) = region {
                        rv[*region].1 += 1;
                    }
                    // Add a fence to the upper region
                    if let Some(above) = above {
                        rv[*above].1 += 1;
                    }
                }
                if region != left {
                    // Add a fence to the right region
                    if let Some(region) = region {
                        rv[*region].1 += 1;
                    }
                    // Add a fence to the left region
                    if let Some(left) = left {
                        rv[*left].1 += 1;
                    }
                }
                if let Some(region) = region {
                    rv[*region].0 += 1;
                }
            }
        }
        rv
    }

    let mut regions = vec![];
    while !g.is_empty() {
        regions.push(pop_region(&mut g));
    }
    let map = regions
        .iter()
        .enumerate()
        .flat_map(|(i, region)| region.iter().map(move |rc| (*rc, i)))
        .collect::<BTreeMap<_, _>>();
    let regions = scan(&map, mr, mc, &regions);
    println!(
        "{}",
        regions
            .into_iter()
            .map(|(area, perimeter)| area * perimeter)
            .sum::<usize>()
    );
}

fn _main_12_2() {
    let (mut g, mr, mc) = io::stdin().lock().lines().map(Result::unwrap).fold(
        (BTreeMap::new(), 0, 0),
        |(g, r, _), l| {
            let (g, mc) = l.chars().fold((g, 0), |(mut g, c), ch| {
                g.insert((r, c), ch);
                (g, c + 1)
            });
            (g, r + 1, mc)
        },
    );

    fn map_region(
        map: &mut BTreeMap<(i32, i32), char>,
        plant: char,
        rc: (i32, i32),
        set: &mut HashSet<(i32, i32)>,
    ) {
        set.insert(rc);
        for nrc in [
            (rc.0 - 1, rc.1),
            (rc.0, rc.1 - 1),
            (rc.0, rc.1 + 1),
            (rc.0 + 1, rc.1),
        ] {
            if !set.contains(&nrc) {
                if let Some(n) = map.get(&nrc) {
                    if *n == plant {
                        map.remove(&nrc);
                        map_region(map, plant, nrc, set);
                    }
                }
            }
        }
    }

    fn pop_region(map: &mut BTreeMap<(i32, i32), char>) -> HashSet<(i32, i32)> {
        let Some((rc, plant)) = map.pop_first() else {
            panic!();
        };
        let mut set = HashSet::new();
        // find neighbors not in set
        map_region(map, plant, rc, &mut set);
        set
    }

    fn scan(
        map: &BTreeMap<(i32, i32), usize>,
        mr: i32,
        mc: i32,
        regions: &Vec<HashSet<(i32, i32)>>,
    ) -> Vec<(usize, usize)> {
        let mut rv = vec![(0, 0); regions.len()];
        let mut last_below = None;
        let mut last_above = None;
        let mut last_right = vec![None; mc as usize + 1];
        let mut last_left = vec![None; mc as usize + 1];
        for r in 0..=mr {
            for c in 0..=mc {
                let rc = (r, c);
                let above_rc = (r - 1, c);
                let left_rc = (r, c - 1);
                let region = map.get(&rc);
                let above = map.get(&above_rc);
                let left = map.get(&left_rc);
                // is there another region above?
                if region != above {
                    // If this side was not started last round add it.
                    if last_below != region || last_above == region {
                        if let Some(region) = region {
                            rv[*region].1 += 1;
//                            println!("Adding horizontal above {:?} for {}", rc, region);
                        }
                    }
                    if last_above != above || last_below == above {
                        if let Some(above) = above {
                            rv[*above].1 += 1;
//                            println!("Adding horizontal below {:?} for {}", above_rc, above);
                        }
                    }
                }
                if let Some(region) = region {
                    if last_below != Some(region) {
                        last_below = Some(region);
                    }
                } else {
                    last_below = None;
                }
                if let Some(above) = above {
                    if last_above != Some(above) {
                        last_above = Some(above);
                    }
                } else {
                    last_above = None;
                }
                if region != left {
                    // If this side was not started last round add it.
                    if last_right[c as usize] != region || last_left[c as usize] == region {
                        if let Some(region) = region {
                            rv[*region].1 += 1;
  //                          println!("Adding vertical left of {:?} for {}", rc, region);
                        }
                    }
                    if last_left[c as usize] != left || last_right[c as usize] == left {
                        if let Some(left) = left {
                            rv[*left].1 += 1;
  //                          println!("Adding vertical right of {:?} for {}", left_rc, left);
                        }
                    }
                }
                if let Some(region) = region {
                    if last_right[c as usize] != Some(region) {
                        last_right[c as usize] = Some(region);
                    }
                } else {
                    last_right[c as usize] = None;
                }
                if let Some(left) = left {
                    if last_left[c as usize] != Some(left) {
                        last_left[c as usize] = Some(left);
                    }
                } else {
                    last_left[c as usize] = None;
                }
                if let Some(region) = region {
                    rv[*region].0 += 1;
                }
            }
        }
        rv
    }

    let mut regions = vec![];
    while !g.is_empty() {
        regions.push(pop_region(&mut g));
    }
    let map = regions
        .iter()
        .enumerate()
        .flat_map(|(i, region)| region.iter().map(move |rc| (*rc, i)))
        .collect::<BTreeMap<_, _>>();
    let regions = scan(&map, mr, mc, &regions);
    println!(
        "{}",
        regions
            .into_iter()
            .map(|(area, perimeter)| area * perimeter)
            .sum::<usize>()
    );
}

fn _main_13_1() {
    let mut s = String::new();
    let _ = io::stdin().lock().read_to_string(&mut s);
    let machines = s
        .split("\n\n")
        .map(|s| s.split('\n').collect::<Vec<_>>())
        .map(|m| {
            let extract = |i, o| {
                (m[i] as &str)
                    .split(' ')
                    .skip(o)
                    .flat_map(|s| {
                        s.split(|c| c == '+' || c == '=')
                            .skip(1)
                            .map(|s| s.replace(",", "").parse())
                            .map(Result::unwrap)
                    })
                    .collect::<Vec<u64>>()
            };
            (extract(0, 2), extract(1, 2), extract(2, 1))
        })
        .collect::<Vec<_>>();
    let sum = machines
        .into_iter()
        .flat_map(|(a, b, tgt)| {
            (0..=100)
                .flat_map(|n| {
                    (0..=100)
                        .find(|m| {
                            let x = n * a[0] + m * b[0];
                            let y = n * a[1] + m * b[1];
                            x == tgt[0] && y == tgt[1]
                        })
                        .map(|m| 3 * n + m)
                })
                .min()
        })
        .sum::<u64>();
    println!("{:?}", sum);
}

fn _main_13_2() {
    let mut s = String::new();
    let _ = io::stdin().lock().read_to_string(&mut s);
    let machines = s
        .split("\n\n")
        .map(|s| s.split('\n').collect::<Vec<_>>())
        .map(|m| {
            let extract = |i, o| {
                (m[i] as &str)
                    .split(' ')
                    .skip(o)
                    .flat_map(|s| {
                        s.split(|c| c == '+' || c == '=')
                            .skip(1)
                            .map(|s| s.replace(",", "").parse())
                            .map(Result::unwrap)
                    })
                    .collect::<Vec<i128>>()
            };
            (
                extract(0, 2),
                extract(1, 2),
                extract(2, 1)
                    .into_iter()
                    .map(|x| x + 10000000000000)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (a, b, tgt) in machines {
        let q = a[1] * b[0] - a[0] * b[1];
        let r = tgt[1] * b[0] - tgt[0] * b[1];
        if r % q == 0 {
            let n = r / q;
            if (tgt[0] - a[0] * n) % b[0] == 0 {
                let m = (tgt[0] - a[0] * n) / b[0];
                if n >= 0 && m >= 0 {
                    sum += 3 * n + m;
                }
            }
        }
    }
    println!("{:?}", sum);
}

fn _main_14_1() {
    fn pair<A, T: Iterator<Item = A> + Sized>(mut i: T) -> (A, A) {
        (i.next().unwrap(), i.next().unwrap())
    }
    let v = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| {
            pair(s.split(' ').map(|s| {
                pair(
                    s.split('=')
                        .skip(1)
                        .next()
                        .unwrap()
                        .split(',')
                        .map(|s| s.parse::<i64>().unwrap()),
                )
            }))
        })
        .collect::<Vec<_>>();
    // heuristic
    let (mx, my) = if v.iter().map(|((x, _), _)| *x).max().unwrap() >= 11 {
        (101, 103)
    } else {
        (11, 7)
    };
    let v = (0..100).fold(v, |v, _| {
        v.into_iter()
            .map(|((x, y), (vx, vy))| (((x + vx + mx) % mx, (y + vy + my) % my), (vx, vy)))
            .collect()
    });
    let (nw, ne, sw, se) =
        v.into_iter()
            .map(|(p, _)| p)
            .fold((0, 0, 0, 0), |(nw, ne, sw, se), r| {
                let (x, y) = r;
                if y < my / 2 {
                    if x < mx / 2 {
                        (nw + 1, ne, sw, se)
                    } else if x > mx / 2 {
                        (nw, ne + 1, sw, se)
                    } else {
                        (nw, ne, sw, se)
                    }
                } else if y > my / 2 {
                    if x < mx / 2 {
                        (nw, ne, sw + 1, se)
                    } else if x > mx / 2 {
                        (nw, ne, sw, se + 1)
                    } else {
                        (nw, ne, sw, se)
                    }
                } else {
                    (nw, ne, sw, se)
                }
            });
    println!("{}", nw * ne * sw * se);
}

fn _main_14_2() {
    fn pair<A, T: Iterator<Item = A> + Sized>(mut i: T) -> (A, A) {
        (i.next().unwrap(), i.next().unwrap())
    }
    let v = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| {
            pair(s.split(' ').map(|s| {
                pair(
                    s.split('=')
                        .skip(1)
                        .next()
                        .unwrap()
                        .split(',')
                        .map(|s| s.parse::<i128>().unwrap()),
                )
            }))
        })
        .collect::<Vec<_>>();
    // heuristic
    let (mx, my) = if v.iter().map(|((x, _), _)| *x).max().unwrap() >= 11 {
        (101, 103)
    } else {
        (11, 7)
    };
    fn occupied(v: &Vec<((i128, i128), (i128, i128))>, x: i128, y: i128) -> usize {
        v.iter()
            .filter(|((px, py), _)| *px == x && *py == y)
            .count()
    }
    fn print(v: &Vec<((i128, i128), (i128, i128))>, mx: i128, my: i128) {
        for y in 0..my {
            let s = (0..mx)
                .map(|x| {
                    let cnt = occupied(v, x, y);
                    if cnt >= 1 {
                        (cnt as u8 + b'0') as char
                    } else {
                        ' '
                    }
                })
                .collect::<String>();
            println!("{}", s);
        }
    }
    // heuristic entropy measure
    fn entropy(v: &Vec<((i128, i128), (i128, i128))>, mx: i128, my: i128) -> i128 {
        const N: i128 = 2;
        let map = v.iter().fold(HashMap::new(), |mut map, ((x, y), _)| {
            map.entry((*x / (mx / N), *y / (my / N)))
                .and_modify(|e| *e += 1)
                .or_insert(1);
            map
        });
        (0..(mx / N)).fold(1, |e, x| {
            (0..(my / N)).fold(e, |e, y| {
                e.checked_mul(*map.get(&(x, y)).unwrap_or(&1))
                    .unwrap_or(i128::MAX)
            })
        })
    }
    let mut min = i128::MAX;
    let mut guess = None;
    let _ = (0..(mx * my)).fold(v, |v, i| {
        if entropy(&v, mx, my) < min {
            min = entropy(&v, mx, my);
            guess = Some(i);
//            print(&v, mx, my);
        }
        v.into_iter()
            .map(|((x, y), (vx, vy))| (((x + vx + mx) % mx, (y + vy + my) % my), (vx, vy)))
            .collect()
    });
    println!("{}", guess.unwrap());
}

fn main() {
    _main_14_2();
    //gpu_main();
}

// GPU shader support
//mod day01;

// fn gpu_main() {
//     let mut input = Vec::new();
//     io::stdin().read_to_end(&mut input).expect("Failed to read stdin");
//     let (dist, freq_score) = day01::solve_day01(&input);
//     println!("Total distance: {}", dist);
//     println!("Similarity score: {}", freq_score);
// }
