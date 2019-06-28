pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let no_letters = message
        .chars()
        .into_iter()
        .all(|char| !char.is_alphabetic());
    let all_uppercase = message.to_uppercase() == message.to_string() && !no_letters;
    let question = message.ends_with("?");
    if message == "" {
        "Fine. Be that way!"
    } else if all_uppercase && !question {
        "Whoa, chill out!"
    } else if all_uppercase && question {
        "Calm down, I know what I'm doing!"
    } else if question {
        "Sure."
    } else {
        "Whatever."
    }
}
