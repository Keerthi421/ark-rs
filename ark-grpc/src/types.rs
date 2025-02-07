use crate::generated;
use crate::Error;
use ark_core::server;
use base64::Engine;
use bitcoin::address::NetworkUnchecked;
use bitcoin::Address;
use bitcoin::Amount;
use bitcoin::OutPoint;
use bitcoin::Psbt;

impl TryFrom<generated::ark::v1::GetInfoResponse> for server::Info {
    type Error = Error;

    fn try_from(value: generated::ark::v1::GetInfoResponse) -> Result<Self, Self::Error> {
        let pk = value.pubkey.parse().map_err(Error::conversion)?;

        let round_lifetime = bitcoin::Sequence::from_seconds_ceil(value.round_lifetime as u32)
            .map_err(Error::conversion)?;

        let unilateral_exit_delay =
            bitcoin::Sequence::from_seconds_ceil(value.unilateral_exit_delay as u32)
                .map_err(Error::conversion)?;

        let network = value.network.parse().map_err(Error::conversion)?;

        let forfeit_address: Address<NetworkUnchecked> =
            value.forfeit_address.parse().map_err(Error::conversion)?;
        let forfeit_address = forfeit_address
            .require_network(network)
            .map_err(Error::conversion)?;

        Ok(Self {
            pk,
            round_lifetime,
            unilateral_exit_delay,
            round_interval: value.round_interval,
            network,
            dust: Amount::from_sat(value.dust as u64),
            boarding_descriptor_template: value.boarding_descriptor_template,
            vtxo_descriptor_templates: value.vtxo_descriptor_templates,
            forfeit_address,
        })
    }
}

impl TryFrom<&generated::ark::v1::Vtxo> for server::VtxoOutPoint {
    type Error = Error;

    fn try_from(value: &generated::ark::v1::Vtxo) -> Result<Self, Self::Error> {
        let redeem_tx = match value.redeem_tx.is_empty() {
            true => None,
            false => {
                let base64 = base64::engine::GeneralPurpose::new(
                    &base64::alphabet::STANDARD,
                    base64::engine::GeneralPurposeConfig::new(),
                );

                let psbt = base64
                    .decode(value.redeem_tx.clone())
                    .map_err(Error::conversion)?;
                let psbt = Psbt::deserialize(&psbt).map_err(Error::conversion)?;
                Some(psbt)
            }
        };

        Ok(Self {
            outpoint: value
                .outpoint
                .clone()
                .map(|out| {
                    Ok(OutPoint {
                        txid: out.txid.parse().map_err(Error::conversion)?,
                        vout: out.vout,
                    })
                })
                .transpose()?,
            spent: value.spent,
            round_txid: value.round_txid.parse().map_err(Error::conversion)?,
            spent_by: value.spent_by.clone(),
            expire_at: value.expire_at,
            swept: value.swept,
            is_pending: value.is_pending,
            redeem_tx,
            amount: Amount::from_sat(value.amount),
            pubkey: value.pubkey.clone(),
            created_at: value.created_at,
        })
    }
}
