fn main() {
	let mut queue=create_queue();
	add_to_queue(&mut queue,1);
	add_to_queue(&mut queue,2);
	add_to_queue(&mut queue,3);
	display_queue(&mut queue);
	display_queue_size(&mut queue);
	add_to_queue(&mut queue,4);
	add_to_queue(&mut queue,5);
	display_queue(&mut queue);
	display_queue_size(&mut queue);
	remove_from_queue(&mut queue);
	display_queue(&mut queue);
	display_queue_size(&mut queue);
}

fn create_queue()->Vec<i32>{
	let mut queue: Vec<i32>=Vec::new();
	queue
}

fn add_to_queue(queue: &mut Vec<i32>, item: i32){
	queue.push(item);
}

fn remove_from_queue(queue: &mut Vec<i32>){
	queue.remove(0);
}

fn display_queue(queue: &mut Vec<i32>){
	println!("queue: {:?}", queue);
}

fn display_queue_size(queue: &mut Vec<i32>){
	println!("Size of queue: {:?}", queue.len());
}