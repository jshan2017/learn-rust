# CH11 - Writting Automated Tests

## 테스트 함수 작성

테스트 함수는 `[#test]` attribute를 사용하여 명시한다.

```Rust
[#test]
fn sample_test(){
    assert_eq!(2 + 2, 4);
}
```

테스트 함수가 수행되며 `panic`이 일어나지 않으면 테스트가 통과된다.

## 테스트 실행

```shell
$ cargo test
```

위 명령어를 통해 프로젝트 내의 테스트 코드를 컴파일하고 실행한다.

## 테스트 관련 매크로

테스트를 위해 아래 매크로들을 활용할 수 있다. 이 매크로들은 조건에 맞지 않는 결과가 나오면 패닉을 일으킨다.
`assert!`, `assert_eq!` `assert_ne!`

## should_panic

경우에 따라 `panic`이 일어나야 하는 테스트 함수도 있다.
이를 위해 `#[should_panic]` annotation을 사용한다/

```Rust
#[test]
#[should_panic]
fn sample_test(){
    assert_eq!(2 + 2, 4);  // pass if 2+2 != 4
}
```

## test thread 설정

Rust의 테스트는 기본적으로 병렬로 실행된다. 만약 스레드 개수를 제한하고 싶다면 아래 옵션을 사용할 수 있다.

```bash
$ cargo test -- --test-threads=1
```

## 테스트시 stdout 출력

테스트가 성공시에는 기본적으로 stdout 출력을 보여주지 않는다. 만약 성공시에도 출력을 보고싶다면 아래 옵션을 사용할 수 있다.

```bash
$ cargo test -- --nocapture
```

## Unit 테스트

Unit 테스트 코드는 보통 테스트 할 대상과 같은 파일 내에 작성한다.
이를 위해 `#[cfg(test)]` attribute를 사용하는데, 이 attribute가 명시된 함수는 테스트시에만 컴파일된다.

```Rust
#[cfg(test)]
mod tests {
    #[test]
    sample_test(){
        // ...
    }
}
```

## private 함수의 테스트

Rust는 테스트 코드에서 private(`pub`이 아닌) 함수를 호출할 수 있다.

## 통합 테스트

통합테스트 코드는 최상위 디렉터리의 `tests` 디렉터리 내에 작성한다.
