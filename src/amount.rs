//! The money one completed request was billed.

use crate::config::types::ModelPricing;
use crate::provider::TokenUsage;
use std::collections::HashMap;

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

    /// The surface form: cents with one significant digit, never more than
    /// four decimal places, so a sub-cent amount reads `0.02`, a very small one
    /// reads `0.0009`, and an amount of a cent or more reads whole cents. A
    /// genuine zero renders as `0`.
    pub fn cents_text(&self) -> String {
        let cents = self.usd * 100.0;
        if cents == 0.0 {
            return "0".to_string();
        }
        // One digit after the leading zeroes, capped at four decimals: the
        // decimals exist to carry the value past its leading zeroes.
        let decimals = if cents.abs() < 1.0 {
            (1 + (-cents.abs().log10()).floor() as i32).clamp(0, 4) as usize
        } else {
            0
        };
        let mut text = format!("{:.decimals$}", cents);
        if text.contains('.') {
            while text.ends_with('0') {
                text.pop();
            }
            if text.ends_with('.') {
                text.pop();
            }
        }
        text
    }
}

/// The recorded price that applies to a completed request: the price of the
/// model the provider reported, or — when that model has no recorded price —
/// the price of the model the request was sent with. A provider routinely
/// answers with its canonical identifier while the recorded price carries the
/// configured alias, so the two rarely match verbatim.
pub fn recorded_price<'a>(
    pricing: &'a HashMap<String, ModelPricing>,
    reported_model: &str,
    requested_model: &str,
) -> Option<&'a ModelPricing> {
    pricing
        .get(reported_model)
        .or_else(|| pricing.get(requested_model))
}

/// `None` when no recorded price applies to the request; otherwise the amount,
/// with `usage_reported` recording whether a report existed.
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

    fn cents_text_of(usd: f64) -> String {
        super::BilledAmount {
            usd,
            usage_reported: true,
        }
        .cents_text()
    }

    fn usage(prompt_tokens: u32, completion_tokens: u32) -> TokenUsage {
        TokenUsage {
            prompt_tokens,
            completion_tokens,
        }
    }

    #[test]
    fn the_reported_model_price_wins_over_the_requested_model_price() {
        let mut pricing = std::collections::HashMap::new();
        pricing.insert("~alias/model".to_string(), price(0.04, 1.00));
        pricing.insert("canonical/model".to_string(), price(0.10, 2.00));
        let recorded = super::recorded_price(&pricing, "canonical/model", "~alias/model")
            .expect("the reported model is priced");
        assert_eq!(recorded.input, 0.10);
        assert_eq!(recorded.output, 2.00);
        let recorded = super::recorded_price(&pricing, "unknown/model", "~alias/model")
            .expect("the requested model is priced");
        assert_eq!(recorded.input, 0.04);
        assert!(super::recorded_price(&pricing, "unknown/model", "also-unknown").is_none());
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
        assert_eq!(amount.cents_text(), "0.02");
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
    fn one_significant_digit_carries_the_value_past_its_leading_zeroes() {
        assert_eq!(cents_text_of(0.000009), "0.0009");
        assert_eq!(cents_text_of(0.00001), "0.001");
        assert_eq!(cents_text_of(0.0001), "0.01");
        assert_eq!(cents_text_of(0.001), "0.1");
        assert_eq!(cents_text_of(0.001001), "0.1");
        assert_eq!(cents_text_of(0.0011), "0.1");
    }

    #[test]
    fn four_decimal_places_is_the_smallest_step_shown() {
        assert_eq!(cents_text_of(0.000001), "0.0001");
        assert_eq!(cents_text_of(0.00000001), "0");
    }

    #[test]
    fn an_amount_of_a_cent_or_more_reads_as_whole_cents() {
        assert_eq!(cents_text_of(0.0135), "1");
        assert_eq!(cents_text_of(0.023), "2");
    }

    #[test]
    fn a_reported_zero_is_shown_as_zero() {
        let amount = billed_amount(Some(&usage(0, 0)), Some(&price(0.15, 0.60)))
            .expect("a recorded price yields an amount");
        assert!(amount.usage_reported());
        assert_eq!(amount.cents_text(), "0");
    }
}
