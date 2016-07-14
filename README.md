# int_traits

This crate provides extra traits for working with integers. The traits
provided are those provided by default for floating point values, but
extended to provide optimized integer routines.

This is mainly intended to avoid extensive casting when we want extended
operations for integers.

```
let a = 50_usize;
let a_sqrt = (a as f32).sqrt() as usize;
```

becomes

```
let a = 50_usize;
let a_sqrt = a.sqrt();
```

## Coverage

We provide the following functions

```
sqrt
cbrt
log
log2
log10
```

for the following types

```
i8
i16
i32
i64
u8
u16
u32
u64
isize
usize
```

If a negative number is passed to one of the functions, we panic on runtime.

## Examples

```
extern crate int_traits;

use int_traits::IntTraits;

fn main() {
    let a = 50_usize;

    println!("{}", a.sqrt());
}
```

## Design Questions

 - It is not known whether we want to return floating point values or perform
   the integer rounding explicitly.

   This could likely be split up into seperate functions `isqrt` and `sqrt` or
   two different trait implementations could be provided.

 - We do not perform any optimized functions currently. For example, since we
   do not need to calculate the fractional part of a square root we can
   make some potential shortcuts to improve this.
