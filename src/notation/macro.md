# Macros

rust에서 function, syntax 는 macro라는 사용자 정의로 확장할 수 있다.
이름이 지정되고 일관된 구문인 `some_extension!(...)`을 통해 호출됩니다.

## macro_rules

macro 호출은 아래와 같이 `macro_rules`를 통해 정의하고 호출합니다.

```rust
/ Used as an expression.
let x = vec![1,2,3];

// Used as a statement.
println!("Hello!");

// Used in a pattern.
macro_rules! pat {
    ($i:ident) => (Some($i))
}

if let pat!(x) = Some(1) {
    assert_eq!(x, 1);
}

// Used in a type.
macro_rules! Tuple {
    { $A:ty, $B:ty } => { ($A, $B) };
}

type N2 = Tuple!(i32, i32);

// Used as an item.
thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

// Used as an associated item.
macro_rules! const_maker {
    ($t:ty, $v:tt) => { const CONST: $t = $v; };
}
trait T {
    const_maker!{i32, 7}
}

// Macro calls within macros.
macro_rules! example {
    () => { println!("Macro call in a macro!") };
}
// Outer macro `example` is expanded, then inner macro `println` is expanded.
example!();

use lazy_static::lazy_static; // Path-based import.

macro_rules! lazy_static { // Textual definition.
    (lazy) => {};
}

lazy_static!{lazy} // Textual lookup finds our macro first.
self::lazy_static!{} // Path-based lookup ignores our macro, finds imported one.
```

### macro_use

모듈에 적용 시 모듈이 닫힐 때 macro 범위가 끝나지 않도록 하는데 사용.

```rust
#[macro_use]
mod inner {
    macro_rules! m {
        () => {};
    }
}

m!();

#[macro_use(lazy_static)] // Or #[macro_use] to import all macros.
extern crate lazy_static;

lazy_static!{}
// self::lazy_static!{} // Error: lazy_static is not defined in `self`
```
