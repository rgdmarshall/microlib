//! No library at all.

#![warn(missing_docs)]

/// Is this awesome?
pub fn is_awesome() -> bool {
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn awesome() {
	assert!(super::is_awesome());
    }
}
