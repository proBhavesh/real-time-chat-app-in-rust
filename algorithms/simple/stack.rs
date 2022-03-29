fn main() {
	println!("this is stack");
	let mut stack=create_stack();
	println!("stack created {:?}",stack);
	add_items(&mut stack, 1);
	add_items(&mut stack, 1);
	add_items(&mut stack, 1);
	add_items(&mut stack, 1);
	println!("Items added{:?}", stack);
	println!("pop_item: {:?}", pop_item(&mut stack).unwrap());
	println!("stack after popping{:?}", stack);
}

fn create_stack()->Vec<i32>{
	let mut stack: Vec<i32>=Vec::new();
	//or 
	//let mut stack=vec![];
	return stack;
}

fn check_empty(stack:&mut Vec<i32>)->bool{
	return stack.len()==0;
}

fn add_items(stack:&mut Vec<i32>, item:i32){
	stack.push(item);
}

fn pop_item(stack:&mut Vec<i32>)->Option<i32>{
	// if(stack.len()==0){
	// 	let msg="stack empty";
	// 	return (msg);
	// };
	return stack.pop();
}