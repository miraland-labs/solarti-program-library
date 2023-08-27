declare module 'buffer-layout' {
  // MI
  // export class Layout {}
  // MI, copied from anchor/ts/packages/borsh/types/buffer-layout/index.d.ts
  export class Layout<T = any> {
    span: number;
    property?: string;

    constructor(span: number, property?: string);

    decode(b: Buffer | string, offset?: number): T;
    encode(src: T, b: Buffer, offset?: number): number;
    getSpan(b: Buffer, offset?: number): number;
    replicate(name: string): this;
  }
  export class UInt {}
  /* eslint-disable  @typescript-eslint/no-unused-vars */
  export function struct<T>(fields: any, property?: string, decodePrefixes?: boolean): any;
  export function s32(property?: string): UInt;
  export function u32(property?: string): UInt;
  export function s16(property?: string): UInt;
  export function u16(property?: string): UInt;
  export function s8(property?: string): UInt;
  export function u8(property?: string): UInt;
  export function f32(property?: string): Layout<number>;
  export function f64(property?: string): Layout<number>;
}
