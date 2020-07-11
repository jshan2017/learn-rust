# CH06 - Enums and Pattern Matching

## 열거자(enumerator)

아래와 같이 정의한다

```Rust
enum Status {
    Selected,
    InProgress,
    Done
}
```

아래와 같이 사용한다

```Rust
let status = Status::Done;
```

Rust에서는 아래와 같이 enumerator의 값에 데이터를 넣을수 있다.

```Rust
enum Status {
    Selected(String),
    InProgress(String),
    Done(String),
}

let status = Status::Done("Mar 11");
```

구조체와 같이 `impl` 키워드를 이용해 메소드를 정의할 수 있다.

```Rust
impl Status{
    fn toast(&self) {
    // .. do something
    }
}
let status = Status::Done("Mar 11");
status.toast();
```

## 패턴 매칭

패턴 매칭은 `switch` 문의 진보된 기법이라고 표현할 수 있다. Rust에서는 `match` 키워드를 통해 패턴 매칭을 지원하며 Scala 등의 언어와 사용방법이 유사하다.

아래는 이전에 정의한 `Status` enumerator을 패턴 매칭을 통해 처리한 코드이다.

```Rust
fn is_Done(status: &Status) -> bool {
    match status {
        Status::Selected => false,
        Status::InProgress => false,
        Status::Done => true
    }
}
```

주의할점으로 `match`는 반드시 모든 케이스에 대해 처리를 해주어야 한다.

## Option enumerator

Rust에 표준 라이브러리에서는 Nullity 개념을 구현하기 위해 `Option` 이라는 열거자를 지원하며 아래와 같이 구현되어있다.

```Rust
enum Option<T> {
    Some(T),
    None,
}
```

## Nullity 검사

`Option` 열거자와 패턴매칭을 이용하여 Nullity 검사 및 핸들링을 할 수 있다.

```Rust
fn status_transition(current: Option<Status>) -> Option<Status> {
    match current {
        None => None,
        Some(s) => {
        // .. do something, see ch06/src/main.rs
        },
    }
}

```

## match 와일드카드

상술했듯이 `match`는 모든 케이스에 대해 핸들링을 해주어야 하는데 이 번거로움을 줄이기 위해 와일드 카드 `_` 를 사용할 수 있다.
`_`는 다른 언어들의 `switch` 문의 `default`와 유사한 역할로 매칭되는 케이스가 없을때 핸들링을 해주는 역하을 해준다.

```Rust
fn is_Done(status: &Status) -> bool {
    match status {
        Status::Done => true,
        _ => false,
    }
}
```

## if let

단 하나의 케이스에 대해서만 핸들링을 해주고 싶다면 `match` 대신 `if let`을 통해 좀 더 간결하게 처리 할 수 있다.

```Rust
if let Status::Done(time) = status {
    println!("{}",time);
}

```
