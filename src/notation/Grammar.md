# Rust Lang Grammar

러스트 언어의 어휘, 구문의 문법을 알아 보자.

| Notation        | Example                  | Meaning                                  |
|-----------------|--------------------------|------------------------------------------|
| CAPITAL         | KW_IF, INTEGER_LITERAL   | A token produced by the lexer            |
| ItalicCamelCase | LetStatement, Item       | A syntactical production                 |
| string          | x, while, *              | The exact character(s)                   |
| \x              | \n, \r, \t, \0           | The character represented by this escape |
| x?              | pub?                     | An optional item                         |
| x*              | OuterAttribute*          | 0 or more of x                           |
| x+              | MacroMatch+              | 1 or more of x                           |
| xa..b           | HEX_DIGIT1..6            | a to b repetitions of x                  |
| \|              | u8 \| u16, Block \| Item | Either one or another                    |
| [ ]             | [b B]                    | Any of the characters listed             |
| [ - ]           | [a-z]                    | Any of the characters in the range       |
| ~[ ]            | ~[b B]                   | Any characters, except those listed      |
| ~string         | ~\n, ~*/                 | Any characters, except this sequence     |
| ( )             | (, Parameter)?           | Groups items                             |

Rust 입력은 UTF-8로 인코딩된 일련의 유니코드 코드 포인트로 해석됩니다.

## Keywords

### strict

아래의 키워드들은 적절한 컨텍스트에서만 사용 가능하며 이름을 다른 곳에서 사용할 수는 없음.

- KW_AS : as
- KW_BREAK : break
- KW_CONST : const
- KW_CONTINUE : continue
- KW_CRATE : crate
- KW_ELSE : else
- KW_ENUM : enum
- KW_EXTERN : extern
- KW_FALSE : false
- KW_FN : fn
- KW_FOR : for
- KW_IF : if
- KW_IMPL : impl
- KW_IN : in
- KW_LET : let
- KW_LOOP : loop
- KW_MATCH : match
- KW_MOD : mod
- KW_MOVE : move
- KW_MUT : mut
- KW_PUB : pub
- KW_REF : ref
- KW_RETURN : return
- KW_SELFVALUE : self
- KW_SELFTYPE : Self
- KW_STATIC : static
- KW_STRUCT : struct
- KW_SUPER : super
- KW_TRAIT : trait
- KW_TRUE : true
- KW_TYPE : type
- KW_UNSAFE : unsafe
- KW_USE : use
- KW_WHERE : where
- KW_WHILE : while
- KW_ASYNC : async
- KW_AWAIT : await
- KW_DYN : dyn

### reserved

이 키워드들은 사용되지는 않았지만 향후 사용을 위해 예약된 키워드들입니다.
strict 키워드와 같이 적절하지 않게 사용할 수는 없음.

- KW_ABSTRACT : abstract
- KW_BECOME : become
- KW_BOX : box
- KW_DO : do
- KW_FINAL : final
- KW_MACRO : macro
- KW_OVERRIDE : override
- KW_PRIV : priv
- KW_TYPEOF : typeof
- KW_UNSIZED : unsized
- KW_VIRTUAL : virtual
- KW_YIELD : yield
- KW_TRY : try

### weak

이 키워드는 특정 컨텍스트에서만 의미를 갖기 때문에 변수나 메서드에서 사용해도 무방합니다.

KW_UNION : union
KW_STATICLIFETIME : 'static
KW_DYN : dyn

## Whitespace

- U+0009 (horizontal tab, '\t')
- U+000A (line feed, '\n')
- U+000B (vertical tab)
- U+000C (form feed)
- U+000D (carriage return, '\r')
- U+0020 (space, ' ')
- U+0085 (next line)
- U+200E (left-to-right mark)
- U+200F (right-to-left mark)
- U+2028 (line separator)
- U+2029 (paragraph separator)

## Tokens

### Literals

단일 토큰으로 구성된 표현식, 참조하는 대신 평가 대상 값을 직접적으로 나타내며 상수 표현식의 한 형태이고 주로 컴파일 타임에 평가됨.

#### Characters and strings

|                 |   Example   |   # sets   |  Characters |         Escapes         |
|:---------------:|:-----------:|:----------:|:-----------:|:-----------------------:|
| Character       | 'H'         | 0          | All Unicode | Quote & ASCII & Unicode |
| String          | "hello"     | 0          | All Unicode | Quote & ASCII & Unicode |
| Raw string      | r#"hello"#  | 0 or more* | All Unicode | N/A                     |
| Byte            | b'H'        | 0          | All ASCII   | Quote & Byte            |
| Byte string     | b"hello"    | 0          | All ASCII   | Quote & Byte            |
| Raw byte string | br#"hello"# | 0 or more* | All ASCII   | N/A                     |

#### ASCII escapes

|      | Name                                                |
|:----:|-----------------------------------------------------|
| \x41 | 7-bit character code (exactly 2 digits, up to 0x7F) |
| \n   | Newline                                             |
| \r   | Carriage return                                     |
| \t   | Tab                                                 |
| \\   | Backslash                                           |
| \0   | Null                                                |

#### Byte escapes

|      | Name                                    |
|:----:|-----------------------------------------|
| \x7F | 8-bit character code (exactly 2 digits) |
| \n   | Newline                                 |
| \r   | Carriage return                         |
| \t   | Tab                                     |
| \\   | Backslash                               |
| \0   | Null                                    |

#### Unicode escapes

|          | Name                                           |
|:--------:|------------------------------------------------|
| \u{7FFF} | 24-bit Unicode character code (up to 6 digits) |

#### Quote escapes

|    | Name         |
|:--:|--------------|
| \' | Single quote |
| \" | Double quote |

#### Numbers

| Number literals* |   Example   | Exponentiation |         Suffixes        |
|:----------------:|:-----------:|:--------------:|:-----------------------:|
| Decimal integer  | 98_222      | N/A            | Integer suffixes        |
| Hex integer      | 0xff        | N/A            | Integer suffixes        |
| Octal integer    | 0o77        | N/A            | Integer suffixes        |
| Binary integer   | 0b1111_0000 | N/A            | Integer suffixes        |
| Floating-point   | 123.0E+77   | Optional       | Floating-point suffixes |

### Punctuation

| Symbol |    Name    |                                                             Usage                                                            |
|:------:|:----------:|:----------------------------------------------------------------------------------------------------------------------------:|
| +      | Plus       | Addition, Trait Bounds, Macro Kleene Matcher                                                                                 |
| -      | Minus      | Subtraction, Negation                                                                                                        |
| *      | Star       | Multiplication, Dereference, Raw Pointers, Macro Kleene Matcher, Use wildcards                                               |
| /      | Slash      | Division                                                                                                                     |
| %      | Percent    | Remainder                                                                                                                    |
| ^      | Caret      | Bitwise and Logical XOR                                                                                                      |
| !      | Not        | Bitwise and Logical NOT, Macro Calls, Inner Attributes, Never Type, Negative impls                                           |
| &      | And        | Bitwise and Logical AND, Borrow, References, Reference patterns                                                              |
| \|     | Or         | Bitwise and Logical OR, Closures, Patterns in match, if let, and while let                                                   |
| &&     | AndAnd     | Lazy AND, Borrow, References, Reference patterns                                                                             |
| \|\|   | OrOr       | Lazy OR, Closures                                                                                                            |
| <<     | Shl        | Shift Left, Nested Generics                                                                                                  |
| >>     | Shr        | Shift Right, Nested Generics                                                                                                 |
| +=     | PlusEq     | Addition assignment                                                                                                          |
| -=     | MinusEq    | Subtraction assignment                                                                                                       |
| *=     | StarEq     | Multiplication assignment                                                                                                    |
| /=     | SlashEq    | Division assignment                                                                                                          |
| %=     | PercentEq  | Remainder assignment                                                                                                         |
| ^=     | CaretEq    | Bitwise XOR assignment                                                                                                       |
| &=     | AndEq      | Bitwise And assignment                                                                                                       |
| \|=    | OrEq       | Bitwise Or assignment                                                                                                        |
| <<=    | ShlEq      | Shift Left assignment                                                                                                        |
| >>=    | ShrEq      | Shift Right assignment, Nested Generics                                                                                      |
| =      | Eq         | Assignment, Attributes, Various type definitions                                                                             |
| ==     | EqEq       | Equal                                                                                                                        |
| !=     | Ne         | Not Equal                                                                                                                    |
| >      | Gt         | Greater than, Generics, Paths                                                                                                |
| <      | Lt         | Less than, Generics, Paths                                                                                                   |
| >=     | Ge         | Greater than or equal to, Generics                                                                                           |
| <=     | Le         | Less than or equal to                                                                                                        |
| @      | At         | Subpattern binding                                                                                                           |
| _      | Underscore | Wildcard patterns, Inferred types, Unnamed items in constants, extern crates, use declarations, and destructuring assignment |
| .      | Dot        | Field access, Tuple index                                                                                                    |
| ..     | DotDot     | Range, Struct expressions, Patterns, Range Patterns                                                                          |
| ...    | DotDotDot  | Variadic functions, Range patterns                                                                                           |
| ..=    | DotDotEq   | Inclusive Range, Range patterns                                                                                              |
| ,      | Comma      | Various separators                                                                                                           |
| ;      | Semi       | Terminator for various items and statements, Array types                                                                     |
| :      | Colon      | Various separators                                                                                                           |
| ::     | PathSep    | Path separator                                                                                                               |
| ->     | RArrow     | Function return type, Closure return type, Function pointer type                                                             |
| =>     | FatArrow   | Match arms, Macros                                                                                                           |
| #      | Pound      | Attributes                                                                                                                   |
| $      | Dollar     | Macros                                                                                                                       |
| ?      | Question   | Question mark operator, Questionably sized, Macro Kleene Matcher                                                             |

