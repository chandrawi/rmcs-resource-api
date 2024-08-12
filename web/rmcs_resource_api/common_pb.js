// source: rmcs_resource_api/common.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

var jspb = require('google-protobuf');
var goog = jspb;
var global =
    (typeof globalThis !== 'undefined' && globalThis) ||
    (typeof window !== 'undefined' && window) ||
    (typeof global !== 'undefined' && global) ||
    (typeof self !== 'undefined' && self) ||
    (function () { return this; }).call(null) ||
    Function('return this')();

goog.exportSymbol('proto.common.DataType', null, global);
/**
 * @enum {number}
 */
proto.common.DataType = {
  NULLD: 0,
  I8: 1,
  I16: 2,
  I32: 3,
  I64: 4,
  I128: 5,
  U8: 6,
  U16: 7,
  U32: 8,
  U64: 9,
  U128: 10,
  F32: 12,
  F64: 13,
  BOOL: 15,
  CHAR: 16,
  STRING: 17,
  BYTES: 18
};

goog.object.extend(exports, proto.common);
