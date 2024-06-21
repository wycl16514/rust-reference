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
        vec!["value1 of key1".to_string(), "value2 of key1".to_string()],
    );
    mapping.insert(
        "key2".to_string(),
        vec!["value1 of key2".to_string(), "value2 of key2".to_string()],
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


