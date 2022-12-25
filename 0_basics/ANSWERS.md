# Step 0: Become familiar with Rust basics
==========================================

## Answers:
- What memory model [Rust] has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?
    - Rust does not yet have a defined memory model. Rust just inherits the memory model for atomics from C++20. A Rust program's memory consists of a static set of items and a heap. Immutable portions of the heap may be safely shared between threads, mutable portions may not be safely shared, but several mechanisms for effectively-safe sharing of mutable values, built on unsafe code but enforcing a safe locking discipline, exist in the standard library. Allocations in the stack consist of variables, and allocations in the heap consist of boxes.
    - Both single- and multithreaded runtimes are available in Rust, which have different strengths and weaknesses.
    - Rust can use two concurrency programming models: threads and async.
    <br><br/>
- What runtime [Rust] has? Does it use a GC (garbage collector)?
    - No built-in runtime is provided by Rust. Instead, runtimes are provided by community maintained crates. Rust does not use a garbage collector.
    <br><br/>
- What statically typing means? What is a benefit of using it?
    - A statically-typed language is a language where variable types are known at compile time. In most of statically-typed languages, types must be expressly indicated by the programmer; in other cases, type inference allows the programmer to not indicate their variable types.
    - It allows statically (without running the program) detecting many programming errors quickly, reliably and automatically. This helps reduce the number of bugs and reduces the time spent on debugging. Type declarations serve as automatically-checked documentation. They make programs easier to understand and maintain. Static typing may improve runtime efficiency.
    <br><br/>
- What are generics and parametric polymorphism? Which problems do they solve?
    - Generics in Rust is a method of generalizing data types. As generics can have multiple forms over given parameters and can be applied to methods, functions, structures, traits, etc they are often referred to as parametric polymorphism in type theory, which means that they are types or functions that have multiple forms (‘poly’ is multiple, ‘morph’ is form) over a given parameter (‘parametric’).
    - They are extremely useful for reducing code duplication in many ways.
    <br><br/>
- What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? What is a marker trait?
    - A trait in Rust is a group of methods that are defined for a particular type.
    - A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.
    - Traits are similar to a feature often called interfaces in other languages, although with some differences.
    - Auto traits permit automatically implementing a trait for types which contain fields implementing the trait. That is, they are fairly close to an automatic derive. They describe properties of types rather than behaviors. The examples of auto traits are : Send, Sync, Unpin, UnwindSafe, and RefUnwindSafe.
    - Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.
    - Rust has a handful of "markers" that classify types: Send, Sync, Copy, Sized. These markers are just traits with empty bodies, which can then be used in both generics and trait objects.
    <br><br/>
- What are static and dynamic dispatches? Which should I use, and when?
    - Methods are executed in two ways:

        static dispatch - When the instance type is known, we have direct knowledge of what function to call.  

        dynamic dispatch - When an instance type is not known, we must find out some way of calling the correct function.  

        Trait types &dyn MyTrait give us the ability to work with instances of objects indirectly using dynamic dispatch.  

        When dynamic dispatch is used, Rust will encourage us to put dyn before your trait type so people are aware.  
    - “When you’re given the choice between static and dynamic dispatch, there is rarely a clear-cut right answer. Broadly speaking, though, you’ll want to use static dispatch in your libraries and dynamic dispatch in your binaries. In a library, you want to allow your users to decide what kind of dispatch is best for them, since you don’t know what their needs are. If you use dynamic dispatch, they’re forced to do the same, whereas if you use static dispatch, they can choose whether to use dynamic dispatch or not.” 
    <br><br/>
- What is a crate and what is a module in Rust? How do they differ? How are the used?
    - Crates can produce an executable or a library, depending on the project. Each crate has an implicit root module that contains the code for that crate. You can then define a tree of sub-modules under that root module. Modules allow you to partition your code within the crate itself, hierarchically splitting code in logical units (modules), and manage visibility (public/private) between them.  
    So a module is a collection of items and it can contain:
        <br><br/>
        Functions  
        Types (structs, enums, unions, type aliases)  
        Traits  
        Impl blocks  
        Macros  
        Constants and statics  
        Extern blocks  
        Extern crates  
        Imports  
        Modules  
        Associated items
        <br><br/>
- What are move semantics? What are borrowing rules? What is the benefit of using them?
    - In Rust, all types are mutable, and all move operations are equivalent to a bit copy of the original data at the new location. Move converts any captured variables by reference or mutable reference to captured variables by value.

    When threads are involved, the move is commonly used.

    Rust removal processes are also destructive. For example, after leaving a variable, the variable is no longer usable in code.

    In Rust, a move is consistently implemented by copying the data in the object itself and not destroying the original object. It is never implemented by copying the object’s managed resources or executing custom code.

    Rust makes an exception for types that do not require move semantics. Instead, the value contains all the necessary information to represent it, and no heap allocations or resources are managed by the value, such as i32.

    These types implement the special Copy trait because copying is inexpensive and the default method for passing to functions or managing assignments:

    fn foo(bar: i32) {
        // Implementation
    }

    let var: i32 = 9;
    foo(var); // copy
    foo(var); // copy
    foo(var); // copy
    The default function call uses move semantics for types other than Copy, such as String. In Rust, when a variable is moved, its lifetime ends prematurely.

    At compile time, the move replaces the destructor call at the end of the block, so it is an error to write the following code for String:

    fn foo(bar: String) {
        // Implementation
    }

    let var: String = "Hello".to_string();
    foo(var); // Move
    foo(var); // Compile-Time Error
    foo(var); // Compile-Time Error
    Clone rust requires implementations, but it is identical for all moves: copy the memory in the value and do not call the original value’s destructor.

    And in Rust, with this exact implementation, all types are movable; non-movable types do not exist (though non-movable values do). The bytes must encode information about the resource that the value is managing, such as a pointer, in the new location just as effectively as they did in the previous location.

    Rust has a mechanism known as pinning that indicates in the type system that a specific value will never move again, which can be used to implement self-referential values and is utilized in async.

    Example:

    fn destroy_box(z: Box<i32>) {
        println!("The box that contains {} is destroyed", z);
    }

    fn main() {
        let a = 5u32;
        let b = a;
        println!("a is {}, and y is {}", a, b);

        let m = Box::new(5i32);

        println!("a contains: {}", m);
        let n = m;
        destroy_box(n);
    }  
    - RULES FOR VARIABLES
    Each value has a variable which is it’s owner.
    There can only be one owner of a value at a time.
    When the owner goes out of scope, the value is dropped.  
    RULES OF BORROWING
    Unlimited borrows for read-only borrows: let a = &x
    For a read only borrow, the original data is immutable for the duration of the borrow
    You can only have a single borrow at a time for mutable borrows: let a = &mut x
    - Using borrowing rules Rust prevents data races at compile time: we’ll get errors if we break the rules.
    <br><br/>
- What is immutability? What is the benefit of using it?
    - In Rust, variables are immutable by default, which means that their values cannot be changed. If a new value is assigned to a variable that was declared with the let keyword, the program won’t compile.
    - To make the code easier to read. Code is read many more times than it’s written. The easiest code to read is the code that requires keeping the least state in mind: i.e. the code with the smallest “working set” at any point. Reducing the number of mutable variables in a chunk makes its understanding infinitely easier because you know that once a variable has been given a value, it remains that way. You don’t need to carefully look for places where the value might be mutated within the function nor in any of the called functions. 
    <br><br/> 
    To prevent mistakes when using APIs. Projects grow and it’s hard to track all the interactions between components. You’d pass everything by value, in which case immutability wouldn’t give you anything, but in practice you end up passing lots of values by reference to avoid copy costs. In those cases, it’s very useful to know that calling a specific function won’t mutate its arguments—which, again, reduces the working set you need to keep in mind.  
    <br><br/>
    To come up with better designs. Combining the previous two items, thinking of immutability has strong implications in the design of your types and interfaces. The reason I routinely highlight the lack of immutability during code reviews is because, inevitably, adding these qualifiers causes the author to have to redesign the code with immutability in mind. The results are cleaner code that is easier to read and easier to test. See my previous post Readability: Don’t modify variables for some examples.  
    <br><br/>
    To help with concurrency. Immutable data structures are trivial to parallelize. Code that deals with immutable data could can be made parallel with ease and, in theory, this could be done automatically.
    <br><br/>
- What is cloning? What is copying? How do they compare?
    - Differs from Copy in that Copy is implicit and extremely inexpensive, while Clone is always explicit and may or may not be expensive. In order to enforce these characteristics, Rust does not allow you to reimplement Copy, but you may reimplement Clone and run arbitrary code.  
    Since Clone is more general than Copy, you can automatically make anything Copy be Clone as well.
    <br><br/>
- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
    - Resource Acquisition Is Initialization or RAII, is a C++ programming technique[1][2] which binds the life cycle of a resource that must be acquired before use (allocated heap memory, thread of execution, open socket, open file, locked mutex, disk space, database connection—anything that exists in limited supply) to the lifetime of an object. 
    <br><br/> 
    RAII guarantees that the resource is available to any function that may access the object (resource availability is a class invariant, eliminating redundant runtime tests). It also guarantees that all resources are released when the lifetime of their controlling object ends, in reverse order of acquisition. Likewise, if resource acquisition fails (the constructor exits with an exception), all resources acquired by every fully-constructed member and base subobject are released in reverse order of initialization. This leverages the core language features (object lifetime, scope exit, order of initialization and stack unwinding) to eliminate resource leaks and guarantee exception safety. Another name for this technique is Scope-Bound Resource Management (SBRM), after the basic use case where the lifetime of an RAII object ends due to scope exit.
    - Variables in Rust do more than just hold data in the stack: they also own resources, e.g. Box<T> owns memory in the heap. Rust enforces RAII, so whenever an object goes out of scope, its destructor is called and its owned resources are freed. The notion of a destructor in Rust is provided through the Drop trait. This trait is not required to be implemented for every type, only implement it for your type if you require its own destructor logic.
    <br><br/>
- What is an iterator? What is a collection? How do they differ? How are they used?
    - An iterator is something that we can call the .next() method on repeatedly, and it gives us a sequence of things. In Rust The Iterator trait is used to implement iterators over collections such as arrays.
    <br><br/>
    The trait requires only a method to be defined for the next element, which may be manually defined in an impl block or automatically defined (as in arrays and ranges).
    <br><br/>
    As a point of convenience for common situations, the for construct turns some collections into iterators using the .into_iter() method.
    - Collections are data structures that are provided by Rust’s standard library. These structures store information in sequences or groups. While most other Rust data types contain only one particular value, collections may contain multiple values at a time.
    <br><br/>
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
    - The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:  
    Custom #[derive] macros that specify code added with the derive attribute used on structs and enums  
    Attribute-like macros that define custom attributes usable on any item  
    Function-like macros that look like function calls but operate on the tokens specified as their argument  
    Macros allow us to abstract at a syntactic level. A macro invocation is shorthand for an "expanded" syntactic form. This expansion happens early in compilation, before any static checking. As a result, macros can capture many patterns of code reuse that Rust’s core abstractions cannot.  
    Simply saying Macros allows us to invent your own syntax and write code that writes more code.
    - Declarative macros are defined using macro_rules! and they're simple (declarative) transformations, so they're mostly convenient helpers. Procedural macros are full blown Rust programs manipulating the token stream. Procedural macros are a lot more powerful because you have an entire programming language available. 
- How code is tested in [Rust]? Where should you put tests and why?
    - Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform these three actions:  
    Set up any needed data or state.  
    Run the code you want to test.  
    Assert the results are what you expect.  
    <br><br/>
    The Rust community thinks about tests in terms of two main categories: unit tests and integration tests. Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.
    - You’ll put unit tests in the src directory in each file with the code that they’re testing. The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).  
    We create a tests directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory. We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate.
    <br><br/>
- Why [Rust] has `&str` and `String` types? How do they differ? When should you use them?
    - There are two types of strings in Rust: String and &str.  
    <br><br/>
    A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.  
    <br><br/>
    &str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>.  
    Maybe we should refer &str as a function parameter or if you want a read-only view of a string; String when you want to own and mutate a string.
    <br><br/>
    String is an owned type that needs to be allocated. It has dynamic size and hence its size is unknown at compile time, since the capacity of the internal array can change at any time.  
    &str consists just of a pointer into memory, its size is known at compile time. It’s not an owned type, but rather a read-only reference to a string slice. the memory the &str points to can not be changed while the &str is in existence, even by the owner of the str. &str is a great candidate for function arguments, if mutability and ownership are not required. Its best used when a slice (view) of a string is needed, which does not need to be changed.
    The &String type is simply a reference to a String. This means that this isn’t an owned type and its size is known at compile-time, since it’s only a pointer to an actual String.  
    &String can be coerced to &str, but not the other way around, it usually makes sense to use &str as a parameter type, if we just need a read-only view of a string.
    We can't use str because the size for values of type str cannot be known at compilation time. And function arguments must have a statically known size, borrowed types always have a known size: &.
    <br><br/>
- What are lifetimes? Which problems do they solve? Which benefits do they give?
    - A lifetime is a construct the compiler (or more specifically, its borrow checker) uses to ensure all borrows are valid. Specifically, a variable's lifetime begins when it is created and ends when it is destroyed. Other words lifetimes are named regions of code that a reference must be valid for. Those regions may be fairly complex, as they correspond to paths of execution in the program. There may even be holes in these paths of execution, as it's possible to invalidate a reference as long as it's reinitialized before it's used again. Types which contain references (or pretend to) may also be tagged with lifetimes so that Rust can prevent them from being invalidated as well.
- Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance?
    - Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations.  
    <br><br/>
    Using this definition, Rust is object-oriented: structs and enums have data, and impl blocks provide methods on structs and enums. Even though structs and enums with methods aren’t called objects, they provide the same functionality, according to the Gang of Four’s definition of objects.
    - It is possible to use SOLID/GRASP?
    - No it does not have an inheritance.



After you're done notify your lead in an appropriate PR (pull request), and he will exam what you have learned.

_Additional_ articles, which may help to understand the above topic better:
- [Chris Morgan: Rust ownership, the hard way][1]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]




[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://www.rust-lang.org/faq.html

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
