# CH13 - Functional Language Features: Iterators and Closures

## 클로저

다른 언어들이 람다 함수를 지원하듯이 Rust도 closure라는 이름으로 이 기능을 지원한다.
아래와 같이 변수나 parameter에 담아 사용한다.

```Rust
let f = |x| x + 1;

let g = |handler, y| {
    let z = handler(y) * 2;
    z;
}

f(1); // 2
g(f, 1); // 4
```

클로저는 parameter와 return 값의 타입을 명시하지 않아도 된다.

## 클로저의 환경 캡처

프로그래밍 언어론에서 클로저는 일반적인 함수와 달리 선언될 때의 환경을 내부에서 이용할 수 있다.
이를테면 아래의 일반적인 함수 선언은 컴파일 에러를 낸다.

```Rust
let x=3;
fn foo(){
    x; // error - can't capture dynamic environment in a fn item
}
```

하지만 클로저는 선언될 때의 환경을 캡처하여 내부적으로 이용할 수 있게 한다.

```Rust
let x=3;
let foo = || x;
foo(); // 3
```

## 클로저 소유권 이전

클로저는 기본적으로 환경으로부터 가져온 값에 대한 소유권을 불변으로 대여한다. 만약 소유권을 클로저로 이전하려면 `move`키워드를 사용할 수 있다.

```Rust
let x= vec![1,2,3];
let steal = move || x;
println!("{:?}", x) // err
```

Heap에 할당된 벡터가 `move`로 인해 `steal`로 소유권이 이전되었다. 만약 `x`가 `3`처럼 `Copy trait`이 적용되는 데이터라면 정상 작동한다.

## iterator

C++, Javascript 처럼 Rust도 iterator를 지원하다. 아래는 벡터의 iterator 사용 예제이다

```Rust
let v1 = vec![1,2,3];
let v1_iter = v1.iter();

for n in v1_iter {
    println!("{}",n);
}
```

## iterator 활용

iterator를 이용해 `map`, `filter` 등의 연산을 할 수 있다. iterator는 기본적으로 lazy evaluataion이므로 마지막에 `collect` 메소드를 호출하여 소비해주어야 한다.

```Rust
  let today= vec![1,2,3];
  let tomorrow : Vec<_>= today.iter().map(|n| n+1).collect();
```
