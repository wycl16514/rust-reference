In previous two sections, we spend pains taking effort to make sure that when using reference, the rule of "life span of object inside the box should longer than the box" not to be broken. But there are plenty of
ways to make you break the rule no matter how careful you are, such as:
```rs
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let r = &v;
    let aside = v;
    r[0]; //erro, the object inside the box is invaldiated
}
```
As the code shown aboved, we put v inside the box of r, then we move the ownership of the vector from r to aside, this will cause v to be invalidate, when the code r[0] is executed, it is getting the object
inside the box of r out and access its first elment, the object is v but v is invalidated at the time the code r[0] is executed, therefore the rule is broken. The principle here is that, as long as you put
the object inside a box, then you can touch the object unless you touch the object throunging the box.

It is crazy hard to guard against the rule from breaking. Let's see another trap that might cause us the pain:
```rs
fn extend_vec(vec: &mut Vec<f64>, slice: &[f64]) {
    /*
    append a slice to the end of given vector
    */
    for elem in slice {
        vec.push(*elem);
    }
}
```
The aboved function is used to append the second slice to the end of the first vector, we can use the aboved function as following:
```rs
 let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = vec![0.0, -1.0];

    extend_vec(&mut wave, &head);
    extend_vec(&mut wave, &tail);

    println!("{:?}", wave);
```
but the tricky thing is when we append a vector to the end of itself as following:
```rs
//what happend if a vector append to itself
    extend_vec(&mut wave, &wave);
```

The very tricky thing in the line of code aboved is that, after two calls of extend_vec, the vector owned by wave has capacity of 4 and it has 4 elements inside it which means its capacity is full.If we try to
add new element into the vector, the compiler will destroy the vector and recreate a new vector and push those elements into the new vector. But when the compiler destroying the old vector, the slice parameter
still referencing to the old vector, this is like the snake eating its tail.

This sort of bug is very difficult to catch in other langauges like c++ or java,but it is rather easy to be catched by Rust because Rust will not allowing putting the same object into more than one box at the 
sametime. That's why you will get the error if you try to compile the code:

![截屏2024-06-25 01 30 33](https://github.com/wycl16514/rust-reference/assets/7506958/0fa2b417-fa9b-47d5-843b-41895960f441)

As we metionded before there are two kinds of reference, one is shared reference, when an object is being share referenced, there is impossible to have mutable reference to it again until all share references
are removed. If an object is being mutable referenced, then there is no possible to have any share or mutable reference to the object any more until the mutable reference is removed. Therefore we can't have 
share reference and mutable reference at the same time just like "extend_vec(&mut vec, &vec);" as shown.

And since the first argument is mutable reference, then the compiler will not allow any reference no matter it is share or mutable, and the second arugment is share reference, then the compiler just report
you can't have share reference to an object that is already under the mutable reference.

Let's use more code example to show the principle of "not share and mutable reference at the same time":
```rs
let mut x = 10;
    let r1 = &x;
    let r2 = &x; //it is ok to have mutiple share reference
                 /*
                 error,
                 x is putted into box you can access x whitout going throug the box
                 */
    //x += 10;
    /*
    try to comment the following line and compile is ok, if the code never
    acess the reference, then compiler will just remove the reference aboved
    and x is never being putted into box eventhoung the code aboved
    */
    println!("x is :{}", r1);

    /*
    error, share and mutable reference can not happen at the same time.
    */
    let m = &mut x;
    println!("{}, {}", m, r2); //comment out this the code can compile
```
The aboved code shows the breaking of the rule "share and mutable reference can't happen at the same time", if you try to comment out the last line, then the code can be compiled that's because you never
access the object throung a share and mutable reference at the same time, therefore the line "let m=&mut x;" will not cause trouble since it has not effect on the program, and the compiler will just ignore
it and the mutable reference is just like never happening.

Let's see another example of breaking the rule:
```rs
  let mut y = 20;
    let m1 = &mut y;
    let m2 = &mut y; //only allow one mutable reference
    let z = y; //y is already putted into box
    println!("{} {} {}", m1, m2, z);
```

Another tricky situation:
```rs
 let mut w = (107, 109);
    let r = &w;
    //parent is share referenced then its field can be share referenced too
    let r0 = &r.0;
    //error, parent is share referenced, then fields can only be share referenced
    let m1 = &mut r.1;
    println!("{}", r0);
```

Let's see another confusing one:
```rs
 let mut v = (136, 139);
    let m = &mut v;
    /*
    parent is mutable reference, then fields can be mutable reference
    */
    let m0 = &mut m.0;
    *m0 = 137;
    /*
    parent is  mutable reference and its field can be share reference
    */
    let r1 = &m.1;
    m.1 = 140; //error, m.1 is already share reference
    println!("{}", r1);
```
