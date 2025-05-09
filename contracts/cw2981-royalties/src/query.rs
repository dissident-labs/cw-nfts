use crate::msg::{CheckRoyaltiesResponse, RoyaltiesInfoResponse};
use crate::state::Cw2981Contract;
use crate::DefaultOptionMetadataExtensionWithRoyalty;
use cosmwasm_std::{Decimal, Deps, Empty, StdResult, Uint128};
use cw721::traits::Cw721Query;

impl
    Cw721Query<
        DefaultOptionMetadataExtensionWithRoyalty,
        Empty, // no collection extension
        Empty, // no extension query
    > for Cw2981Contract<'_>
{
}

/// NOTE: default behaviour here is to round down
/// EIP2981 specifies that the rounding behaviour is at the discretion of the implementer
pub fn query_royalties_info(
    deps: Deps,
    token_id: String,
    sale_price: Uint128,
) -> StdResult<RoyaltiesInfoResponse> {
    let contract = Cw2981Contract::default();
    let token_info = contract.query_nft_info(deps.storage, token_id)?;

    let royalty_percentage = match token_info.extension {
        Some(ref ext) => match ext.royalty_percentage {
            Some(percentage) => Decimal::percent(percentage),
            None => Decimal::percent(0),
        },
        None => Decimal::percent(0),
    };
    let royalty_from_sale_price = sale_price.mul_floor(royalty_percentage);

    let royalty_address = match token_info.extension {
        Some(ext) => ext.royalty_payment_address.unwrap_or_default(),
        None => String::from(""),
    };

    Ok(RoyaltiesInfoResponse {
        address: royalty_address,
        royalty_amount: royalty_from_sale_price,
    })
}

/// As our default implementation here specifies royalties at token level
/// and not at contract level, it is therefore logically true that
/// on sale, every token managed by this contract should be checked
/// to see if royalties are owed, and to whom. If you are importing
/// this logic, you may want a custom implementation here
pub fn check_royalties(_deps: Deps) -> StdResult<CheckRoyaltiesResponse> {
    Ok(CheckRoyaltiesResponse {
        royalty_payments: true,
    })
}
