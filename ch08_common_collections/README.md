# CH08 - Common Collections

## Vector 생성

Vector는 C++의 Vector와 유사하다. Heap 영역 메모리를 통해 데이터를 동적으로 관리할 수 있다.

아래와 같이 생성한다.

```Rust
let v: Vec<i32> =Vec::new();
```

생성만 하고 값을 넣지 않는경우 타입추론이 되지 않으므로 타입 명시를 해준다.

아래와 같이 매크로를 이용할 수도 있다.

```Rust
let v = vec![1,2,3];
```

## 데이터 삽입

Javascript 처럼 `push`를 통해 데이터를 삽입할 수 있다.

```Rust
v.push(5);
```

## Vector 해제

다른 구조체와 마찬가지로 scope를 벗어나면 메모리를 해제하기 위해 `drop`이 호출된다.

```Rust
{
    let v = vec![1,2,3];
    // ... do something

} // dropped
```

## Vector의 원소 읽기

Vector의 원소를 읽는 법은 두가지가 있다.
첫번째는 &, [] 를 이용해서 값의 참조를 읽는 것이다.

```Rust
let n = &v[2];
```

두번째는 `get` 메소드를 이용하는 것이다. 이때 리턴값의 타입은 `Option<&T>`이다.

```Rust
let n = v.get(2);
```

첫번째 방법의 경우 벡터의 길이를 초과하는 인덱싱을 할경우 `panic`이 발생하지만 두번째 방법을 사용한다면 `Option` 타입이 반환되므로 패턴 매칭을 통해 보다 안전하게 사용 가능하다.

## 열거자를 이용하는 배열

벡터는 같은 타입의 값만을 저장할 수 있다. 만약 서로 다른 타입의 값을 저장하고 싶다면 열거자를 이용하면 된다.

```Rust
enum AntProps`
    Name(String),
    Age(u32),
    Height(f64)
`

let data = vec![AntProps::Name("Hary"), AntProps::Age(12), AntProps::Height(0.56)];
```

## String 인스턴스 생성

아래 방법들을 통해 String 인스턴스를 생성할 수 있다.

```Rust
let s0 = String.new();
let s1 = String.from("ant");
let s2 = "ant".to_string();
```

## 문자열 붙이기

`String` instance에 문자열 슬라이스를 붙이기 위해 `push_str`을 사용한다.

```Rust
let s = String::from("Hello");
s.push_str("world");
```

`String` instance 두개를 붙이기 위해 `+` 연산자를 사용할 수 있다.

```Rust
let s0 = String::from("Hello");
let s1 = String::from("World");
s2 = s0 + s1;
```

위 예제에서 `s2 = s0 + &s1` 이후로는 s0이 유효하지 않다. 그 이유는 `+` 연산자가 사용하는 `add` 메소드가 아래와 같이 선언되어있기 때문이다.

```Rust
fn add(self, s: &str) -> String
```

`add`는 `self`로 `s0`의 소유권을 받고 여기에 `s1`의 데이터를 복사해서 그 결과값의 소유권을 리턴한다.

## 문자열 인덱싱

Rust의 문자열은 UTF-8 인코딩을 사용한다. UTF-8에서는 ASCII에 포함된 문자는 1바이트를, 그렇지 않은 문자는 3바이트를 차지한다. `String` 타입은 기본적으로 `Vec<u8>` 타입의 wrapper인데 `u8` 타입은 1바이트를 차지한다 이로인해 정수값 indexing을 통해 글자를 가져오려고 시도하면 글자를 가져올 수 없고 따라서 에러가 발생한다. 이는 일관성을 위해 ASCII에 포함된 문자에 대해서도 마찬가지이다.

올바르게 글자를 가져오려면 다음과 같이 해야한다

```Rust
let ant = String::from("Ant");
let a = &ant[0..3]; // Ant - case ASCII

let ant_kr = String::from("개미");
let a_kr = &ant_kr[0..3]; // 개 = case non-ASCII
```

만약 non ASCII 문자에 대해 올바르지 않은 슬라이싱을 하면 에러가 발생한다.

```Rust
let ant_kr = String::from("개미");
let a_kr = &ant_kr[1..4]; // error!
```

## HashMap

`HashMap`은 Heap 메모리 영역을 이용하여 동적으로 Key-Value 쌍을 저장할 수 있는 컬렉션이다.
`HashMap`은 아래와 같이 생성한다.

```Rust
let ants = std::collections::HashMap::new();
```

이 외에 전체적인 용도 및 사용법은 다른 언어들과 큰 차이가 없다.
