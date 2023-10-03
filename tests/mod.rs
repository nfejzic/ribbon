use ribbon::Ribbon;

#[test]
fn test_tape() {
    use ribbon::Tape;

    let mut tape = Tape::new(0..10);
    tape.expand_n(5);

    assert_eq!(tape.len(), 5);
    assert_eq!(tape.peek_front(), Some(&0));
    assert_eq!(tape.peek_back(), Some(&4));
}

#[test]
fn test_band() {
    use ribbon::Band;

    // Band with capacity for 5 items
    let mut band: Band<3, _> = Band::new(0..4);
    band.expand_n(2); // consume 0, 1 from iterator

    assert_eq!(band.len(), 2);
    assert_eq!(band.peek_front(), Some(&0));
    assert_eq!(band.peek_back(), Some(&1));

    // "slides" over the items from iterator -> returns first and expands by 1
    assert_eq!(band.progress(), Some(0)); // consume 3 from iterator
    assert_eq!(band.progress(), Some(1)); // consumes 4 from iterator, iterator has no more values

    // iterator does not produce more values, progress becomes no-op.
    assert_eq!(band.progress(), None);
}

#[test]
fn test_enroll() {
    use ribbon::Enroll;

    let iter = 0..10;

    let mut tape = iter.tape();
    tape.expand_n(5);
    assert_eq!(tape.progress(), Some(0));
    assert_eq!(tape.peek_at(2), Some(&3));

    let iter = 0..10;

    let mut band = iter.band::<5>();
    band.expand_n(3);
    assert_eq!(band.progress(), Some(0));
    assert_eq!(band.progress(), Some(1));
}
