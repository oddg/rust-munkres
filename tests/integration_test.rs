extern crate munkres;

#[test]
fn it_solves_problem_of_size_six() {
    let weights: &Vec<Vec<u32>> = &vec![
        vec![13, 13, 19, 50, 33, 38],
        vec![73, 33, 71, 77, 97, 95],
        vec![20, 8, 56, 55, 64, 35],
        vec![26, 25, 72, 32, 55, 77],
        vec![83, 40, 69, 3, 53, 49],
        vec![67, 20, 44, 29, 86, 61],
    ];
    let solution = vec!((0,4), (1,1), (2,5), (3,0), (4,3), (5,2));

    let mut problem = munkres::Problem::new(weights);
    assert_eq!(problem.solve(), solution);
}
