fn f(mut v: Vec<usize>) {
	v.push(1);
	println!("{:?}", v);
}


fn main() {
	let v = Vec::new();

	f(v);

	let mut arr = vec!{vec![1], vec![2, 100], vec![3], vec![4]};

	// let r = arr.iter_mut()
	// 	.map(|x: &mut i32| { *x = (*x).pow(*x as u32); x})
	// 	.collect::<Vec<_>>();

	let rr: Vec<i32> = arr.into_iter()
		.map(|vc| vc.iter().sum())
		.collect();

	let mut i = &(1 + rr[0]);
	i = &rr[2];

	println!("{:?}", rr);
}