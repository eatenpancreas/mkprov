// use std::fmt::Display;

// #[test]
// fn test_capacity() {
//     let oldvec = vec![1_u64, 4, 5, 1, 2, 3, 4, 8, 2];
//     let mut large_ints = Vec::<u64>::new();
//     display_push(&mut large_ints, &oldvec, "OG int array").unwrap();

//     // empty vec
//     display_push(&mut Vec::<u8>::new(), &large_ints, "empty vec").unwrap();

//     // with capacity
//     let mut withcap = Vec::<u8>::with_capacity(large_ints.capacity());
//     display_push(&mut withcap, &large_ints, "with capacity of OG").unwrap();

//     // with len
//     let mut withlen = Vec::<u8>::with_capacity(large_ints.len());
//     display_push(&mut withlen, &large_ints, "with capacity of the OG Len").unwrap();

//     // with collect
//     let withcollect: Vec<_> = oldvec.iter().map(|i| *i as u8).collect();
//     print_capacity(&withcollect, "with collect");
// }

// fn display_push<A: Display, B: TryInto<A> + Copy>(
//     newvec: &mut Vec<A>,
//     oldvec: &Vec<B>,
//     name: &str,
//     expected_cap: usize,
// ) -> Result<(), B::Error> {
//     for i in oldvec {
//         newvec.push((*i).try_into()?);
//     }
//     print_capacity(&newvec, name, expected_cap);
//     Ok(())
// }

// fn print_capacity<T: Display>(vec: &Vec<T>, name: &str, expected_cap: usize) {
//     // assert_eq!(assert_eq!(vec.capacity(), expected_cap));
//     // println!("{name}: (capacity={}, len={})", vec.capacity(), vec.len());
// }
