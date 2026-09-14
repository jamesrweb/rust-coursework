pub fn login(username: &str) -> bool {
    if username.is_empty() {
        return false;
    }

    true
}

pub fn logout(username: &str) -> bool {
    if username.is_empty() {
        return false;
    }

    true
}

pub fn post(username: &str, content: &str) -> bool {
    if username.is_empty() && content.is_empty() {
        return false;
    }

    true
}
