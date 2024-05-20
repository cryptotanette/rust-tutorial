fn main(){
    let v = vec![1,2,3]; 
    // vector v owns the object in heap
 
    //only a single variable owns the heap memory at any given time
    let v2 = v; 
    // here two variables owns heap value,
    //two pointers to the same content is not allowed in rust
 
    //Rust is very smart in terms of memory access ,so it detects a race condition
    //as two variables point to same heap

    // display(v2); // v2 is moved to display and v2 is invalidated
 
    // println!("{:?}",v); // incorrect
    println!("{:?}",v2); // correct

    display(v2);
}

fn display(v:Vec<i32>){
    println!("inside display {:?}",v);
}