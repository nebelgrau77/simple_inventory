### Simple inventory management system

This program is a variation on the "Students' records" example from the "Embedded C for absolute beginners" Udemy course by FastBitLabs.
I wanted to port that C program to Rust, to learn Rust by doing :) 

At first I tried using an array, just like the C example. This required some more advanced features, like implementing the "new" trait for the struct,
and using lifetimes. It did work up to a certain point, but I ran into some problems with ownership of the str fields of the struct. 
That first version with its problems can be seen here: https://gist.github.com/nebelgrau77/e7af0e8e0c2185dc282f990c32ebc716

This program works as expected, but is not finished: it can be easily broken by inserting a text when a numeric value is expected.

__TO DO:__
* add correct error handling for user input (in case of an invalid value display an error and prompt the user again)
