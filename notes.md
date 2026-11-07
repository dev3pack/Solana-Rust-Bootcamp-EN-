## What is usize? 

- an `unsigned integer` type used primarily for sizes, lengths, and indexing collections.

Think of it  as Rust's natural way for working with the size of data structures.

# Example 

```bash
let numbers = [1, 2, 3, 4, 5];

let length: usize = numbers.len();

```

- `numbers.len()` return a usize.

- usize: usigned integer for the `target architecture`. Generally used for collection lengths, indexes, and counting elements. 

- u64: used for balances, token amounts and lamports.

## What is deferencing and ownership?

# Example

```bash
let number: u64 = 50;

let reference = &number;

let actual_value = *reference;
```

`What does &number mean?`

- It means: Give me a reference to the value stored in `number`, rather than giving me the value itself. The reference lets you access the original value. 

Actual value contains `50`. 

## What is ownership?

- `Ownership` is a Rust's system for managing memory.

- Every value in Rust has an owner. The owner is resonsible for that value's memory. 

NB: There can only be one owner at a time. This works with strings. Integers, floats, boolean, characters, and tuples implement a Rust trait called `copy`.

- You can also use a value without tranferring ownership; this is where `borrowing` (&) comes in. Borrowing allows you to use a value without taking ownership of it. 

Two ways you can return an owned copy of a borrowed string (&str):

&str: borrowed string slice
Strin: Owned, growable string

1. .to_string(): creates an owned string 
2. .to_owned(): creates an owned copy

## Advanced Rust concepts 

# Iter methods and closures

Example: 

`flags &[bool]`

`flags.iter().filter(|&&value| value).count()`

- .iter(): iterate over slice
- .filter(): filter elements based on a condition --> Examines each element and decides whether to keep it. --> true: keep the itemc--> false: discard the item

What is `|&&value|` value ?

- A closure is an anonymous function

- Its general structure is: `|parameter| expression`

- In the above example: 

* &&value is the parameter pattern. 
* value is the expression returned by the closure. 

Why `two ampersands` ?  Here's why?

1. .iter() produces `&bool`
2. .filter() passes a referece to each iterator item into its closure.
3. Therefore the closure recieves &&bool.
4. &&value destructers those references and gives us the underlying bool. 