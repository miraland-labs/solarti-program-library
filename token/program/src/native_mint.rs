//! The Mint that represents the native token

/// There are 10^9 lamports in one MLN
pub const DECIMALS: u8 = 9;

// The Mint for native MLN Token accounts
miraland_program::declare_id!("MLN1111111111111111111111111111111111111111");

#[cfg(test)]
mod tests {
    use {super::*, miraland_program::native_token::*};

    #[test]
    fn test_decimals() {
        assert!(
            (lamports_to_mln(42) - crate::amount_to_ui_amount(42, DECIMALS)).abs() < f64::EPSILON
        );
        assert_eq!(
            mln_to_lamports(42.),
            crate::ui_amount_to_amount(42., DECIMALS)
        );
    }
}
