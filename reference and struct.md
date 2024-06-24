We have seen the life time trap in reference and function, such trap can happen on fields of struct. Let's see an example first:
```rs
 struct S {
        r: &i32,
    }

    let s;
    {
        let x = 10;
        s = S { r: &x };
    } //x ends here
      //the life span of reference which is the field r in s is longer than x
    println!("r in s: {}", s.r);
```
When compile the aboved code, the compiler will stop at the line " r: &i32," and it gives you the error message as following:

![截屏2024-06-24 18 01 51](https://github.com/wycl16514/rust-reference/assets/7506958/e2e6d6af-4c70-424a-ad63-c5cd0e194e1e)

The rule is that, if we want to have reference as fields in a struct, we need to add its life span indicator when defining the field. Following the tip given by the compiler our change is 
adding life span indicator as following:
```rs
struct S {
        r: &'static i32,
    }
```
Compile again and this time we get another error:
![截屏2024-06-24 18 04 43](https://github.com/wycl16514/rust-reference/assets/7506958/58e1a689-df89-4db1-a443-d73eefecfd58)

It is not difficult to find out that the trap of "life span of the box is longer than the object inside" is happenning again. Even though we indicate the life span of the "box" which is the field r in s
is &'static which is as long as the program itself, but the object putting inside the box which is x will end when the code visit the right bracket.

The code aboved has a problem, the life span of field r is as long as the program, but field r only lives as long as its wrapping struct s lives which means adding the life span to r forces its wrapping struct
s to have the same life span as r, this just like "the boss following the commands from the employee". It is better that we should enable "employees follow the commands of boss", then we can do the folloing:
```rs
 struct S<'a> {
        r: &'a i32,
    }
```
The aboved code means the struct S has a life span as indicated by "'a", therefore the life span of field r is also has the life span as 'a which is the same as its wrapping struct. The same principle also apply
to struct inside another struct such as:
```r
 struct D {
        s: S,
    }
```
If you compile the code aboved, we will get the following error:

![截屏2024-06-24 18 29 50](https://github.com/wycl16514/rust-reference/assets/7506958/82f54289-156a-40e0-8bf6-c2bac87ed225)

When we define a field in a struct as type of struct, it is just like we define a field in struct as type reference, we need to add the life span indicator as :
```rs
 struct D<'a> {
        s: S<'a>,
    }
```
This will enforce the life span of field s of D  as type of struct S will not longer than the wrapping struct D. Here we setup an implicit inequality that is:

life span of field r <= life span of struct S <= life span of Struct D

There are some trick situation we need to think about, for example we deifine a struct and set its fields to the life span as indicate by the struct:
```rs
struct S<'a> {
        x: &'a i32,
        y: &'a i32,
    }
```
The life time indicator in aboved code shows the field x and y has the same life span. This will cause problem if we assign reference objects to x and y but each of 
which has different lief span such as :
```rs
 let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        } //s ends here and so does its fiels of  x and y
          //even though x is removed as s but the object inside x still valid
    } // y ends here
    println!("{}", r);
```
if you compile the code aboved, the compiler will complain as following:

![截屏2024-06-24 18 29 50](https://github.com/wycl16514/rust-reference/assets/7506958/c2bad0bf-96f1-4634-a1fd-f1f0ce894d82)

The compiler says y dose not live long enough, but the life span of y is certainly longer than struct s and therefore longer than the field y in struct s. The 
reasoning is that, since field x and y has the same life span as indicated in the definition,  if field x and y has different life span in code, the compiler need
to find a comman ground that satisfy both x and y.

And the assigment "r = s.x" forces the life span of x should be the same as r, but the intialization of "let s = S { x: &x, y: &y };" requires the y field in the 
struct has the same life span as y aboved, there is not "common ground" that between these two, therefore the compiler gives out the complain.The solution is we give
x and y different life span as following:
```rs
struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }
```
Now since x and y has different life span, then the conflict aboved can be solved. By default we don't need to indicate life span for each reference field in struct,
Rust compiler simply assign different life span to each reference field in the struct, for example the struct defined above is the same as the following:
```rs
 struct S{
        x: &i32,
        y: &i32
    }
```
