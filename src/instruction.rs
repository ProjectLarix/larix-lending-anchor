//! Instruction types

use larix_lending::instruction::LendingInstruction;
use std::mem::size_of;

use crate::traits::Pack;

impl Pack for LendingInstruction {
    fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());

        match *self {
            Self::InitLendingMarket {
                owner,
                quote_currency,
            } => {
                buf.push(0);
                buf.extend_from_slice(&owner.as_ref());
                buf.extend_from_slice(&quote_currency);
            }
            Self::SetLendingMarketOwner { new_owner } => {
                buf.push(1);
                buf.extend_from_slice(&new_owner.as_ref());
            }
            // Self::InitReserve {
            //     config,
            //     total_mining_speed,
            //     kink_util_rate,
            //     use_pyth_oracle,
            //     is_lp,
            // } => {
            //     buf.push(2);
            //     buf.extend_from_slice(&[config.optimal_utilization_rate]);
            //     buf.extend_from_slice(&[config.loan_to_value_ratio]);
            //     buf.extend_from_slice(&[config.liquidation_bonus]);
            //     buf.extend_from_slice(&[config.liquidation_threshold]);
            //     buf.extend_from_slice(&[config.min_borrow_rate]);
            //     buf.extend_from_slice(&[config.optimal_borrow_rate]);
            //     buf.extend_from_slice(&[config.max_borrow_rate]);
            //     buf.extend_from_slice(&config.fees.borrow_fee_wad.to_le_bytes());
            //     buf.extend_from_slice(&config.fees.reserve_owner_fee_wad.to_le_bytes());
            //     buf.extend_from_slice(&config.fees.flash_loan_fee_wad.to_le_bytes());
            //     buf.extend_from_slice(&[config.fees.host_fee_percentage]);

            //     buf.extend_from_slice(&config.fees.host_fee_receivers);

            //     buf.extend_from_slice(&[config.deposit_paused as u8]);
            //     buf.extend_from_slice(&[config.borrow_paused as u8]);
            //     buf.extend_from_slice(&[config.liquidation_paused as u8]);
            //     buf.extend_from_slice(&config.deposit_limit.to_le_bytes());
            //     buf.extend_from_slice(&total_mining_speed.to_le_bytes());
            //     buf.extend_from_slice(&kink_util_rate.to_le_bytes());
            //     buf.extend_from_slice(&[use_pyth_oracle as u8]);
            //     buf.extend_from_slice(&[is_lp as u8]);
            // }
            Self::RefreshReserve => {
                buf.push(3);
            }
            Self::DepositReserveLiquidity { liquidity_amount } => {
                buf.push(4);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::RedeemReserveCollateral { collateral_amount } => {
                buf.push(5);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
            }
            Self::InitObligation => {
                buf.push(6);
            }
            Self::RefreshObligation => {
                buf.push(7);
            }
            Self::DepositObligationCollateral { collateral_amount } => {
                buf.push(8);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
            }
            Self::WithdrawObligationCollateral { collateral_amount } => {
                buf.push(9);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
            }
            Self::BorrowObligationLiquidity { liquidity_amount } => {
                buf.push(10);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::RepayObligationLiquidity { liquidity_amount } => {
                buf.push(11);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::LiquidateObligation { liquidity_amount } => {
                buf.push(12);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            // Self::FlashLoan {
            //     amount,
            //     call_back_data,
            // } => {
            //     buf.push(13);
            //     buf.extend_from_slice(&amount.to_le_bytes());
            //     buf.extend_from_slice(&call_back_data);
            // }
            Self::SetConfig => {
                buf.push(14);
            }
            Self::InitMining => {
                buf.push(16);
            }
            Self::DepositMining { amount } => {
                buf.push(17);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::WithdrawMining { amount } => {
                buf.push(18);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::ClaimMiningMine => {
                buf.push(20);
            }
            Self::ClaimObligationMine => {
                buf.push(21);
            }
            Self::ClaimOwnerFee => {
                buf.push(22);
            }
            Self::ReceivePendingOwner => {
                buf.push(23);
            }
            Self::RefreshReserves => {
                buf.push(24);
            }
            Self::LiquidateObligation2 { liquidity_amount } => {
                buf.push(25);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            _ => {
                // TODO: implementation
            }
        }

        buf
    }
}
