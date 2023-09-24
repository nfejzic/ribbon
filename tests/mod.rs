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
    let mut band: Band<3, _, _> = Band::new(0..4);
    band.expand_n(2); // consume 0, 1 from iterator

    assert_eq!(band.len(), 2);
    assert_eq!(band.peek_front(), Some(&0));
    assert_eq!(band.peek_back(), Some(&1));

    // just expands, no need to pop first item
    assert_eq!(band.progress(), None); // consume 3 from iterator

    // needs space, pops first item
    assert_eq!(band.progress(), Some(0)); // consumes 4 from iterator, iterator has no more values

    // iterator does not produce more values, progress becomes no-op. No extra capacity is needed,
    // hence progress does not return any more values.
    assert_eq!(band.progress(), None);
}
