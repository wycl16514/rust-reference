In previous section, we haven seen the only one onwer principle and find out that, when we ressign an object from one owner to a new owner,
the original owner becomes invalidated such as:

```r
let a = vec![1,2,3];
let b = a; //a becomes invalidated
```

As aboved code shows, if you try to access a after the code "let b = a;", then a panic will happen and the program crashes. This mechanism is
somehow inintutive and unconvinent. Since most of other programming languages will allow the continue using of of a. What can we do to enable
the normal way as other languages do and do not break the safty assurance Rust provides to us? 

That's how the reference and borrowing mechanism comes into being! According to our trandition, Let's put out a piece of code first:
```rs
use std::collections::HashMap;
//align the type mapping as a hashmap
type Mapping = HashMap<String, Vec<String>>;

fn iterating_mapping(mapping: Mapping) {
    for (key, values) in mapping {
        //here takes the ownership for the input hashmap
        println!("key is: {} and its value is:", key);
        for value in values {
            //here takes the ownership of vector for the given key
            //we can't visit the value for the given key again!
            println!("  {}", value);
        }
    }
}

fn main() {
    let mut mapping = Mapping::new();
    mapping.insert(
        "key1".to_string(),
        vec!["2".to_string(), "1".to_string()],
    );
    mapping.insert(
        "key2".to_string(),
        vec!["4".to_string(), "3".to_string()],
    );

    iterating_mapping(mapping); //ownership of mapping is moved and mapping is invalidated
    println!("value of key1 is:{:?}", mapping["key1"]); //panic here
}

```
As you can see from aboved code, in the bottom there is a comment saying the ownership of the map is transfer from variable mapping
to the function being called, if we want to access the hashmap by using the mapping variable again, the panic occurs. This is far different
from other programming language, such as python, golang, the mapping variable will still accessible after the function call and the value
for the given key can be accessed in next time!

How can we make the Rust code "normal" as other language? We can use reference here. Reference comes in to ways, the first one is shared 
reference, which means you won't take the ownership of the object, but you can peek into the content of the object. The second one is mutable
reference, you can read and modify the content of given object without taking the ownership. The difference about this two reference is that,
shared reference can be used on many places at the sametime, and the second one is exclusive, if some one gain the mutable reference, not 
other one can have shared or mutable reference on the same object any more!

The symbolc to indicate shared reference is &, and the symbol for mutable reference is &mut. The shared reference is like multiple readers, any one of them can read
to the same content as long as the content is not changed. Mutable reference is like selecting only one writer from multiple candidates, if someone otherthan the 
original owner is given the mutable reference, even the original owner will lost the right of writting to the object which is owned by it.

According to theory aboved, since the show function in the code example need only the right of reading into the object, then we can utilize the shared reference to
solve the problem, that is we change the code aboved a little bit as following:
```rs
//fn iterating_mapping(mapping: Mapping) {
fn iterating_mapping(mapping: &Mapping) {
    // using shared reference
    for (key, values) in mapping {
        //here takes the ownership for the input hashmap
        println!("key is: {} and its value is:", key);
        for value in values {
            //here takes the ownership of vector for the given key
            //we can't visit the value for the given key again!
            println!("  {}", value);
        }
    }
}

fn main() {
    ....
     // iterating_mapping(mapping); //ownership of mapping is moved and mapping is invalidated
    iterating_mapping(&mapping); //using shared reference then no panic below
    println!("value of key1 is:{:?}", mapping["key1"]); //panic here
}
```
The paremeter for function iterating_mapping changed from value of Mapping to &Mapping, a reference symbol prefixes before the Mapping type. Iterating a shared 
reference to HashMap will cause the reading of its key and value as shared reference. Therefore the type of the key is changed from String to &String and the value
for the given key is changed from Vec<String> to &Vec<String>. The rule applied recursively, when when we iterating a reference vec type, then each element in the 
vec turns into shared reference, that is when we get the element from &Vec<String>, the type of element changes from String to &String.

How about we want to change the content of the HashMap by a function. Then we need to use mutable reference in the function, for example:

```rs
fn sort_mapping(mapping: &mut Mapping) {
    /*
    Since we will change the HashMap, we need to use mutable reference
    */
    for (key, value) in mapping {
        value.sort();
    }
}

fn main() {
  ....
    sort_mapping(&mut mapping);
    println!("mapping after sort: {:?}", mapping);
}
```

Let's summerize something related to above code. If we passing an object to a function and causing the ownership transfer, then we call this passing by value, if 
we passing the reference of the object to a function and keep the ownership to its oringinal owner , we call this passing by reference. If we passing an object to
function and allow the function to write into the object and keeping the ownership unchanged, we call this passing mutable reference.

As we have seen aboved, reference is usful when we want to pass an object to a function without lossing the ownership. But this is only the tip of the iceberg, there
are many details or traps covered by the symbol of & and &mut. As we have seen before, when we want to access the value with reference type, we need the * operator
to undo the reference such as :
```r
fn main() {
    let x = 10;
    let r = &x;
    println!("value of is r equals to 10?: {}", r == 10);
}
```
The aboved code will cause error, but if we change to following it would be ok:
```r
fn main() {
    let x = 10;
    let r = &x;
    //error to access the value of reference
    //println!("value of is r equals to 10?: {}", r == 10);

    //need to dereference by using *
    println!("value of is r equals to 10?: {}", *r == 10);
}
```

But could you notice we never using * in function of iterating_mapping to access the content of an referenced object. Why? There is an implicit deference to object
of type with struct, when we want to access fields in an struct object, we use the operator ".", this operator will implicitly using the * to do the dereference.
Let's see an example here:
```r
struct Anime {
        name: String,
        bechdel_pass: bool,
    };
    let aria = Anime {
        name: "Aria: The Animation".to_string(),
        bechdel_pass: true,
    };
    let aria_ref = &aria;
    /*
    aria_ref.name is Equivalent to to *(aria_ref).name
    */
    println!("aria_ref.name is : {}", aria_ref.name);
    /*
    error, operator "." convert the left object to shared reference only
    */
    aria_ref.name = "Tom".to_string();
```

The "." operator will implicitly convert the object at the left to reference if the right side is a field of the object, or convert the object at the left to mutable
reference if the right side is a method call such as :
```r
let mut v = vec![1973, 1968];
    /*
    it is the same as (&mut v).sort(); or sort(&mut v); since the right of . is method call, then Rust compiler
    convert the left which is the vairable v into a mutable reference
    */
    v.sort();
```

reference can be reassigned as following:
```r
/*
    reference can be reassigend
    */
    let x = 10;
    let y = 20;
    let mut r = &x;
    println!("value of r:{}", r); //10
    r = &y;
    println!("value of r:{}", r); //20
```

Rust's comparison operator can "see through" the reference, that is if we take reference as wrapping the object in a box, then when you compareing two boxes.
Rust will take the object inside each box and compare them:
```rs
 let f = 10;
    let g = 10;

    let rf = &f;
    let rg = &g;
    /*
    reference to reference is just like wrapping box with another box, when compare with
    the outside box, rust will take the object out which is located in the most inner box
    */
    let rrf = &rf;
    let rrg = &rg;
    println!("is rrf <= rrg? :{}", rrf <= rrg); //true, the object in the most inner box is value 10
    println!("is rrf == rrg?:{}", rrf == rrg);

    /*
    if we want to check the given to boxes are the same, we need to use std::prt::eq
    */
    println!("is rf == rg: {}", std::ptr::eq(rf, rg)); //comparing the wrapping box
                                                       /*
                                                       we can't compare the inside box with the outside box
                                                       */
    //println!("is rg == rrg:{}", rg == rrg); //panic here
    /*
    we can take the insided box out from the outside box
    */
    println!("is rx == *rrx:{}", rg == *rrg);
```

The tricky thing is , We can convert a constant to a reference as following:
```r
  /*
    Rust will create two "unseen" varaible to hold the reference of constant 1 and 2
    that is &1 is Equivalent to let a = 1; let unsee_variable = &a;
    */
    println!("the result of two constant reference is :{}", &1 + &2);
```
As the code aboved, we have symbol & prefix with a constant number, Rust will implicitly create an unseen variabe and initialize its value to 1, then create another
unseen variable to hold the reference of the first unseen variable as the shown in the comment aboved.

There are several rules that restrict the use of reference, the first rule that is we can't reference to a local variable and take it out of its living scope, 
for example:
```rs
 /*
    not allowed to reference an object and take it out from its living scope
    */
    {
        let r;
        {
            //x can't live out the scope in the brackets
            let x = 1;
            r = &x;
        }// life of x ends here
        println!("r: {}", r); //error, the object being referenced is invalidated
    } life of r ends here
```
and the following one is ok, if the variable that is being referenced has life time longer thant the variable holding the reference:
```r
 /*
    life time of variable being referenced is longer thant the variable holding the
    reference
    */
    {
        /*
        x is the variable being referenced and r holds the reference to x,
        and the life time of x is longer thant r
        */
        let x;
        {
            let r = &x;
            println!("{}", r);
        } //life of r ends here
    } //life of x ends here
```
Since the variable being referenced has life time longer than the variable holding the reference, therefore the aboved code is legal.




