# CH02 - Guessing Game

## 모듈

- 다른 모듈을 가져오기 위해 `use` keword를 사용한다.
- standard library인 `std`의 경우 Rust Compiler가 기본으로 링크를 해주기때문에 `use` 키워드를 사용하지 않아도 된다. `no_std` 옵션으로 이걸 끌 수 있다.
- rust의 모듈을 `crate`라고 하며 패키지 저장소로 [crates.io](https://crates.io)를 사용한다. `Cargo.toml`의 dependency에 패키지를 추가하고 `$ cargo build` 하면 설치할 수 있다.

## 변수

- immutable 변수의 선언에는 `let`을, mutable에는 `let mut`을 사용한다.
- 변수의 타입은 명시하지 않아도 된다. 타입추론이 지원되기 때문. 타입 명시는 Typescript처럼 `let a:u32 = 0;`와 같이 한다.
- 각종 naming conventinon은 [여기](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)에서 확인할 수 있다.
- shadowing이 가능하다. CH03에서 자세히 다룸.

## 기타 Syntax

- `String::new`의 `::` 는 associated function임을 의미한다. C++의 `static` method와 유사. `::new()` 로 인스턴스를 생성한다.
- `&`는 reference를 나타낸다. mutable 변수의 참조를 넘길때는 `&mut guess` 처럼 `mut` 키워드를 포함해야한다.
- `Result` 타입은 `expect`를 통해 에러 핸들링을 해 주어야 한다.
- 포매팅은 `{}`를 이용해서 한다.
- `match` 키워드로 패턴매칭을 할 수 있다. `match` 문은 리턴값을 가진다.
