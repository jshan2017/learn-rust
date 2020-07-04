# CH03 - Common Programming Concepts

## 상수

- 상수는 `const` 키워드를 붙여 선언한다.
- 컴파일 타입에 값이 결정되어야 하며 타입을 명시해서 선언해야 한다. `C/C++`의 `static` 과 유사.

## Variable Shadowing

- mutable 여부와 상관없이 varaible shadowing이 가능하다.

## Data Types

- 타입추론이 불가능한 경우(undecidable) 변수의 타입을 명시해 주어야 한다.

```Rust
// parse는 어떠한 숫자 자료형으로 리턴해야할지 알려줘야 한다.
let guess: u32 = "42".parse().expect("Nan");
```

### Scalar Type

- Scalar Type은 단일 데이터를 표현하는 integer, floating point number, boolean, character 네가지 유형의 타입을 의미한다.

- Rust의 `char`형은 4바이트를 차지하여 유니코드를 사용 가능하다.

### Compound Type

- Compound Type은 복수의 데이터를 그룹화한 타입이다. 기본적으로 tuple과 array가 있다.
- tupple은 `()`를 이용해 선언하며 요소들은 자료형이 달라도 된다.
- 아래와 같이 tuple destructing을 하거나 index member로 요소에 접근 가능하다.

```Rust
let tup = (1, 2.3, 4);
let (x, y, z) = tup;
println("{} {} {}", tup.0, tup.1, tup.2);
```

- array는 C와 용도가 거의 동일하며 선언은 아래와 같이 한다.

```Rust
// 타입과 값을 모두 명시하는 경우
let a: [i32;5] = [0, 1, 2, 3, 4];
// 0 으로 모두 초기화 하는경우;
let a = [0; 5];
```

- Rust의 array는 길이를 초과하는 index에 접근할 경우 컴파일 에러가 발생한다.

## 함수

- `C/C++`와 달리 함수의 선언 순서는 중요하지 않다.
- parameter와 return value의 type을 명시해주어야 한다.

### Statements & Expression

- 평가되어 값이 리턴되면 Expression 그렇지 않으면 Statements다.
- 아래와 같은 사용이 가능

```Rust
let a: i32 = 0;
let b = {
  println!("here is code block");
  a+1 // no semicolon
}
println!("{}", b); // -> 4
```

### 값을 리턴하는 함수

- Rust에서 `return` 키워드를 사용하여 함수 중간에서 리턴할 수 있다.
- `return`이 없는경우 **마지막 Expression**의 결과를 리턴한다.

## 조건문

- if 문에는 condition에 `( )`가 필요없다.
- 조건 판별에는 오직 boolean expression을 사용해야한다.

```Rust
if 3 {} // err
```

- `Scala`와 유사하게 `if`문 또한 expression이므로 리턴값을 가진다. 이때 모든 case에 대한 return type이 같아야 한다.

### 반복문

- loop, for, while 키워드를 사용하여 반복문을 사용 할 수 있다.
- loop의 경우 break 뒤에 값을 붙이면 그 값으로 evaluate 된다.
