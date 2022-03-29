fn main() {
	let mut a = [1,2,3,4,5];
	factorial(&a);
}

fn factorial(arr: [i32]) -> [i32] {
	let mut b=[];
	for i in 0..arr.len(){
		println!("{:?}",i );
		a[i]=a[i]*a[i]
	};
	println!("{:?}",a );
}

// const arr = [1, 2, 3, 4, 5, 6]

// const factorial = (arr) => {
//   for (i = 0; i < arr.length; i++) {
//   answer = 1;
//     for (b=arr[i];b>=1;b--){
//       answer = answer * b;
//       console.log("this is answer", answer)
//     }
//     console.log("This is answer", answer)
//   }
//   console.log("this is answer2", answer)
// }
// console.log(factorial(arr))
