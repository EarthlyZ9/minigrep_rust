# Minigrep

__Rust 로 구현한 grep 의 mini version.__

_From Rust Lang Book Chap 12_

> query 와 filename 을 받아 file 내용에서 query 를 검색해주는 간단한 CLI 프로그램.

## 사용된 Rust 개념들

### 1 .Vector

Vector 는 배열 (array) 과 같이 동일한 타입의 요소들을 갖지만, 그 크기가 고정된 배열과는 달리, 그 크기를 신축적으로 조절할 수 있는 데이터 타입이다.

### 2. Binary Crate 와 Library Crate

* `main.rs`: Binary crate. `lib.rs` 에서 함수를 호출함.
* `lib.rs`: Library crate. 각종 함수들과 모듈이 정의되는 곳.

> Crate 는 러스트가 컴파일 한 차례에 고려하는 가장 작은 코드 단위이다.

### 3. Result 의 `unwrap_or_else`

* `unwrap_or_else` 를 사용하면 `panic!` 으로 에러를 처리하는 대신에 직접 에러를 어떻게 처리할지 정의해줄 수 있다.
* Result 값이 Ok 이면 그 값을 그대로 전달해주지만 (`unwrap()` 과 동일), Err 이면 closure 의 코드를 호출한다.
* 에러가 발생했을 때, 실행할 콜백함수를 제공해주는 것과 같다.

> closure: 익명의 함수로 unwrap_or_else 함수에 인자로 전달되는 함수

### 4. 함수가 레퍼런스 타입을 반환하는 경우

* 함수가 어떤 값의 레퍼런스를 반환하는 경우, 반환값의 lifetime 을 함수의 인자 중 하나의 lifetime 과 연결해줘야한다.
* `search()` 함수 참고

### 5. `println!` 으로 출력한 에러메시지를 std error 로 출력하기

* 일반적인 `println!` 매크로를 사용할 경우 `cargo run > output.txt` 를 수행했을 때 에러메시지가 output 파일에 쓰여지는 일이 발생한다.
* 이를 방지하기 위해 에러메시지를 출력하는 경우에는 `eprintln!` 매크로를 사용한다.