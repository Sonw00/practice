https://exercism.org/tracks/rust/exercises/armstrong-numbers

인자로 받은 `u32` 정수의 각 자릿수를 그 정수의 자릿수 개수만큼 각각 거듭제곱한 뒤 모두 더했을 때 원래 숫자와 같은지 확인해야 합니다.

우선 `u32` 공식 문서에서 `u32`를 n번 거듭 제곱한 수를 반환하는 함수를 제공하는지 확인합니다. -> [pow()](https://doc.rust-lang.org/std/primitive.u32.html#method.pow)

인자로 받은 num 변수를 10의 제곱수로 나누어가면서 몫이 0이 아닐 때까지 몫에 자릿수를 제곱해서 더하는 방법으로 구현합니다.

```rust
pub fn is_armstrong_number(num: u32) -> bool {
    let mut result:u32 = 0;

    for i in 0..=u32::MAX {
        let div = num / 10u32.pow(i);
        if div <= 0 {
            break;
        }
        else {
            result += div;
        }
    }

    result == num
}
```

테스트 실행 결과

```powershell
C:\Users\PC\Exercism\rust\armstrong-numbers> cargo test -- --include-ignored

running 9 tests
test four_digit_number_that_is_not_an_armstrong_number ... ok
test there_are_no_two_digit_armstrong_numbers ... ok
test single_digit_numbers_are_armstrong_numbers ... ok
test three_digit_number_that_is_not_an_armstrong_number ... ok
test zero_is_an_armstrong_number ... ok
test four_digit_number_that_is_an_armstrong_number ... FAILED
test seven_digit_number_that_is_an_armstrong_number ... FAILED
test three_digit_number_that_is_an_armstrong_number ... FAILED
test seven_digit_number_that_is_not_an_armstrong_number ... ok
```

알고리즘에 오류가 있었습니다. 자릿수의 개수를 각 자릿수에 제곱하기 위해 `num / 10u32.pow(i);` 연산으로 특정 자릿수를 구하려고 한 것이지만 이것은 i 뒤쪽 자릿수를 제거한 값이 됩니다.

---

자릿수를 구하는데 나머지를 활용할 수 있을 것 같습니다.

```rust
pub fn is_armstrong_number(num: u32) -> bool {
    let mut result: u32 = 0;

    let mut num_of_digit: u32 = 1;

    // 자릿수 개수를 구합니다.
    while num / 10u32.pow(num_of_digit) > 0 {
        num_of_digit += 1;
    }

    // num의 가장 높은 자릿수부터 시작해서 각 자릿수 max_digit을 자릿수 개수 num_of_digit 만큼 제곱해서 합산합니다.
    let mut remainder: u32 = num;
    for i in (0..=num_of_digit - 1).rev() {
        let max_digit: u32 = remainder / 10u32.pow(i);
        result += max_digit.pow(num_of_digit);

        // 합산한 최대 자릿수를 버립니다.
        remainder %= 10u32.pow(i);
    }

    result == num
}

```

테스트 통과됩니다.
