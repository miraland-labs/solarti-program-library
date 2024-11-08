import { PublicKey } from '@solarti/web3.js';
import * as beetMiraland from '@metaplex-foundation/beet-miraland';
import * as beet from '@metaplex-foundation/beet';

/**
 * Canopy fields necessary for deserializing an on-chain Path
 * used in an {@link ConcurrentMerkleTree}
 */
export type Path = {
  proof: PublicKey[];
  leaf: PublicKey;
  index: number; // u32
  _padding: number; // u32
};

/**
 * Factory function for generating a `beet` that can deserialize
 * an on-chain {@link Path}
 *
 * @param maxDepth
 * @returns
 */
export const pathBeetFactory = (maxDepth: number) => {
  return new beet.BeetArgsStruct<Path>(
    [
      ['proof', beet.uniformFixedSizeArray(beetMiraland.publicKey, maxDepth)],
      ['leaf', beetMiraland.publicKey],
      ['index', beet.u32],
      ['_padding', beet.u32],
    ],
    'Path'
  );
};
