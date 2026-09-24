//! The money one completed request was billed.

use crate::config::types::ModelPricing;
use crate::provider::TokenUsage;

/// The money one completed request was billed, in USD.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BilledAmount {
    usd: f64,
    usage_reported: bool,
}

impl BilledAmount {
    /// The billed amount in USD, as the post-request metadata line reports it.
    pub fn usd(&self) -> f64 {
        self.usd
    }

    /// Whether the response carried a usage report at all. The metadata line
    /// cannot tell this apart from a reported zero; the review surface must.
    pub fn usage_reported(&self) -> bool {
        self.usage_reported
    }

    /// The surface form: cents with four decimals and trailing zeros trimmed,
    /// so a request the provider accounted for never reads as zero. A genuine
    /// zero renders as `0`; a billed amount too small for four decimals keeps
    /// one more decimal at a time until it is visible.
    pub fn cents_text(&self) -> String {
        if self.usd == 0.0 {
            return "0".to_string();
        }
        let mut decimals = 4;
        loop {
            let mut text = format!("{:.decimals$}", self.usd * 100.0);
            while text.ends_with('0') {
                text.pop();
            }
            if text.ends_with('.') {
                text.pop();
            }
            if text != "0" || decimals >= 12 {
                return text;
            }
            decimals += 1;
        }
    }
}

/// `None` when no recorded price matches the reported model; otherwise the
/// amount, with `usage_reported` recording whether a report existed.
///
/// A priced model without a usage report yields `0.0`, which is what the
/// post-request metadata line has always printed for it.
pub fn billed_amount(
    usage: Option<&TokenUsage>,
    price: Option<&ModelPricing>,
) -> Option<BilledAmount> {
    let price = price?;
    let prompt_tokens = usage.map_or(0, |usage| usage.prompt_tokens);
    let completion_tokens = usage.map_or(0, |usage| usage.completion_tokens);
    let usd = (price.input * f64::from(prompt_tokens)
        + price.output * f64::from(completion_tokens))
        / 1_000_000.0;
    Some(BilledAmount {
        usd,
        usage_reported: usage.is_some(),
    })
}

#[cfg(test)]
mod tests {
    use super::billed_amount;
    use crate::config::types::ModelPricing;
    use crate::provider::TokenUsage;

    fn price(input: f64, output: f64) -> ModelPricing {
        ModelPricing { input, output }
    }

    fn usage(prompt_tokens: u32, completion_tokens: u32) -> TokenUsage {
        TokenUsage {
            prompt_tokens,
            completion_tokens,
        }
    }

    #[test]
    fn no_recorded_price_yields_no_amount() {
        assert!(billed_amount(Some(&usage(1200, 90)), None).is_none());
    }

    #[test]
    fn reported_usage_is_billed_at_the_recorded_price() {
        let amount = billed_amount(Some(&usage(1200, 90)), Some(&price(0.15, 0.60)))
            .expect("a recorded price yields an amount");
        assert!(amount.usage_reported());
        assert_eq!(amount.cents_text(), "0.0234");
    }

    #[test]
    fn a_priced_request_without_a_usage_report_is_not_reported() {
        let amount =
            billed_amount(None, Some(&price(0.15, 0.60))).expect("a recorded price yields 0.0");
        assert!(!amount.usage_reported());
        assert_eq!(amount.usd(), 0.0);
        assert_eq!(amount.cents_text(), "0");
    }

    #[test]
    fn trailing_zeros_are_trimmed() {
        let amount = billed_amount(Some(&usage(1500, 200)), Some(&price(2.50, 10.00)))
            .expect("a recorded price yields an amount");
        assert_eq!(amount.cents_text(), "0.575");
        let amount = billed_amount(Some(&usage(1000, 100)), Some(&price(2.50, 10.00)))
            .expect("a recorded price yields an amount");
        assert_eq!(amount.cents_text(), "0.35");
    }

    #[test]
    fn a_reported_zero_is_shown_as_zero() {
        let amount = billed_amount(Some(&usage(0, 0)), Some(&price(0.15, 0.60)))
            .expect("a recorded price yields an amount");
        assert!(amount.usage_reported());
        assert_eq!(amount.cents_text(), "0");
    }

    #[test]
    fn a_billed_request_never_reads_as_zero() {
        // One token at $0.02 per million is 0.000002 cents: four decimals
        // would round it to `0`, so the form keeps more decimals until the
        // billed amount is visible.
        let amount = billed_amount(Some(&usage(1, 0)), Some(&price(0.02, 0.02)))
            .expect("a recorded price yields an amount");
        assert_eq!(amount.cents_text(), "0.000002");
    }
}
