pub fn reply(message: &str) -> &str {
    let is_question = message.trim().ends_with("?");
    let is_silence = message.trim().is_empty();
    let alphabet: Vec<char> = message.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let is_yell = alphabet.len() != 0 && alphabet.iter().all(|c| c.is_uppercase());

    if is_silence {
        return "Fine. Be that way!";
    }

    if is_question && is_yell {
        return "Calm down, I know what I'm doing!";
    }

    if is_question {
        return "Sure.";
    }

    if is_yell {
        return "Whoa, chill out!";
    }

    "Whatever."
}
