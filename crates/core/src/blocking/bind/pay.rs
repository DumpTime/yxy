//! Paying APIs

/// Create cashier URL by transaction No.
pub fn to_cashier(tran_no: &str) -> String {
    format!("{}?tran_no={}", crate::url::pay::TO_CASHIER, tran_no)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_cashier() {
        assert_eq!(
            to_cashier("1234567890"),
            "https://pay.xiaofubao.com/pay/unified/toCashier.shtml?tran_no=1234567890"
        );
    }
}
