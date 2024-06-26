use std::collections::HashMap;
type Mapping = HashMap<String, Vec<String>>;

fn iterating_mapping(mapping: &Mapping) {
    for (key, values) in mapping {
        //it takes the ownership for the input hashmap
        println!("key is: {} and its value: ", key);
        for value in values {
            /*
            here takes the ownership of vector for the given key
            we can't visit the value of the given key again

            if using the shared reference the type of the key will change from
            Vec<String> -> &Vec<String> -> value in the vector will change to reference too
            */
            println!("   {}", value);
        }
    }
}
/*
1, shared reference, which means we can allowing keeping the ownership at the same time
allow other guys read into the object, &, &Mapping => shared reference

but the shared reference can't allow to write if you are not the owner.

2, mutable reference, this will allow one writer who can write into the object but without
the transfering of the ownership. &mut Mapping, symbol for mutable reference is &mut

3, reference is recursive, if we have reference for HashMap, then we have reference
for its key and value
*/

fn sort_mapping(mapping: &mut Mapping) {
    for (key, value) in mapping {
        /*
        we change the content of the object without the ownership of the object.
        even though there is not transfer of ownership, but the oringinal owner
        lost the right of writing into the object
        */
        value.sort();
    }
}
//static -> the variable will only end when the progrom ends
static mut global_variable: &i32 = &128;
/*
make sure life span of global_variable should not shorter than the life span of the
object inside, but how about the object inside has the same life span as the global_variable,
, we need to tell the compiler that the life span of the box and the object inside can be the same

how to tell? life span indicator=> 'any_string => 'static => indicate the object has life
as long as the progroam,
*/
fn f(p: &'static i32) {
    unsafe {
        /*
        unsafe is telling the compiler that we know the piece of code here will break
        the safty rules of Rust, but We will take the cost fot that, if running the
        piece of the code here will shoot our own feet, we wan't blame you!
        */
        /*
        the object inside p may destroyed before the program ends, but
        the global_variable as a box will end when the program ends,
        the life span of the object inside will shorter than the box itself
        */
        global_variable = p;
    }
}

/*
life span indicator is the symbol of ' following any string, static is a special string
that used to indicate the life length is long as the program
*/

// fn construct_string(str_one: &mut String, str_two: &mut String) -> &String {
//     str_one.push_str("  returned");
//     str_one
// }
fn construct_string<'str_one_life_span, 'str_two_life_span>(
    str_one: &'str_one_life_span mut String,
    str_two: &'str_two_life_span mut String,
) -> &'str_one_life_span String {
    str_one.push_str("  returned");
    str_one
}
/*
we want to insert item from one collection to other, for example get item from a list
and insert into other list, get item from hashamp and insert into other hashmap

what about the two collections are the same one ?
when you are iterating an collection and at the same time you change the collection?
for example iteration a list, and you remove element of the list in the loop -> tricky bug
*/
fn extend_vec(vec: &mut Vec<f64>, slice: &[f64]) {
    for elem in slice {
        //dangling pointer problem
        vec.push(*elem);
    }
}

fn main() {
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = vec![0.0, -1.0];

    extend_vec(&mut wave, &head);
    extend_vec(&mut wave, &tail);

    println!("{:?}", wave);
    /*
    Since an object can only be mutable reference or share reference at one time,
    it is not allowed to have object being mutable referenced and share referenced at
    the same time

    tricky!!!:
    wave vector has lengh of 4 and capacity of 4 => the capacity of the vector wave is full,
    when we want to add item to a collection with capacity full, the program will destroy memory
    of the old collection and create a new memory with bigger capacity and insert the old elements
    into the new memory.

    when extend_vec is running, the memory of wave will be destroied, but second parameter
    which is wave again still referencing the old memory

    //dangling pointer problem
    */
    //extend_vec(&mut wave, &wave);

    //mutable reference and share reference work together
    // let mut x = 10;
    // let r1 = &x;
    // let r2 = &x; //ok we can have multiple share reference
    // let m = &mut x; //error, not share and mutale reference at the same time
    // println!("{}, {}", m, r1);

    // let mut y = 20;
    // let m1 = &mut y; //ok
    //                  //let m2 = &mut y; //error, only one mutable reference at the same time
    // let z = y; //error, y is already mutable referenced, and it can't be moved
    // println!("{}, {}", m1, z);

    // let mut w = (107, 108);
    // let r = &w;
    // let r0 = &r.0; //ok,
    // let m1 = &mut r.1; //error r is shafe reference, its fild can't mutbale reference
    // let m2 = &mut w.1; //is it ok?
    // println!("{}, {}", r0, m1);

    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0; //this ok
    *m0 = 140; //ok

    let r1 = &m.1; // ok too
    println!("{}", r1);
}
