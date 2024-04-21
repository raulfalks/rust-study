// Boolean = true, false
// Integer = 1, 2, 50, 99, -2
// Double / Float = 1.1, 5.5, 200.0001. 2.0
// Character = 'a', 'b', '7', '$'
// String = "Hello", "string", "this is a string"
// These are the basic data types 

// ===================================================================================

// VARIABLES
/* 

Immutable by default, but can be mutable, this helps to improve the speed
since the code doesn't need to think if that var is going to change, since
its immutable


*/

let two = 2; // Integer type
let hello = "hello"; // String
let j = 'j'; // Character because of single quotation
let my_half = 0.5; // Double or Float type 
let mut my_name = "Bill" // mut is for mutable, later it can be assigned another value, for example, Nick and also -> this is a String
let quit_program = false; // Boolean type
let your_half = my_half; // My_Half is another variable, so we are assigning a var to another var, which here means 0.5

/*

Variables can be assigned to any value, including other variables
And remember always, they are immutable by default here on Rust 

*/

// ===================================================================================

// FUNCTIONS

fn add(a: i32, b: i32) -> i32 {
    a + b
}

/*

fn = function
add = name  < we can not give names that already existing to the function, like fn for example
(a: i32, b: i32) = parameters, always opening and closing with parentesis ()
a: and b: are variables 
after the variable name we have the type of the parameter, in this case i32 is 
an 32bit integer and we should aways separate with a comma , 

to define what the data we want to be returned we need an arrow ->
which here means that we want a return of type i32

when we open curly brace and close, everything between is called the
function body, which here is a+b

*/

// (LET Y) it reads like: Let the variable Y be equal to the value of the function add, with the function arguments 3 and 0

let x = add(1, 1); // calling the function add, with the values of 1, and 1 ( which means = 2)
let y = add(3, 0); // these values are called function arguments, here the A parameter is 3, B is 0
let z = add(x, 1); // here we are getting the value of x that was set before, which is 2, so it becames 2+1 so the result to be returned here is 3

// So the value of the variable X is what RETURNS from the function above, and the function
// since we passed the add it means that we are getting the value from the fn add which is
// a+b, so the value of X is = add with arguments 1 to a and 1 to b 
// Which means that the var X is 2

// =====================================================================================================

// MACROS - println

/*

Macros are similar to functions 
Macros expand into additional code
println "Prints" (displays) information to the terminal
which is useful for debugging, just like console log

*/

let life = 42; 
println! ("Hello World"); // The exclamation means that this is a Macro, not a Function 
println! ("{:?}", life); // {:?} is a sequence of tokens indicates that we can take a external value and include, here we are passing life, so the value will be passed inside this
println! ("{:?} {:?}", life, life); // the : with the question marks means that we want a debugging print and not for the end user 
println! ("the meanin is {:?}", life); // here we pass a phrase with the :? inside {} which means the value after the comma , which here is life, which is a variable that means = 42

/*

We can do it like this too:

let life = 42;
println! ("{life:?);
println! ("{life}"); 

without needing to pass the value after the comma like on the examples above
the only difference between this two is the :? 
which means that the first one is for debugging
and the second one can be seen by the final user 

*/

// ========================================================================

// CONTROL FLOW - EXECUTION FLOW 

/*

Code executed line-by-line
Actions are performed & control flow may change
    This is possible with specific conditions such as:
        "if"
        "else"
        "else if"
     
*/

/*
This is a simple flow for example: 

let v = 1;
let r = 2; 
let g = 3; 
*/

// Now lets use this with contorl flow logic 

let v = 99; 
if v > 99 {
    println!("Big Number"); 
} else {
    println!("Small number");
}

// And now lets take a look into a nested if.. else

let r = 99;
if r > 99 {
    if r > 200 {
        println!("Huge number"); 
    } else {
        println!("Big number");
    }
 } else { 
    println!("Small number");
 }

 // Here we are checking if is greater than 99, if yes, we check if its greater than 200, if its huge number
 // If not we print big number 
 // If not greater than 99 we simply print small number 

 // Now lets do kind of the same thing but without nesting 

 let g = 99; 
 if g > 200 {  // We check 99 first and then 200 
    println!("Huge number");
else if g > 99 {
    println!("Big number");
} else {
    println!("Small number");
}
 }

 /*
Meanwhile this will not work, because we are first checking the lowest number, so if its 300
for example it will pass big number, we should aways check and be aware of the biggest value
which is 200, because on this case, once the thing is true, it simply jump out for the next
part of the code, so in the code below, if the value is 300, it will print big number
but it should be huge number, and the reason is not 
is because we are checking 99 before checking 200 

Remember: code executes line by line 

let g > 99 {
    println!("Big number");
} else if g > 200 {
    println!("Huge number"); 
} else {
    println!("Small number");
}
 
  */


// ==============================================================================

// REPETITION USING LOOPS

/*

Called "looping" or "iteration"

Multiple types of loops

- "loop" - infinite loop
- "while" - conditional loop


*/

// Starting with the infinite loop 

let mut tiro = 0;
loop { 
    if tiro == 5{
        break;
    }
    println!("{:?", tiro);
    tiro = tiro + 1; 
}

// Here we are checking if the MUTABLE variable, tiro, is == to 5, if not it will add +1 
// infinitely until the value is == 5, when it reaches 5 we stop ' BREAK '
// on this example it will loop 5 times since the var tiro starts with 0, but will change in
// every loop, because its mutable, if we didnt set that is mutable, it will run forever


// Now a While loop or Conditional loop

let mut noise = 0; 
while noise != 5 { // != means not equal, so in this example is true, which means that we are entering the next loop
    println!("{:?}", noise); // debug print because of the :? 
    noise = noise + 1; // so it will require 5 loops since its 0, until it reaches 5, and then finally BREAK
}

// Both type of loops can be stopped by using Break 

// ========================================================================================================

// TOOL INSTALATION 