extern crate munkres;

#[test]
fn it_solves_problem_of_size_six() {
    let weights: &Vec<Vec<u32>> = &vec![
        vec![13, 13, 19, 50, 33, 38],
        vec![73, 33, 71, 77, 97, 95],
        vec![20,  8, 56, 55, 64, 35],
        vec![26, 25, 72, 32, 55, 77],
        vec![83, 40, 69,  3, 53, 49],
        vec![67, 20, 44, 29, 86, 61],
    ];
    let solution = vec!((0,4), (1,1), (2,5), (3,0), (4,3), (5,2));

    let mut problem = munkres::Problem::new(weights);
    assert_eq!(problem.solve(), solution);
}

#[test]
fn it_solves_problem_of_size_ten() {
    let weights: &Vec<Vec<u32>> = &vec![
        vec![612, 643, 717,   2, 946, 534, 242, 235, 376, 839],
        vec![224, 141, 799, 180, 386, 745, 592, 822, 421,  42],
        vec![241, 369, 831,  67, 258, 549, 615, 529, 458, 524],
        vec![231, 649, 287, 910,  12, 820,  31,  92, 217, 555],
        vec![912,  81, 568, 241, 292, 653, 417, 652, 630, 788],
        vec![32,  822, 788, 166, 122, 690, 304, 568, 449, 214],
        vec![441, 469, 584, 633, 213, 414, 498, 500, 317, 391],
        vec![798, 581, 183, 420,  16, 748,  35, 516, 639, 356],
        vec![351, 921,  67,  33, 592, 775, 780, 335, 464, 788],
        vec![771, 455, 950,  25,  22, 576, 969, 122,  86,  74],
    ];
    let solution = vec![(0, 7), (1, 9), (2, 3), (3, 6), (4, 1), (5, 0), (6, 5), (7, 4), (8, 2), (9, 8)];

    let mut problem = munkres::Problem::new(weights);
    assert_eq!(problem.solve(), solution);
}
