//! The register and login forms, and their checks.

use serde::Deserialize;

/// Password length, in characters.
const PASSWORD_MIN_CHARS: usize = 8;
const PASSWORD_MAX_CHARS: usize = 128;
/// Display name length, in characters.
const NAME_MIN_CHARS: usize = 2;
const NAME_MAX_CHARS: usize = 50;
const EMAIL_MAX_CHARS: usize = 254;

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

/// Trims and lowercases an email, so one address maps to one account.
pub(super) fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

impl RegisterForm {
    /// The first problem with the form, as a message for the user.
    pub fn problem(&self) -> Option<String> {
        let name = self.display_name.trim().chars().count();
        if !(NAME_MIN_CHARS..=NAME_MAX_CHARS).contains(&name) {
            return Some(format!(
                "Tên hiển thị dài từ {NAME_MIN_CHARS} đến {NAME_MAX_CHARS} ký tự."
            ));
        }
        let email = normalize_email(&self.email);
        let looks_valid = email.len() <= EMAIL_MAX_CHARS
            && email.split_once('@').is_some_and(|(user, domain)| {
                !user.is_empty()
                    && domain.contains('.')
                    && !domain.starts_with('.')
                    && !domain.ends_with('.')
            })
            && !email.contains(char::is_whitespace);
        if !looks_valid {
            return Some("Email không hợp lệ.".to_owned());
        }
        let password = self.password.chars().count();
        if !(PASSWORD_MIN_CHARS..=PASSWORD_MAX_CHARS).contains(&password) {
            return Some(format!(
                "Mật khẩu dài từ {PASSWORD_MIN_CHARS} đến {PASSWORD_MAX_CHARS} ký tự."
            ));
        }
        if self.password != self.confirm_password {
            return Some("Hai mật khẩu không khớp.".to_owned());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn form(name: &str, email: &str, password: &str, confirm: &str) -> RegisterForm {
        RegisterForm {
            display_name: name.to_owned(),
            email: email.to_owned(),
            password: password.to_owned(),
            confirm_password: confirm.to_owned(),
        }
    }

    #[test]
    fn register_checks() {
        assert_eq!(
            form("Kha", " Kha@Example.com ", "12345678", "12345678").problem(),
            None
        );
        assert!(
            form("K", "a@b.vn", "12345678", "12345678")
                .problem()
                .is_some()
        );
        assert!(
            form("Kha", "not-an-email", "12345678", "12345678")
                .problem()
                .is_some()
        );
        assert!(form("Kha", "a@b.vn", "short", "short").problem().is_some());
        assert!(
            form("Kha", "a@b.vn", "12345678", "12345679")
                .problem()
                .is_some()
        );
        assert_eq!(normalize_email(" Kha@Example.COM "), "kha@example.com");
    }
}
