/* tslint:disable */
/* eslint-disable */

/**
 * Web / WASM 平台入口
 *
 * 当浏览器初始化 WASM 模块后，由 wasm-bindgen 自动调用此函数。
 * `index.html` 中只需执行 `init()`，无需手动调用此函数。
 *
 * 参考官方模式：`#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]`
 */
export function wasm_entry(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly wasm_entry: () => void;
    readonly slint_qt_get_widget: (a: number) => number;
    readonly send_keyboard_string_sequence: (a: number, b: number) => void;
    readonly slint_mock_elapsed_time: (a: bigint) => void;
    readonly slint_send_keyboard_char: (a: number, b: number, c: number) => void;
    readonly slint_send_mouse_click: (a: number, b: number, c: number) => void;
    readonly slint_get_mocked_time: () => bigint;
    readonly wasm_bindgen__convert__closures_____invoke__h89cbb6c393ab7ec6: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__hf7c528b290fbb449: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0e14c616c535c519: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0e14c616c535c519_3: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h2a2bf34df7a1051d: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0e14c616c535c519_5: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h2a2bf34df7a1051d_6: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0e14c616c535c519_7: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h2a2bf34df7a1051d_8: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0e14c616c535c519_9: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h2a2bf34df7a1051d_10: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0e14c616c535c519_11: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0e14c616c535c519_12: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h2a2bf34df7a1051d_13: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0e14c616c535c519_14: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h6e2ad17035bd71fa: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
