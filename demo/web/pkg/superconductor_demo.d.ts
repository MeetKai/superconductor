/* tslint:disable */
/* eslint-disable */
/**
*/
export function main(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: () => void;
  readonly ZSTD_compress2: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_decompressDCtx: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_isError: (a: number) => number;
  readonly ZSTD_versionNumber: () => number;
  readonly ZSTD_versionString: () => number;
  readonly ZSTD_minCLevel: () => number;
  readonly ZSTD_maxCLevel: () => number;
  readonly ZSTD_getDecompressedSize: (a: number, b: number) => number;
  readonly ZSTD_compressBound: (a: number) => number;
  readonly ZSTD_createCCtx: () => number;
  readonly ZSTD_initCStream: (a: number, b: number) => number;
  readonly ZSTD_CCtx_loadDictionary: (a: number, b: number, c: number) => number;
  readonly ZSTD_CCtx_refCDict: (a: number, b: number) => number;
  readonly ZSTD_CCtx_refPrefix: (a: number, b: number, c: number) => number;
  readonly ZSTD_sizeof_CCtx: (a: number) => number;
  readonly ZSTD_CCtx_reset: (a: number, b: number) => number;
  readonly ZSTD_CCtx_setParameter: (a: number, b: number, c: number) => number;
  readonly ZSTD_CCtx_setPledgedSrcSize: (a: number, b: number) => number;
  readonly ZSTD_freeCCtx: (a: number) => number;
  readonly ZSTD_getErrorName: (a: number) => number;
  readonly ZSTD_compressCCtx: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly ZSTD_createDCtx: () => number;
  readonly ZSTD_initDStream: (a: number) => number;
  readonly ZSTD_DCtx_reset: (a: number, b: number) => number;
  readonly ZSTD_DCtx_loadDictionary: (a: number, b: number, c: number) => number;
  readonly ZSTD_DCtx_refDDict: (a: number, b: number) => number;
  readonly ZSTD_DCtx_refPrefix: (a: number, b: number, c: number) => number;
  readonly ZSTD_DCtx_setParameter: (a: number, b: number, c: number) => number;
  readonly ZSTD_sizeof_DCtx: (a: number) => number;
  readonly ZSTD_freeDCtx: (a: number) => number;
  readonly ZSTD_compress_usingDict: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly ZSTD_decompress_usingDict: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly ZSTD_createCDict: (a: number, b: number, c: number) => number;
  readonly ZSTD_sizeof_CDict: (a: number) => number;
  readonly ZSTD_getDictID_fromCDict: (a: number) => number;
  readonly ZSTD_freeCDict: (a: number) => number;
  readonly ZSTD_compress_usingCDict: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly ZSTD_createDDict: (a: number, b: number) => number;
  readonly ZSTD_sizeof_DDict: (a: number) => number;
  readonly ZSTD_getDictID_fromDDict: (a: number) => number;
  readonly ZSTD_freeDDict: (a: number) => number;
  readonly ZSTD_decompress_usingDDict: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly ZSTD_CStreamInSize: () => number;
  readonly ZSTD_CStreamOutSize: () => number;
  readonly ZSTD_DStreamInSize: () => number;
  readonly ZSTD_DStreamOutSize: () => number;
  readonly ZSTD_findFrameCompressedSize: (a: number, b: number) => number;
  readonly ZSTD_getFrameContentSize: (a: number, b: number) => number;
  readonly ZSTD_getDictID_fromDict: (a: number, b: number) => number;
  readonly ZSTD_getDictID_fromFrame: (a: number, b: number) => number;
  readonly rust_zstd_wasm_shim_malloc: (a: number) => number;
  readonly rust_zstd_wasm_shim_calloc: (a: number, b: number) => number;
  readonly rust_zstd_wasm_shim_free: (a: number) => void;
  readonly rust_zstd_wasm_shim_memcpy: (a: number, b: number, c: number) => number;
  readonly rust_zstd_wasm_shim_memmove: (a: number, b: number, c: number) => number;
  readonly rust_zstd_wasm_shim_memset: (a: number, b: number, c: number) => number;
  readonly ZSTD_createCCtx_advanced: (a: number) => number;
  readonly ZSTD_initStaticCCtx: (a: number, b: number) => number;
  readonly ZSTD_sizeof_CStream: (a: number) => number;
  readonly ZSTD_createCCtxParams: () => number;
  readonly ZSTD_freeCCtxParams: (a: number) => number;
  readonly ZSTD_CCtxParams_reset: (a: number) => number;
  readonly ZSTD_CCtxParams_init: (a: number, b: number) => number;
  readonly ZSTD_CCtxParams_init_advanced: (a: number, b: number) => number;
  readonly ZSTD_checkCParams: (a: number) => number;
  readonly ZSTD_cParam_getBounds: (a: number, b: number) => void;
  readonly ZSTD_CCtxParams_setParameter: (a: number, b: number, c: number) => number;
  readonly ZSTD_CCtx_getParameter: (a: number, b: number, c: number) => number;
  readonly ZSTD_CCtxParams_getParameter: (a: number, b: number, c: number) => number;
  readonly ZSTD_CCtx_setParametersUsingCCtxParams: (a: number, b: number) => number;
  readonly ZSTD_CCtx_loadDictionary_advanced: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_CCtx_loadDictionary_byReference: (a: number, b: number, c: number) => number;
  readonly ZSTD_CCtx_refThreadPool: (a: number, b: number) => number;
  readonly ZSTD_CCtx_refPrefix_advanced: (a: number, b: number, c: number, d: number) => number;
  readonly ZSTD_adjustCParams: (a: number, b: number, c: number, d: number) => void;
  readonly ZSTD_estimateCCtxSize_usingCCtxParams: (a: number) => number;
  readonly ZSTD_estimateCCtxSize_usingCParams: (a: number) => number;
  readonly ZSTD_estimateCCtxSize: (a: number) => number;
  readonly ZSTD_estimateCStreamSize_usingCCtxParams: (a: number) => number;
  readonly ZSTD_estimateCStreamSize_usingCParams: (a: number) => number;
  readonly ZSTD_estimateCStreamSize: (a: number) => number;
  readonly ZSTD_getFrameProgression: (a: number, b: number) => void;
  readonly ZSTD_toFlushNow: (a: number) => number;
  readonly ZSTD_copyCCtx: (a: number, b: number, c: number) => number;
  readonly ZSTD_generateSequences: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_compressStream2: (a: number, b: number, c: number, d: number) => number;
  readonly ZSTD_mergeBlockDelimiters: (a: number, b: number) => number;
  readonly ZSTD_writeSkippableFrame: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_compressContinue: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_getBlockSize: (a: number) => number;
  readonly ZSTD_compressBlock: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_compressBegin_advanced: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_compressBegin_usingDict: (a: number, b: number, c: number, d: number) => number;
  readonly ZSTD_compressBegin: (a: number, b: number) => number;
  readonly ZSTD_compressEnd: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_compress_advanced: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly ZSTD_compress: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_estimateCDictSize_advanced: (a: number, b: number, c: number) => number;
  readonly ZSTD_estimateCDictSize: (a: number, b: number) => number;
  readonly ZSTD_createCDict_advanced: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly ZSTD_createCDict_advanced2: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly ZSTD_createCDict_byReference: (a: number, b: number, c: number) => number;
  readonly ZSTD_initStaticCDict: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly ZSTD_compressBegin_usingCDict_advanced: (a: number, b: number, c: number, d: number) => number;
  readonly ZSTD_compressBegin_usingCDict: (a: number, b: number) => number;
  readonly ZSTD_compress_usingCDict_advanced: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly ZSTD_createCStream: () => number;
  readonly ZSTD_createCStream_advanced: (a: number) => number;
  readonly ZSTD_initStaticCStream: (a: number, b: number) => number;
  readonly ZSTD_freeCStream: (a: number) => number;
  readonly ZSTD_resetCStream: (a: number, b: number) => number;
  readonly ZSTD_initCStream_usingCDict_advanced: (a: number, b: number, c: number, d: number) => number;
  readonly ZSTD_initCStream_usingCDict: (a: number, b: number) => number;
  readonly ZSTD_initCStream_advanced: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_initCStream_usingDict: (a: number, b: number, c: number, d: number) => number;
  readonly ZSTD_initCStream_srcSize: (a: number, b: number, c: number) => number;
  readonly ZSTD_compressStream: (a: number, b: number, c: number) => number;
  readonly ZSTD_compressStream2_simpleArgs: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly ZSTD_compressSequences: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly ZSTD_flushStream: (a: number, b: number) => number;
  readonly ZSTD_endStream: (a: number, b: number) => number;
  readonly ZSTD_defaultCLevel: () => number;
  readonly ZSTD_getCParams: (a: number, b: number, c: number, d: number) => void;
  readonly ZSTD_getParams: (a: number, b: number, c: number, d: number) => void;
  readonly ZSTD_createDDict_advanced: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_createDDict_byReference: (a: number, b: number) => number;
  readonly ZSTD_initStaticDDict: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly ZSTD_estimateDDictSize: (a: number, b: number) => number;
  readonly ZSTD_estimateDCtxSize: () => number;
  readonly ZSTD_initStaticDCtx: (a: number, b: number) => number;
  readonly ZSTD_createDCtx_advanced: (a: number) => number;
  readonly ZSTD_copyDCtx: (a: number, b: number) => void;
  readonly ZSTD_isFrame: (a: number, b: number) => number;
  readonly ZSTD_isSkippableFrame: (a: number, b: number) => number;
  readonly ZSTD_frameHeaderSize: (a: number, b: number) => number;
  readonly ZSTD_getFrameHeader_advanced: (a: number, b: number, c: number, d: number) => number;
  readonly ZSTD_getFrameHeader: (a: number, b: number, c: number) => number;
  readonly ZSTD_readSkippableFrame: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_findDecompressedSize: (a: number, b: number) => number;
  readonly ZSTD_decompressBound: (a: number, b: number) => number;
  readonly ZSTD_insertBlock: (a: number, b: number, c: number) => number;
  readonly ZSTD_decompressBegin_usingDict: (a: number, b: number, c: number) => number;
  readonly ZSTD_decompress: (a: number, b: number, c: number, d: number) => number;
  readonly ZSTD_nextSrcSizeToDecompress: (a: number) => number;
  readonly ZSTD_nextInputType: (a: number) => number;
  readonly ZSTD_decompressContinue: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_decompressBegin: (a: number) => number;
  readonly ZSTD_decompressBegin_usingDDict: (a: number, b: number) => number;
  readonly ZSTD_createDStream: () => number;
  readonly ZSTD_initStaticDStream: (a: number, b: number) => number;
  readonly ZSTD_createDStream_advanced: (a: number) => number;
  readonly ZSTD_freeDStream: (a: number) => number;
  readonly ZSTD_DCtx_loadDictionary_advanced: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ZSTD_DCtx_loadDictionary_byReference: (a: number, b: number, c: number) => number;
  readonly ZSTD_DCtx_refPrefix_advanced: (a: number, b: number, c: number, d: number) => number;
  readonly ZSTD_initDStream_usingDict: (a: number, b: number, c: number) => number;
  readonly ZSTD_initDStream_usingDDict: (a: number, b: number) => number;
  readonly ZSTD_resetDStream: (a: number) => number;
  readonly ZSTD_DCtx_setMaxWindowSize: (a: number, b: number) => number;
  readonly ZSTD_dParam_getBounds: (a: number, b: number) => void;
  readonly ZSTD_DCtx_setFormat: (a: number, b: number) => number;
  readonly ZSTD_DCtx_getParameter: (a: number, b: number, c: number) => number;
  readonly ZSTD_sizeof_DStream: (a: number) => number;
  readonly ZSTD_decodingBufferSize_min: (a: number, b: number) => number;
  readonly ZSTD_estimateDStreamSize: (a: number) => number;
  readonly ZSTD_estimateDStreamSize_fromFrame: (a: number, b: number) => number;
  readonly ZSTD_decompressStream: (a: number, b: number, c: number) => number;
  readonly ZSTD_decompressStream_simpleArgs: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly ZSTD_decompressBlock: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly wgpu_render_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_blend_constant: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_scissor_rect: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_viewport: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly wgpu_render_pass_set_stencil_reference: (a: number, b: number) => void;
  readonly wgpu_render_pass_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_render_pass_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_compute_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_compute_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_compute_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_set_push_constant: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_render_bundle_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_bundle_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_bundle_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_pop_debug_group: (a: number) => void;
  readonly wgpu_render_bundle_insert_debug_marker: (a: number, b: number) => void;
  readonly wgpu_render_bundle_push_debug_group: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_execute_bundles: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h73857af794b0ab6c: (a: number, b: number, c: number, d: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8fac3659e0e4ae26: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h321469236e57f8ab: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
