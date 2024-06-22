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

