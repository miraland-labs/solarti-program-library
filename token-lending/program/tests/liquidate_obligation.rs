#![allow(clippy::arithmetic_side_effects)]
#![cfg(feature = "test-sbf")]

mod helpers;

use {
    helpers::*,
    miraland_program_test::*,
    miraland_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    spl_token::instruction::approve,
    spl_token_lending::{
        instruction::{liquidate_obligation, refresh_obligation},
        processor::process_instruction,
        state::INITIAL_COLLATERAL_RATIO,
    },
};

#[tokio::test]
async fn test_success() {
    let mut test = ProgramTest::new(
        "spl_token_lending",
        spl_token_lending::id(),
        processor!(process_instruction),
    );

    // limit to track compute unit increase
    test.set_compute_max_units(68_000);

    // 100 MLN collateral
    const MLN_DEPOSIT_AMOUNT_LAMPORTS: u64 = 100 * LAMPORTS_TO_MLN * INITIAL_COLLATERAL_RATIO;
    // 100 MLN * 80% LTV -> 80 MLN * 20 USDC -> 1600 USDC borrow
    const USDC_BORROW_AMOUNT_FRACTIONAL: u64 = 1_600 * FRACTIONAL_TO_USDC;
    // 1600 USDC * 50% -> 800 USDC liquidation
    const USDC_LIQUIDATION_AMOUNT_FRACTIONAL: u64 = USDC_BORROW_AMOUNT_FRACTIONAL / 2;
    // 800 USDC / 20 USDC per MLN -> 40 MLN + 10% bonus -> 44 MLN
    const MLN_LIQUIDATION_AMOUNT_LAMPORTS: u64 = 44 * LAMPORTS_TO_MLN * INITIAL_COLLATERAL_RATIO;

    const MLN_RESERVE_COLLATERAL_LAMPORTS: u64 = 2 * MLN_DEPOSIT_AMOUNT_LAMPORTS;
    const USDC_RESERVE_LIQUIDITY_FRACTIONAL: u64 = 2 * USDC_BORROW_AMOUNT_FRACTIONAL;

    let user_accounts_owner = Keypair::new();
    let user_transfer_authority = Keypair::new();
    let lending_market = add_lending_market(&mut test);

    let mut reserve_config = TEST_RESERVE_CONFIG;
    reserve_config.loan_to_value_ratio = 50;
    reserve_config.liquidation_threshold = 80;
    reserve_config.liquidation_bonus = 10;

    let mln_oracle = add_mln_oracle(&mut test);
    let mln_test_reserve = add_reserve(
        &mut test,
        &lending_market,
        &mln_oracle,
        &user_accounts_owner,
        AddReserveArgs {
            collateral_amount: MLN_RESERVE_COLLATERAL_LAMPORTS,
            liquidity_mint_pubkey: spl_token::native_mint::id(),
            liquidity_mint_decimals: 9,
            config: reserve_config,
            mark_fresh: true,
            ..AddReserveArgs::default()
        },
    );

    let usdc_mint = add_usdc_mint(&mut test);
    let usdc_oracle = add_usdc_oracle(&mut test);
    let usdc_test_reserve = add_reserve(
        &mut test,
        &lending_market,
        &usdc_oracle,
        &user_accounts_owner,
        AddReserveArgs {
            borrow_amount: USDC_BORROW_AMOUNT_FRACTIONAL,
            user_liquidity_amount: USDC_BORROW_AMOUNT_FRACTIONAL,
            liquidity_amount: USDC_RESERVE_LIQUIDITY_FRACTIONAL,
            liquidity_mint_pubkey: usdc_mint.pubkey,
            liquidity_mint_decimals: usdc_mint.decimals,
            config: reserve_config,
            mark_fresh: true,
            ..AddReserveArgs::default()
        },
    );

    let test_obligation = add_obligation(
        &mut test,
        &lending_market,
        &user_accounts_owner,
        AddObligationArgs {
            deposits: &[(&mln_test_reserve, MLN_DEPOSIT_AMOUNT_LAMPORTS)],
            borrows: &[(&usdc_test_reserve, USDC_BORROW_AMOUNT_FRACTIONAL)],
            ..AddObligationArgs::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = test.start().await;

    let initial_user_liquidity_balance =
        get_token_balance(&mut banks_client, usdc_test_reserve.user_liquidity_pubkey).await;
    let initial_liquidity_supply_balance =
        get_token_balance(&mut banks_client, usdc_test_reserve.liquidity_supply_pubkey).await;
    let initial_user_collateral_balance =
        get_token_balance(&mut banks_client, mln_test_reserve.user_collateral_pubkey).await;
    let initial_collateral_supply_balance =
        get_token_balance(&mut banks_client, mln_test_reserve.collateral_supply_pubkey).await;

    let mut transaction = Transaction::new_with_payer(
        &[
            approve(
                &spl_token::id(),
                &usdc_test_reserve.user_liquidity_pubkey,
                &user_transfer_authority.pubkey(),
                &user_accounts_owner.pubkey(),
                &[],
                USDC_LIQUIDATION_AMOUNT_FRACTIONAL,
            )
            .unwrap(),
            refresh_obligation(
                spl_token_lending::id(),
                test_obligation.pubkey,
                vec![mln_test_reserve.pubkey, usdc_test_reserve.pubkey],
            ),
            liquidate_obligation(
                spl_token_lending::id(),
                USDC_LIQUIDATION_AMOUNT_FRACTIONAL,
                usdc_test_reserve.user_liquidity_pubkey,
                mln_test_reserve.user_collateral_pubkey,
                usdc_test_reserve.pubkey,
                usdc_test_reserve.liquidity_supply_pubkey,
                mln_test_reserve.pubkey,
                mln_test_reserve.collateral_supply_pubkey,
                test_obligation.pubkey,
                lending_market.pubkey,
                user_transfer_authority.pubkey(),
            ),
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(
        &[&payer, &user_accounts_owner, &user_transfer_authority],
        recent_blockhash,
    );
    assert!(banks_client.process_transaction(transaction).await.is_ok());

    let user_liquidity_balance =
        get_token_balance(&mut banks_client, usdc_test_reserve.user_liquidity_pubkey).await;
    assert_eq!(
        user_liquidity_balance,
        initial_user_liquidity_balance - USDC_LIQUIDATION_AMOUNT_FRACTIONAL
    );

    let liquidity_supply_balance =
        get_token_balance(&mut banks_client, usdc_test_reserve.liquidity_supply_pubkey).await;
    assert_eq!(
        liquidity_supply_balance,
        initial_liquidity_supply_balance + USDC_LIQUIDATION_AMOUNT_FRACTIONAL
    );

    let user_collateral_balance =
        get_token_balance(&mut banks_client, mln_test_reserve.user_collateral_pubkey).await;
    assert_eq!(
        user_collateral_balance,
        initial_user_collateral_balance + MLN_LIQUIDATION_AMOUNT_LAMPORTS
    );

    let collateral_supply_balance =
        get_token_balance(&mut banks_client, mln_test_reserve.collateral_supply_pubkey).await;
    assert_eq!(
        collateral_supply_balance,
        initial_collateral_supply_balance - MLN_LIQUIDATION_AMOUNT_LAMPORTS
    );

    let obligation = test_obligation.get_state(&mut banks_client).await;
    assert_eq!(
        obligation.deposits[0].deposited_amount,
        MLN_DEPOSIT_AMOUNT_LAMPORTS - MLN_LIQUIDATION_AMOUNT_LAMPORTS
    );
    assert_eq!(
        obligation.borrows[0].borrowed_amount_wads,
        (USDC_BORROW_AMOUNT_FRACTIONAL - USDC_LIQUIDATION_AMOUNT_FRACTIONAL).into()
    )
}
