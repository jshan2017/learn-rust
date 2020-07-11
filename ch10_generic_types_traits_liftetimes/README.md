# CH10 - Generic Types, Traits, Lifetimes

## Generic

Rust는 제네릭 타입을 지원한다 사용법은 아래 예시와 같다.

```Rust
// defining function
fn sort<T>(list: &[T]) -> T {
    // .. do something
}

// defining structure
struct Component<T> {
    id: u32,
    light_mode_states: T,
    dark_mode_states: T
}

// implementing methods
impl<T> Component<T> {
    fn setState(&self) -> &T {
        // .. do something
    }
}
```

메서드 정의를 특정 타입에 대해서만 허용하고 싶다면 아래와 같이 할수 있다.

```Rust
impl Component<ScreenStates>{
    fn flash(&self) -> ScreenStates {
        // .. do something
    }
}
```

## 단일화

제네릭을 사용하는 코드는 컴파일 과정에서 단일화(Monoporphazation) 과정을 거친다. 단일화란 제네릭 코드를 구체적인 타입으로 변환하는 과정이다.
이를테면 아래와 같다.

```Rust
// before monoporphazation
let a = Option<i32>::Some(5);
let b = Option<f64>::Some(3.9);

// after monoporphazation

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

let a = Option_i32::Some(5);
let b = Option_f64::Some(3.9);

```

이러한 과정을 통해 런타임 성능 저하를 막을수 있다.

## Trait

`trait`은 다른 언어들의 interface와 유사한 역할을 하지만 사용법에 다소 차이가 있다.
아래와 같이 선언한다

```Rust
trait Skills {
    fn jump(&self);
    fn run(&self);
}
```

아래와 같이 타입에 `trait`를 구현하여 사용한다.

```Rust
impl Skills for Ant {
    fn jump(&self) {
        println!("ant jump!");
    }
}
```

`trait`을 정의할때부터 기본 동작을 구현해 줄 수 있다.

```Rust
trait Skills {
    fn jump(&self) {
        println!("ant jump!");
    }
}

impl Skills for Ant {} // empty block for using default actions
```

Rust에서는 이러한 방식을 통해 인스턴스의 액션을 정의해줄 수 있다. `trait`은 이름처럼 인스턴스에 어떤 속성을 더해줄지 정의해주는 역할을 한다.

## trait 매개변수

특정 `trait`을 구현하는 타입에 대해서만 함수를 정의하려면 `trait` 매개변수를 사용하면 된다.

```Rust
fn learn(target: impl Skills){
    println!("learn - {}", target.jump());
}
```

위 코드는 Skills를 구현한 타입을 매개변수로 사용하는 함수이다.
`trait` 경계 문법(Trait Bound Syntax)을 사용하여 아래와 같이 작성할 수도 있다.

```Rust
fn learn<T: Skills>(target: T) {
    // ..
}
```

`trait` 경계 문법은 parameter가 여러개일때 타입을 일치시키기 위해 사용할 수 있다.

```Rust
fn learn2<T: Skills>(target1: T, target2L T) {
    // ..
}
```

하나 이상의 `trait`에 대해 함수 선언을 재사용하기 위해 아래와 같이 `+` 기호를 사용한다.

```Rust

fn learn3(target: impl Skills + Tips) {
    // ..
}

// or

fn learn3<T: Skills+Tips>(target: T) {
    // ..
}
```

`trait` 경계를 사용하면 함수 시그니처의 가독성을 해칠 수 있는데 이를 개선하기 위해 `where`를 사용할 수 있다.

```Rust
fn learn3<T>(target: T)
    where T: Skills + Tips {
    // ..
}
```

## 수명

Rust에서는 C와 같이 수동으로 Heap 메모리를 해제하지 않고 컴파일 타임에 변수들의 수명을 검사하고 이를 통해 해제 시점이 결정된다.
수명이란 변수를 참조가능한 scope를 말한다

## 수명 관련 에러

아래 코드는 에러가 발생한다

```Rust
fn longest(x:&str,y:&str) -> &str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

이는 리턴값이 a일수도 있고 b일수도 있는데 각각의 수명이 다를수 있고 따라서 컴파일타임에 리턴값의 수명을 확정할 수 없기 때문이다.
리턴값의 타입추론이 불가능한것과 마찬가지이다.

## 수명 annotation

수명 확정이 불가능한 상황에는 수명 매개변수를 이용해 수명을 명시 한다.
수명 매개변수는 타입 추론에 비유하자면 type variable(T)와 같은 역할을 한다.
아래와 같이 사용한다.

```Rust
fn longest(x:&'a str,y:&'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

위 코드는 x, y, 리턴값이 a라는 수명의 scope 안에서 참조가 가능하다는 것을 나타낸다.

## 구조체의 수명 annotation

구조체에 필드의 수명을 binding 해주기 위해 annotatoin을 아래와 같이 사용한다

```Rust
struct Ant<'a>{
    name:String'a;
}
```

이렇게 하면 Ant 인스턴스의 수명은 name 필드의 수명에 의해 결정된다.

## 수명 생략 규칙

Rust에는 수명 annotation을 생략할수 있게 하는 규칙이 세가지 존재한다.

1. 각각의 parameter는 각각의 수명 매개변수를 가진다.
2. parameter의 수명 매개변수가 하나이면 같은 수명 매개변수를 return 값에도 적용한다.
3. parameter의 수명 매개변수가가 하나 이상이고 함수가 메소드로 선언되어 self parameter를 갖고 있으면 self의 수명을 return 값에도 적용한다

아래 예시들에 수명 생략 규칙을 적용해보자.

> 예시 1

```Rust
fn foo(bar: &str) -> &str
```

1번 규칙을 적용하면 아래와 같이 된다.

```Rust
fn foo(bar: &'a str) -> &str
```

그 다음에 2번 규칙을 적용하면 다음과 같다.

```Rust
fn foo(bar: &'a str) -> &'a str
```

함수의 입력과 출력에 대해 모두 수명 매개변수가 확정되었으므로 annotation을 생략할 수 있다.

> 예시 2

```Rust
fn foo(bar1: &str, bar2: &str ) -> &str
```

1번 규칙을 적용하면 아래와 같이 된다.

```Rust
fn foo(bar1: &'a str, bar2: &'b str ) -> &str
```

parameter가 두개이므로 2번 규칙을 적용할 수 없고 메서드가 아니므로 3번 규칙을 적용할 수 없다.

최종적으로 출력의 수명 매개변수가 확정되지 않았으므로 annotation을 생략할 수 없다.

## static 수명

static 수명은 프로그램 전역에서 수명이 유지된다.

`'static`으로 표기한다.

```Rust
let immortal_ant: &'static Ant = & Ant { name: "Megan" };
```

모든 string literal은 static 수명이 적용된다.
