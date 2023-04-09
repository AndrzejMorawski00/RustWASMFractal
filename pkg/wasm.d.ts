/* tslint:disable */
/* eslint-disable */
/**
* @param {number} iters
* @param {number} size
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {CanvasRenderingContext2D} ctx
*/
export function generate_mandelbrot_set(iters: number, size: number, x_min: number, x_max: number, y_min: number, y_max: number, ctx: CanvasRenderingContext2D): void;
/**
*/
export class Image {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_image_free: (a: number) => void;
  readonly generate_mandelbrot_set: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
