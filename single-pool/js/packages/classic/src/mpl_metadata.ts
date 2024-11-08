import { PublicKey } from '@miraland/web3.js';
import { Buffer } from 'buffer';

export const MPL_METADATA_PROGRAM_ID = new PublicKey('Meta88XpDHcSJZDFiHop6c9sXaufkZX5depkZyrYBWv');

export function findMplMetadataAddress(poolMintAddress: PublicKey) {
  const [publicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from('metadata'), MPL_METADATA_PROGRAM_ID.toBuffer(), poolMintAddress.toBuffer()],
    MPL_METADATA_PROGRAM_ID,
  );
  return publicKey;
}
