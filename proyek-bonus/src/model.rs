use crate::error::AppError;
use crate::shortcode::ShortCode;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

/// Status siklus hidup sebuah link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkStatus {
    Active,
    Disabled,
}

impl LinkStatus {
    pub fn can_redirect(self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disabled => "disabled",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, AppError> {
        match value {
            "active" => Ok(Self::Active),
            "disabled" => Ok(Self::Disabled),
            other => Err(AppError::Internal(format!("status tidak dikenal: {other}"))),
        }
    }
}

impl fmt::Display for LinkStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for LinkStatus {
    type Err = AppError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_db(value)
    }
}

/// Satu link yang disimpan aplikasi.
#[derive(Debug, Clone, Serialize)]
pub struct Link {
    pub code: ShortCode,
    pub target: String,
    pub clicks: u64,
    pub status: LinkStatus,
}

impl Link {
    pub fn new(code: ShortCode, target: String) -> Self {
        Self {
            code,
            target,
            clicks: 0,
            status: LinkStatus::Active,
        }
    }

    pub fn can_redirect(&self) -> bool {
        self.status.can_redirect()
    }

    pub fn with_status(mut self, status: LinkStatus) -> Self {
        self.status = status;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_link_is_active_and_has_no_clicks() {
        let link = Link::new(
            ShortCode::parse("promo").unwrap(),
            "https://example.com".to_string(),
        );
        assert_eq!(link.clicks, 0);
        assert!(link.can_redirect());
    }

    #[test]
    fn status_roundtrip() {
        assert_eq!(
            LinkStatus::from_db("disabled").unwrap(),
            LinkStatus::Disabled
        );
        assert!(!LinkStatus::Disabled.can_redirect());
    }
}
