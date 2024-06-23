We have seen in previous section that, reference just like a box, the value being referenced just like an object being putted into the box. Therefore we need to make sure the life span of the object inside the box
should longer than the wrapping box. This principle is not easy to enforce, becuase there are plenty of tricky situations that this principle would be break especially when we are using reference as arguments for
function call.

For example the following code would break the principle above:
```rs
static mut global_variable: &i32 = &128;
fn f(p: &i32) {
    unsafe {
        global_variable = p;
    }
}
```

The static keyword is somehow like a keyword to declare a global varaible, it is life span will last until the end of the programm, when we assign a reference to the global_variable, now the global_variable will
be the new box, and its life span can be sure that would not shorter than the object inside it, therefore there is risk that, when the time we open the box to get the object out, the object insided is already 
invalidated, that's why the code aboved will never get compile by Rust.

Since the life span of function input parameter is an important factor, it is better to have some way to indicate the life span of a function parameter, here comes the syntax sugar that can used to regulate the
life span of function paremeter, we can use the symbol ' then following a keyword "a" to indicate the life time. For example fn f<'a>(p :&'a i32), when we seeing the part of <'a>, it is telling Rust compiler:"
hei, I would setupt a life span condtion for the input parameters, and the life span is given by a", then in the argument list, we see p:&'a i32, this is telling Rust compiler that:"hei, I would require the 
paremeter p which has type of reference to i32 and it needs to have life span as indicated by a".

Then If we change the function of f as following，then the harsh compiler will let you go:
```rs
static WORTH_POINTING_AT: i32 = 1000;
fn f<'satic>(p: &'static i32) {
    unsafe {
        global_variable = p;
    }
}

fn main() {
    /*
    WORTH_POINTING_AT is a static global variable and
    its life time will to the end of the program which
    is meeting the time requirement of 'static
    */
    f(&WORTH_POINTING_AT);
}
```

Here is the first time we meet the unsafe keyword, it is a warning light, it is like telling the compiler that I know the piece of code here break the safty rules of Rust, and I fully know that if I run this 
piece of code, it may shoot on my feet at some time and I will not blame you for that.

The life time indicator is special feature of Rust, and it is not easy to understand, let's dive deep into it. As we have told many times, reference in Rust is just like putting some object
into a box, we need to make sure the object in the box can last longer than the box, otherwise when we open the box, we will find out the object inside the box is already dispear.Think about
corpse and coffin, If you open a coffin that is buried for 100 years, you don't expect you get the corpse with fresh as he just put into the coffin.

Since the life time of the object inside the box and the life time for the box are not easy to determinated, Rust rely on the coder to tell it, that's why it has the life time indicator.As we
have seen the life time indicator is a tick symbol following a string like 'static, you may think the string following the tick symbol should be some kind of keyword, but actually we can put
arbitrary string, as long as we can let the compiler can reason out the life time for the reference type, let's see an example:

```rs
fn construct_string(str_one: &mut String, str_two: &mut String) -> &String {
    str_one.push_str(" returned");
    str_one
}

fn main() {
    let str_one = "hello";
    let v: &str;
    {
        let str_two = "world";
        v = construct_string(str_one, str_two);
    } //str_two ends here
    println!("construct string:{}", v);
}
```
Could you see the trick here, the function construct_string return a reference of string literal, and this return value depend on the two input parameters as we have shown in the code. If the returned value
is depend on the first arguemnt, then the aboved code is legal, since v is constructed from str_one and when we access the v in the println!, it is still in valite state, but if the return value is depend on
the second parameter, then when we access v in the line of println!, then content of v is already invalidated because str_two is invalidated, therefore the code has ambiguity and the compiler will refuse to 
compile.

We need to tell the compiler the returned value of construct_string depends on which parameter, the first one or the second, that's place we need to utilize the life time indicator, and we can change the code
as following:
```rs
fn construct_string<'str_one_life_time, 'str_two_life_time>(
    str_one: &'str_one_life_time mut String,
    str_two: &'str_two_life_time mut String,
) -> &'str_one_life_time String {
    str_one.push_str(" returned");
    str_one
    //what if we return str_two?
    //str_two
}
```
Since the code aboved tell the compiler that the returned value of construct_string has the same life time as the first input argument, and when we calling construct_string, the first arguemnt is str_one,
and when we access v, the str_one is still in valid state, therefore the code can be compiled. If we add a new function as following:
```
fn construct_string2<'str_one_life_time, 'str_two_life_time>(
    str_one: &'str_one_life_time mut String,
    str_two: &'str_two_life_time mut String,
) -> &'str_two_life_time String {
    str_two.push_str(" returned");
    str_two
}

fn main() {
    let mut str_one = "hello".to_string();
    let v: &str;
    {
        let mut str_two = "world".to_string();
        //v = construct_string(str_one, str_two);
        v = construct_string2(&mut str_one, &mut str_two);
    } //str_two ends here
    println!("construct string:{}", v);
}
```

When we call construct_string2, the compiler would know the returned value has the same life time as str_two, but the life time of v is longer than str_two, that means when we access v, str_two may already 
invalidated, therefore the compiler will refuse to compile the code aboved.

