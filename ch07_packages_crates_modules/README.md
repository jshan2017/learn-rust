# CH07 - Managing Growing Projects with Packages, Crates, and Modules

## 모듈 트리

다른 언어들과 유사하게 Rust도 모듈을 여러단계에 걸쳐 구조화 할 수 있다. 이 모듈의 계층 구조를 모듈 트리라고 한다.

## crate

모듈트리의 root를 `crate`라고 한다. 기본적으로 `main.rs`와 `lib.rs` 파일은 모듈트리의 root가 된다.

## 모듈 정의

`mod` 키워드를 사용해 모듈을 정의한다. 아래와 같이 `mod` scope 내에 또다른 `mod` scope를 생성할 수 도 있다.

```Rust
mod foo {
    mod bar {
        fn ant() {}
    }
}
```

## 모듈 참조

`mod` 키워드를 이용해 정의된 모듈을 참조하기 위해 절대경로와 상대경로를 모두 사용가능하다. 절대경로의 경우 root인 `crate`부터 시작한다.

```Rust
crate::foo::bar::ant(); // absolute path

// ..

bar::ant(); // relative path from foo

```

상위 모듈을 참조하려면 `super` 키워드를 사용한다 파일 시스템에서 `../`와 유사한 역할을 수행한다.

```Rust
super::bar::ant() // relative path from baz, a sibling of bar
```

## public 모듈

기본적으로 부모 모듈에서 자식 모듈을 참조하는것이 비활성화 되어있다. 이러한 점을 이용해 외부에 공개할 필요가 없는 데이터에 대한 접근을 막을 수 있다.
모듈 또는 함수에 대한 상위 모듈의 참조를 허용하고 싶다면 `pub`키워드를 사용한다.

```Rust
mod foo {
    pub mod bar{
        pub fn ant(){}
    }
}
```

## enum, struct에 대한 공개 설정

열거체, 구조체도 `pub` 키워드를 통해 공개 설정을 할 수 있다. 구조체의 경우, 내부 필드는 여전히 비공개이기 때문에 추가적으로 `pub`을 붙여주어야한다.

```Rust
pub struct Ant{
    pub name: String, //public
    age: u32, // private
}

pub enum Lang {
    C, // public
    Rust, // public
    Typescript // public
}
```

## use 키워드

CH02에서 언급했듯이 `use` 키워드를 사용하여 네임스페이스를 단축할 수 있다. C++의 `using namespace`와 비슷한 기능.

```Rust
use crate::foo::bar;
bar::ant();
```

`use` 키워드와 상대경로를 사용할 때 현재 경로부터 시작한다면 `self`를 붙여주어야한다.

```Rust
use self::bar::ant(); // relative path from foo
```

## as 키워드

`use`로 모듈을 가져올때 `as` 키워드로 별칭을 붙일 수 있다. Python이나 Javascript 모듈 시스템과 유사하다.

```Rust
use crate::foo::bar as foobar
```

## pub use

`use` 키워드로 가져온 모듈또한 기본적으로 비공개가 된다. 이를 공개하기 위해 `pub use`를 사용한다.

## 중첩 경로

아래와 같이 `use`문의 줄 수 를 줄일 수 있다.

```Rust
use std::io;
use std::cmp;
use std::cmp::Ordering;

// reduces to

use std::{io, cmp::{self, Ordering}}
```

## use \*

특정 경로의 아이템을 모두 가져오기 위해 `*`을 사용한다.

```Rust
use std::*

io::stdin();
```

## 파일 분리

아래와 같이 파일을 분리하여 모듈을 보다 체계적으로 관리할 수 있다.

```Rust
// src/foo.rs
mod bar;
```

```Rust
// src/foo/bar.rs;
pub ant() {}
```

또는 아래와 같이 `mod.rs`를 사용한다. Web 프로젝트의 `index.js`와 같은 역할을 한다.

```Rust
// src/foo/mod.js
mod bar;
```

```Rust
// src/foo/bar.rs;
pub ant() {}
```
