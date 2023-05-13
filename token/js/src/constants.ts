import { PublicKey } from '@solarti/web3.js';

/** Address of the Solarti Token program */
export const TOKEN_PROGRAM_ID = new PublicKey('Token4Q2B47VCdUy8u3rSTMMk2bGA1k7eN8qfKSzdiM');

/** Address of the Solarti Token 2022 program */
export const TOKEN_2022_PROGRAM_ID = new PublicKey('Token8N5ecJeFxL83iFa2h7AgJ8AtufM7bbg63LrW89');

/** Address of the SPL Associated Token Account program */
export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('ATAccPjxdgWfJKKN4PmfJ55FbEDEwD8zJUwVjuL9MuHy');

/** Address of the special mint for wrapped native MLN in solarti-token */
export const NATIVE_MINT = new PublicKey('MLN1111111111111111111111111111111111111111');

/** Address of the special mint for wrapped native MLN in solarti-token-2022 */
export const NATIVE_MINT_2022 = new PublicKey('968j6eVSgVdL7NeJoEjtRD7XV9LFSt4Bt6MsAtvtAxvx');

/** Check that the token program provided is not `Tokenkeg...`, useful when using extensions */
export function programSupportsExtensions(programId: PublicKey): boolean {
    if (programId === TOKEN_PROGRAM_ID) {
        return false;
    } else {
        return true;
    }
}
