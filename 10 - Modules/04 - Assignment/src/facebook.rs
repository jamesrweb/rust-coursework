pub fn login(username: &str) -> bool {
  if username.len() == 0 {
    return false;
  }

  return true;
}

pub fn logout(username: &str) -> bool {
  if username.len() == 0 {
    return false;
  }

  return true;
}

pub fn post(username: &str, content: &str) -> bool {
  if username.len() == 0 && content.len() == 0 {
    return false;
  }

  return true;
}
