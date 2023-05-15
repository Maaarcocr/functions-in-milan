use output::Discount;
use output::ProductVariantTarget;
use output::Target;
use shopify_function::prelude::*;
use shopify_function::Result;

use serde::{Serialize};

generate_types!(
    query_path = "./input.graphql",
    schema_path = "./schema.graphql"
);


#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {
    let no_discount = output::FunctionResult {
        discounts: vec![],
        discount_application_strategy: output::DiscountApplicationStrategy::FIRST,
    };

    let targets = input.cart.lines.into_iter().filter_map(|line| {
        match line.merchandise {
            input::InputCartLinesMerchandise::CustomProduct => None,
            input::InputCartLinesMerchandise::ProductVariant(variant) => {
                if variant.product.product_type.as_ref().map_or_else(|| false, |typ| {
                    typ == "t-shirt"
                }) {
                    Some(Target {
                        product_variant: Some(ProductVariantTarget {
                            id: variant.id,
                            quantity: None,
                        })
                    })
                } else {
                    None
                }
            },
        }
    }).collect::<Vec<Target>>();

    if targets.is_empty() {
        return Ok(no_discount);
    }

    Ok(output::FunctionResult {
        discounts: vec![
            Discount {
                message: None,
                targets: targets,
                value: output::Value {
                    fixed_amount: None,
                    percentage: Some(output::Percentage {
                        value: "10.0".to_string()
                    })
                }
            }
        ],
        discount_application_strategy: output::DiscountApplicationStrategy::FIRST,
    })
}
