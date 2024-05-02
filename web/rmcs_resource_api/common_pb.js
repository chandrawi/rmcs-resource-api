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

goog.exportSymbol('proto.common.ConfigType', null, global);
goog.exportSymbol('proto.common.DataIndexing', null, global);
goog.exportSymbol('proto.common.DataType', null, global);
/**
 * @enum {number}
 */
proto.common.DataIndexing = {
  TIMESTAMP: 0,
  INDEX: 1,
  TIMESTAMP_INDEX: 2
};

/**
 * @enum {number}
 */
proto.common.DataType = {
  NULLD: 0,
  I8: 1,
  I16: 2,
  I32: 3,
  I64: 4,
  U8: 5,
  U16: 6,
  U32: 7,
  U64: 8,
  F32: 9,
  F64: 10,
  CHAR: 11,
  BOOL: 12
};

/**
 * @enum {number}
 */
proto.common.ConfigType = {
  NULLC: 0,
  INT: 1,
  FLOAT: 2,
  STR: 3
};

goog.object.extend(exports, proto.common);
