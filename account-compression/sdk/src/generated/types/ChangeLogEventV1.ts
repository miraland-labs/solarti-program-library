/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solarti/web3.js';
import * as beet from '@metaplex-foundation/beet';
import * as beetSolana from '@metaplex-foundation/beet-solana';

import { PathNode, pathNodeBeet } from './PathNode';
export type ChangeLogEventV1 = {
  id: web3.PublicKey;
  path: PathNode[];
  seq: beet.bignum;
  index: number;
};

/**
 * @category userTypes
 * @category generated
 */
export const changeLogEventV1Beet =
  new beet.FixableBeetArgsStruct<ChangeLogEventV1>(
    [
      ['id', beetSolana.publicKey],
      ['path', beet.array(pathNodeBeet)],
      ['seq', beet.u64],
      ['index', beet.u32],
    ],
    'ChangeLogEventV1'
  );
