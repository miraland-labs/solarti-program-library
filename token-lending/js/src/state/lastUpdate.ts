import { struct } from '@miraland/buffer-layout';
import { bool, u64 } from '@miraland/buffer-layout-utils';

export interface LastUpdate {
    slot: bigint;
    stale: boolean;
}

/** @internal */
export const LastUpdateLayout = struct<LastUpdate>([u64('slot'), bool('stale')], 'lastUpdate');
