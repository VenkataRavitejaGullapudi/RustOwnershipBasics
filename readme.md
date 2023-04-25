# Rust



## Memory Management:

- All programs have to manage the way they use a computer’s memory while running.
- Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs
- In other languages, the programmer must explicitly allocate and free the memory.
- Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

- Lets see about different memory management solutions.

  1. Garbage collection:
    <br/><b>Pros:</b><br/>
     - Error free* as programmer will not manage memory and it was taken care by garbage collector. But still garbage collector may had the chance of bugs.
     - Faster write time as we as a programmers dont need to worry about the allocating and deallocating memory
     
     <br/><b>Cons:</b><br/>
     - We are giving entire control to the garbage controller and programmers may not have control over memory.
     - Slower and unpredictable runtime performance because we will not manually manage before running the program and garbage collector can remove the memory anytime
     - Larger program size as garbage controller also need to be added along with our code.

  2. Manual Memory Management
     <br/><b>Pros:</b><br/>
        - Full control over the memory
        - Faster run time
        - Smaller program size.
      
     <br/><b>Cons:</b><br/>
        - Error prone as developer need to take care of memory
        - And also Slower write time
    
  3. Ownership model
    <br/>Ownership is a set of rules that govern how a Rust program manages memory.
    <br/><b>Pros:</b><br/>
        - Full control over the memory
        - Faster run time
        - Smaller program size.
        - Error free* but we can use some unsafe keyword to override.
     <br/><b>Cons:</b><br/>
        - Slower write time as we need to fight with borrow checker and other ownership rules initially but it is worth of fighting before running of code than debugging memory errors for hours.
    
     <br/>Here in rust we can over come most of the cons of garbage collection and manual memory management as rust manages the memory based on the ownership rules.  
   
### Ownership rules:
1. Each value in rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of the scope, the value will be dropped.

For example,
```
{ // s is not valid here, as it is not declared
  let s: String = String::from("hello"); 
  // s is valid from this point forward. By the way, here the s variable is stored in stack with a reference pointing to the memory address in heap of the value "hello" because all the dynamic values whose size can be increased are stored in heap as heap size can be increased in runtime.
} // the scope of s is now completed here and s i no longer valid. So after that rust will drop the variable.

