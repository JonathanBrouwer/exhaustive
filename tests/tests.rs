use exhaustive::Exhaustive;

#[test]
fn test_bool() {
    assert_eq!(
        bool::iter_exhaustive(2).collect::<Vec<_>>(),
        vec![false, true],
    );
}

#[test]
fn test_vec_bool() {
    assert_eq!(
        Vec::<bool>::iter_exhaustive(3).collect::<Vec<_>>(),
        vec![
            vec![],
            vec![false],
            vec![true],
            vec![false, false],
            vec![false, true],
            vec![true, false],
            vec![true, true],
        ],
    );
}



#[test]
fn test_larger() {
    assert_eq!(
        Vec::<bool>::iter_exhaustive(8).count(),
        255
    )
}

#[test]
fn test_vec_unit() {
    assert_eq!(
        Vec::<()>::iter_exhaustive(4).count(),
        5
    )
}