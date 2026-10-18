pub fn reply(message: &str) -> &str {
    let is_question = message.trim().ends_with("?");
    let is_yell = message.to_uppercase() == message;
    let is_silence = message.trim().is_empty();

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
