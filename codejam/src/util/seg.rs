
fn seg_intersect<T>( seg1: &(T, T), seg2: &(T, T) ) -> bool
where T : std::cmp::PartialOrd
{
    assert!(seg1.0 <= seg1.1);
    assert!(seg2.0 <= seg2.1);
    //https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-two-integer-ranges-for-overlap
    return seg1.0 <= seg2.1 && seg2.0 <= seg1.1  ;
} 

#[test]
fn test_seg_intersect() {
    assert!(!seg_intersect::<u64>( &(1,2), &(3,4) ));
    assert!(!seg_intersect::<i64>( &(-4,-3), &(-2,-1) ));

    assert!(seg_intersect::<u8>( &(1,14), &(3,5) ), "seg2 fully inside seg1");
    assert!(seg_intersect::<f64>( &(3.1,3.2), &(2.9,5.1) ), "seg1 fully inside seg2");

    assert!(seg_intersect::<u8>( &(1,4), &(4,5) ), "endpoint shared");
    assert!(seg_intersect::<i8>( &(-1,3), &(-4,-1) ), "endpoint shared");
}