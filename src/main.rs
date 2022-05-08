use clap::Arg;
use clap::Command;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Default)]
struct Rechenraetsel<const ALLOW_NEGATIVE: bool> {
    cache: HashMap<Box<[u8]>, (BTreeMap<i64, String>, BTreeMap<i64, [String; 2]>)>,
    cache_mul: HashMap<Box<[u8]>, (BTreeMap<i64, String>, BTreeMap<i64, [String; 2]>)>,
}

impl<const ALLOW_NEGATIVE: bool> Rechenraetsel<ALLOW_NEGATIVE> {
    fn possible_results(
        &mut self,
        digits: &[u8],
    ) -> (BTreeMap<i64, String>, BTreeMap<i64, [String; 2]>) {
        if digits.len() == 0 {
            return (
                [(0, String::from(""))].into_iter().collect(),
                BTreeMap::new(),
            );
        }
        match self.cache.get(digits) {
            Some(r) => return r.clone(),
            None => {}
        }
        let result = self.possible_results_uncached(digits);
        self.cache
            .insert(digits.to_owned().into_boxed_slice(), result.clone());
        result
    }
    // Split off the last summand.
    fn possible_results_uncached(
        &mut self,
        digits: &[u8],
    ) -> (BTreeMap<i64, String>, BTreeMap<i64, [String; 2]>) {
        let mut new_possible: BTreeMap<i64, String> = BTreeMap::new();
        let mut new_duplicates = BTreeMap::new();
        for sep in 0..digits.len() {
            let (possible1, duplicates1) = self.possible_results(&digits[..sep]);
            let (possible2, duplicates2) = self.possible_results_mul(&digits[sep..]);
            new_duplicates.extend(possible1.iter().flat_map(|(v1, l1)| {
                duplicates2.iter().map(move |(v2, la2)| {
                    (
                        v1 + v2,
                        [format!("{}+{}", l1, la2[0]), format!("{}+{}", l1, la2[1])],
                    )
                })
            }));
            new_duplicates.extend(duplicates1.iter().flat_map(|(v1, la1)| {
                possible2.iter().map(move |(v2, l2)| {
                    (
                        v1 + v2,
                        [format!("{}+{}", la1[0], l2), format!("{}+{}", la1[1], l2)],
                    )
                })
            }));
            new_duplicates.extend(duplicates1.iter().flat_map(|(v1, la1)| {
                duplicates2.iter().map(move |(v2, la2)| {
                    (
                        v1 + v2,
                        [
                            format!("{}+{}", la1[0], la2[0]),
                            format!("{}+{}", la1[1], la2[1]),
                        ],
                    )
                })
            }));

            if sep != 0 {
                if ALLOW_NEGATIVE {
                    new_duplicates.extend(possible1.iter().flat_map(|(v1, l1)| {
                        duplicates2.iter().map(move |(v2, la2)| {
                            (
                                v1 - v2,
                                [format!("{}-{}", l1, la2[0]), format!("{}-{}", l1, la2[1])],
                            )
                        })
                    }));
                    new_duplicates.extend(duplicates1.iter().flat_map(|(v1, la1)| {
                        possible2.iter().map(move |(v2, l2)| {
                            (
                                v1 - v2,
                                [format!("{}-{}", la1[0], l2), format!("{}-{}", la1[0], l2)],
                            )
                        })
                    }));
                    new_duplicates.extend(duplicates1.iter().flat_map(|(v1, la1)| {
                        duplicates2.iter().map(move |(v2, la2)| {
                            (
                                v1 - v2,
                                [
                                    format!("{}-{}", la1[0], la2[0]),
                                    format!("{}-{}", la1[1], la2[1]),
                                ],
                            )
                        })
                    }));
                } else {
                    new_duplicates.extend(possible1.iter().flat_map(|(v1, l1)| {
                        duplicates2.iter().flat_map(move |(v2, la2)| {
                            if v1 - v2 >= 0 {
                                Some((
                                    v1 - v2,
                                    [format!("{}-{}", l1, la2[0]), format!("{}-{}", l1, la2[1])],
                                ))
                            } else {
                                None
                            }
                        })
                    }));
                    new_duplicates.extend(duplicates1.iter().flat_map(|(v1, la1)| {
                        possible2.iter().flat_map(move |(v2, l2)| {
                            if v1 - v2 >= 0 {
                                Some((
                                    v1 - v2,
                                    [format!("{}-{}", la1[0], l2), format!("{}-{}", la1[1], l2)],
                                ))
                            } else {
                                None
                            }
                        })
                    }));
                    new_duplicates.extend(duplicates1.iter().flat_map(|(v1, la1)| {
                        duplicates2.iter().flat_map(move |(v2, la2)| {
                            if v1 - v2 >= 0 {
                                Some((
                                    v1 - v2,
                                    [
                                        format!("{}-{}", la1[0], la2[0]),
                                        format!("{}-{}", la1[1], la2[1]),
                                    ],
                                ))
                            } else {
                                None
                            }
                        })
                    }));
                }
            }

            let possible_plus: BTreeMap<i64, String> = possible1
                .iter()
                .flat_map(|(v1, l1)| {
                    possible2
                        .iter()
                        .map(move |(v2, l2)| (v1 + v2, format!("{}+{}", l1, l2)))
                })
                .collect();
            let possible_minus: BTreeMap<i64, String> = if sep != 0 {
                if ALLOW_NEGATIVE {
                    possible1
                        .iter()
                        .flat_map(|(v1, l1)| {
                            possible2
                                .iter()
                                .map(move |(v2, l2)| (v1 - v2, format!("{}-{}", l1, l2)))
                        })
                        .collect()
                } else {
                    possible1
                        .iter()
                        .flat_map(|(v1, l1)| {
                            possible2.iter().filter_map(move |(v2, l2)| {
                                if v1 - v2 >= 0 {
                                    Some((v1 - v2, format!("{}-{}", l1, l2)))
                                } else {
                                    None
                                }
                            })
                        })
                        .collect()
                }
            } else {
                BTreeMap::new()
            };

            new_duplicates.extend(new_possible.iter().filter_map(|(v1, l1)| {
                if let Some(l2) = possible_minus.get(v1) {
                    Some((*v1, [l1.clone(), l2.clone()]))
                } else {
                    None
                }
            }));
            if sep != 0 {
                new_duplicates.extend(possible_plus.iter().filter_map(|(v1, l1)| {
                    if let Some(l2) = possible_minus.get(v1) {
                        Some((*v1, [l1.clone(), l2.clone()]))
                    } else {
                        None
                    }
                }));
                new_duplicates.extend(new_possible.iter().filter_map(|(v1, l1)| {
                    if let Some(l2) = possible_minus.get(v1) {
                        Some((*v1, [l1.clone(), l2.clone()]))
                    } else {
                        None
                    }
                }));
            }

            let new_possible_prev = new_possible;
            new_possible = BTreeMap::new();
            new_possible.extend(
                possible_plus
                    .iter()
                    .filter(|(v, _)| !new_duplicates.contains_key(v))
                    .map(|(v, l)| (*v, l.clone())),
            );
            new_possible.extend(
                new_possible_prev
                    .iter()
                    .filter(|(v, _)| !new_duplicates.contains_key(v))
                    .map(|(v, l)| (*v, l.clone())),
            );
            if sep != 0 {
                new_possible.extend(
                    possible_minus
                        .iter()
                        .filter(|(v, _)| !new_duplicates.contains_key(v))
                        .map(|(v, l)| (*v, l.clone())),
                );
            }
        }
        (new_possible, new_duplicates)
    }

    fn possible_results_mul(
        &mut self,
        digits: &[u8],
    ) -> (BTreeMap<i64, String>, BTreeMap<i64, [String; 2]>) {
        if digits.len() == 0 {
            return (
                [(1, String::from(""))].into_iter().collect(),
                BTreeMap::new(),
            );
        }
        match self.cache_mul.get(digits) {
            Some(r) => return r.clone(),
            None => {}
        }
        let result = self.possible_results_mul_uncached(digits);
        self.cache_mul
            .insert(digits.to_owned().into_boxed_slice(), result.clone());
        result
    }
    fn possible_results_mul_uncached(
        &mut self,
        digits: &[u8],
    ) -> (BTreeMap<i64, String>, BTreeMap<i64, [String; 2]>) {
        let (possible, duplicates) = self.possible_results_mul(&digits[..digits.len() - 1]);
        let next_digit = digits[digits.len() - 1] as i64;
        if next_digit == 0 {
            if let Some((_, [l1, l2])) = duplicates.iter().next() {
                return (
                    BTreeMap::new(),
                    [(0, [format!("{}*", l1), format!("{}*", l2)])]
                        .into_iter()
                        .collect(),
                );
            } else {
                let mut iter = possible.iter();
                let (v1, l1) = iter.next().unwrap();
                if let Some((_, l2)) = iter.next() {
                    return (
                        BTreeMap::new(),
                        [(0, [format!("{}*", l1), format!("{}*", l2)])]
                            .into_iter()
                            .collect(),
                    );
                } else {
                    return (
                        [(0, format!("{}*", v1))].into_iter().collect(),
                        BTreeMap::new(),
                    );
                }
            }
        }
        let new_possible_mul: BTreeMap<i64, String> = possible
            .iter()
            .map(|(p, l)| (p * next_digit, format!("{}*", l)))
            .collect();
        let new_possible_div: BTreeMap<i64, String> = if digits.len() != 1 {
            possible
                .iter()
                .filter_map(|(p, l)| {
                    if p % next_digit == 0 {
                        Some((p / next_digit, format!("{}/", l)))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            BTreeMap::new()
        };
        let mut new_duplicates: BTreeMap<i64, [String; 2]> = BTreeMap::new();
        new_duplicates.extend(
            duplicates
                .iter()
                .map(|(p, [l1, l2])| (p * next_digit, [format!("{}*", l1), format!("{}*", l2)])),
        );
        if digits.len() != 1 {
            new_duplicates.extend(duplicates.iter().filter_map(|(v, la)| {
                if v % next_digit == 0 {
                    Some((
                        v / next_digit,
                        [format!("{}/", la[0]), format!("{}/", la[1])],
                    ))
                } else {
                    None
                }
            }));
            new_duplicates.extend(new_possible_mul.iter().filter_map(|(&v1, l1)| {
                if let Some(l2) = new_possible_div.get(&v1) {
                    Some((v1, [l1.clone(), l2.clone()]))
                } else {
                    None
                }
            }));
        }
        (
            new_possible_mul
                .iter()
                .filter(|(v1, _)| !new_possible_div.contains_key(v1))
                .chain(
                    new_possible_div
                        .iter()
                        .filter(|(v1, _)| !new_possible_mul.contains_key(v1)),
                )
                .filter(|(v, _)| !new_duplicates.contains_key(v))
                .map(|(&v, l)| (v, l.clone()))
                .collect(),
            new_duplicates,
        )
    }
}

fn main() {
    let matches = Command::new("rechenraetsel")
        .author("Tobias")
        .arg(
            Arg::new("digits")
                .value_name("DIGITS")
                .required(true)
                .help("Digits, one character per digit. E.g. \"443\"."),
        )
        .arg(
            Arg::new("result")
                .value_name("RESULT")
                .help("Result to test for."),
        )
        .arg(
            Arg::new("no-negative-partials")
                .long("--no-negative-partials")
                .help("Disallow negative partial sums"),
        )
        .get_matches();

    let mut digits = Vec::new();
    for c in matches.value_of("digits").unwrap().chars() {
        if !('0' <= c && c <= '9') {
            panic!("invalid digit {}", c);
        }
        digits.push(c as u8 - '0' as u8);
    }
    let result: Option<i64> = if matches.is_present("result") {
        Some(matches.value_of_t_or_exit("result"))
    } else {
        None
    };

    let (possible, duplicates) = if matches.is_present("no-negative-partials") {
        Rechenraetsel::<false>::default().possible_results(&digits)
    } else {
        Rechenraetsel::<true>::default().possible_results(&digits)
    };

    if let Some(r) = result {
        if possible.contains_key(&r) {
            println!("unique");
        } else if duplicates.contains_key(&r) {
            println!("duplicate");
        } else {
            println!("impossible");
        }
    } else {
        println!("uniques: {:?}", possible);
        println!("duplicates: {:?}", duplicates);
    }
}
