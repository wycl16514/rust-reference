In previous section, we haven seen the only one onwer principle and find out that, when we ressign an object from one owner to a new owner,
the original owner becomes invalidated such as:

```r
let a = vec![1,2,3];
let b = a; //a becomes invalidated
```

As aboved code shows, if you try to access a after the code "let b = a;", then a panic will happen and the program crashes. This mechanism is
somehow inintutive and unconvinent. Since most of other programming languages will allow the continue using of of a. What can we do to enable
the normal way as other languages do and do not break the safty assurance Rust provides to us? 

That's how the reference and borrowing mechanism comes into being!
