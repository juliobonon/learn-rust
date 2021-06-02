# learn-rust

Just a small project to learn rust lang.

## rustarm

In case you want to cross-compile rust applications for ARM architecture, please refer to [rustarm](https://github.com/juliobonon/rustarm) repository.

## rust-cleaner

This script cleans all the docker volumes and images and runs an apt-get clean command.

## Writing is a good path to learn

Variables naming convention:
 - Constants: uppercase snake_case, ex: `MAX_POINTS`

### Shadowing ###

You can use let keyword to re-declare the same variable with a different value;

```
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

You could use the `mut` keyword to create a mutable variable instead of shadowing but when you need a different type result, it's better to use shadowing.
