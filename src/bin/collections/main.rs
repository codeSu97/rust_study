fn main() {
    // Vector
    // create new Vector
    let v: Vec<i32> = Vec::new();
    // v.push(1);
    println!("v: {:?}", v);

    let mut v1 = Vec::new();
    v1.push(1);
    println!("v1: {:?}", v1);

    let mut v2 = Vec::with_capacity(16);
    v2.push(1);
    println!("v2: {:?}", v2);
    println!("v2[0]: {:?}", v2[0]);

    // init new Vector
    let mut v3 = vec![1, 2, 3];
    v3.push(4);
    let v3_0 = &v3[0];
    println!("v3: {:?}", v3);
    println!("v3[0]: {:?}", v3[0]);
    println!("v3.get(0): {:?}", v3.get(0));
    println!("v3_0: {:?}", v3_0);
}
