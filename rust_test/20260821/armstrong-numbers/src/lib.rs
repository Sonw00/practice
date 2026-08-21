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
