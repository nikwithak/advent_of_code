use std::str::Chars;

struct StringSearchStateMachine<'a> {
    chars: Chars<'a>,
}

impl StringSearchStateMachine<'_> {
    fn new(needle: &str) -> Self {
        Self {
            chars: needle.chars(),
        }
    }
}
