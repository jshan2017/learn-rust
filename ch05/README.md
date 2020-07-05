# CH05 - Using Structs to Structure Related Data

## struct

구조체는 아래와 같이 정의한다.

```Rust
struct Ant {
    id: String,
    name: String,
    age: u32
}
```

구조체는 다음과 같이 사용한다

```Rust
let immutableAnt = Ant {
    id: String::from("ck92nq"),
    name: String::from("James"),
    age: 23,
};

let mut mutableAnt = Ant {
    id: String::from("lc9vkn"),
    names: String::from("Sally"),
    age: 54,
};
```

위 코드에서 mut으로 선언한경우 모든 필드가 mutable이다. mutable/immutable 필드를 혼용 할 수 없다.

## 필드 초기화 단축 문법

Javascript와 유사하게 필드 명과 동일한 이름의 변수가 있다면 value를 생략할 수 있다.

```Rust
let id: String::from("ck92nq");

let james = Ant {
    id,
    name: String::from("James"),
    age: 23,
};
```

## 구조체 갱신 문법

Javscript의 destructing과 비슷하게 아래와 같은 방식으로 인스턴스를 생성 가능하다.

```Rust
let paul = Ant {
    name: String::from("Paul"),
    ..james
}
```

## 이름 없는 필드를 가진 구조체

아래와 같이 선언 및 사용 할 수 있다.

```Rust
strict Ant{String,String,u32};
let james = Ant("ck92nq", "James",23);
println!("{} {} {}", james.0, james.1, james.2);
```

## 메서드 정의하기

다른 언어들의 클래스 개념은 따로 없지만 구조체에 메서드를 정의할수 있다.
메서드를 정의하기 위해 `impl` 키워드를 사용한다.
`impl` 블록 여러개로 나누어 메서드를 정의할 수도 있다.

```Rust
impl Ant {
    fn jump(&self){
        println!("{} jumps",self.name);
    }
}
```

메서드 내에서 인스턴스를 참조하기 위해 `&self`를 parameter로 사용한다.
`self`는 가변참조로 넘길수도 있고 아얘 소유권을 넘길 수도 있다. `self`가 없을 경우 C++의 `static` 메서드처럼 인스턴스 없이 사용할 수 있다. String::new()가 그런 케이스

```Rust
 pub const fn new() -> String {
        String { vec: Vec::new() }
    }
```

`self`인자의 경우 파이썬과 유사하게 호출시에는 인자로 넘겨줄 필요없다.
