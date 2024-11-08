import { TOKEN_PROGRAM_ID } from '@solarti/solarti-token';
import { PublicKey, SYSVAR_CLOCK_PUBKEY, SYSVAR_RENT_PUBKEY, TransactionInstruction } from '@solarti/web3.js';
import { struct, u8 } from '@miraland/buffer-layout';
import { LENDING_PROGRAM_ID } from '../constants';
import { LendingInstruction } from './instruction';

interface Data {
    instruction: number;
}

const DataLayout = struct<Data>([u8('instruction')]);

export const initObligationInstruction = (
    obligation: PublicKey,
    lendingMarket: PublicKey,
    obligationOwner: PublicKey
): TransactionInstruction => {
    const data = Buffer.alloc(DataLayout.span);
    DataLayout.encode({ instruction: LendingInstruction.InitObligation }, data);

    const keys = [
        { pubkey: obligation, isSigner: false, isWritable: true },
        { pubkey: lendingMarket, isSigner: false, isWritable: false },
        { pubkey: obligationOwner, isSigner: true, isWritable: false },
        { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ];

    return new TransactionInstruction({
        keys,
        programId: LENDING_PROGRAM_ID,
        data,
    });
};
