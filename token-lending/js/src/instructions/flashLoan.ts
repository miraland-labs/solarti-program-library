import { TOKEN_PROGRAM_ID } from '@solarti/solarti-token';
import { PublicKey, TransactionInstruction } from '@solarti/web3.js';
import { struct, u8 } from '@miraland/buffer-layout';
import { LENDING_PROGRAM_ID } from '../constants';
import { u64 } from '@miraland/buffer-layout-utils';
import { LendingInstruction } from './instruction';

interface Data {
    instruction: number;
    liquidityAmount: bigint;
}

const DataLayout = struct<Data>([u8('instruction'), u64('liquidityAmount')]);

export const flashLoanInstruction = (
    liquidityAmount: number | bigint,
    sourceLiquidity: PublicKey,
    destinationLiquidity: PublicKey,
    liquidityReserve: PublicKey,
    flashLoanFeeReceiver: PublicKey,
    hostFeeReceiver: PublicKey,
    lendingMarket: PublicKey,
    lendingMarketAuthority: PublicKey,
    flashLoanProgram: PublicKey,
    transferAuthority: PublicKey
): TransactionInstruction => {
    const data = Buffer.alloc(DataLayout.span);
    DataLayout.encode(
        {
            instruction: LendingInstruction.FlashLoan,
            liquidityAmount: BigInt(liquidityAmount),
        },
        data
    );

    const keys = [
        { pubkey: sourceLiquidity, isSigner: false, isWritable: true },
        { pubkey: destinationLiquidity, isSigner: false, isWritable: true },
        { pubkey: liquidityReserve, isSigner: false, isWritable: true },
        { pubkey: flashLoanFeeReceiver, isSigner: false, isWritable: true },
        { pubkey: hostFeeReceiver, isSigner: false, isWritable: true },
        { pubkey: lendingMarket, isSigner: false, isWritable: false },
        { pubkey: lendingMarketAuthority, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: flashLoanProgram, isSigner: false, isWritable: false },
        { pubkey: transferAuthority, isSigner: true, isWritable: false },
    ];

    return new TransactionInstruction({
        keys,
        programId: LENDING_PROGRAM_ID,
        data,
    });
};
