/**
 * @fileoverview gRPC-Web generated client stub for log
 * @enhanceable
 * @public
 */

// Code generated by protoc-gen-grpc-web. DO NOT EDIT.
// versions:
// 	protoc-gen-grpc-web v1.5.0
// 	protoc              v5.26.1
// source: rmcs_resource_api/log.proto


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');


var rmcs_resource_api_common_pb = require('../rmcs_resource_api/common_pb.js')
const proto = {};
proto.log = require('./log_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.log.LogServiceClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname.replace(/\/+$/, '');

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.log.LogServicePromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname.replace(/\/+$/, '');

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.log.LogId,
 *   !proto.log.LogReadResponse>}
 */
const methodDescriptor_LogService_ReadLog = new grpc.web.MethodDescriptor(
  '/log.LogService/ReadLog',
  grpc.web.MethodType.UNARY,
  proto.log.LogId,
  proto.log.LogReadResponse,
  /**
   * @param {!proto.log.LogId} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.log.LogReadResponse.deserializeBinary
);


/**
 * @param {!proto.log.LogId} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.log.LogReadResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.log.LogReadResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.log.LogServiceClient.prototype.readLog =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/log.LogService/ReadLog',
      request,
      metadata || {},
      methodDescriptor_LogService_ReadLog,
      callback);
};


/**
 * @param {!proto.log.LogId} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.log.LogReadResponse>}
 *     Promise that resolves to the response
 */
proto.log.LogServicePromiseClient.prototype.readLog =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/log.LogService/ReadLog',
      request,
      metadata || {},
      methodDescriptor_LogService_ReadLog);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.log.LogTime,
 *   !proto.log.LogListResponse>}
 */
const methodDescriptor_LogService_ListLogByTime = new grpc.web.MethodDescriptor(
  '/log.LogService/ListLogByTime',
  grpc.web.MethodType.UNARY,
  proto.log.LogTime,
  proto.log.LogListResponse,
  /**
   * @param {!proto.log.LogTime} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.log.LogListResponse.deserializeBinary
);


/**
 * @param {!proto.log.LogTime} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.log.LogListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.log.LogListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.log.LogServiceClient.prototype.listLogByTime =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/log.LogService/ListLogByTime',
      request,
      metadata || {},
      methodDescriptor_LogService_ListLogByTime,
      callback);
};


/**
 * @param {!proto.log.LogTime} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.log.LogListResponse>}
 *     Promise that resolves to the response
 */
proto.log.LogServicePromiseClient.prototype.listLogByTime =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/log.LogService/ListLogByTime',
      request,
      metadata || {},
      methodDescriptor_LogService_ListLogByTime);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.log.LogTime,
 *   !proto.log.LogListResponse>}
 */
const methodDescriptor_LogService_ListLogByLastTime = new grpc.web.MethodDescriptor(
  '/log.LogService/ListLogByLastTime',
  grpc.web.MethodType.UNARY,
  proto.log.LogTime,
  proto.log.LogListResponse,
  /**
   * @param {!proto.log.LogTime} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.log.LogListResponse.deserializeBinary
);


/**
 * @param {!proto.log.LogTime} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.log.LogListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.log.LogListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.log.LogServiceClient.prototype.listLogByLastTime =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/log.LogService/ListLogByLastTime',
      request,
      metadata || {},
      methodDescriptor_LogService_ListLogByLastTime,
      callback);
};


/**
 * @param {!proto.log.LogTime} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.log.LogListResponse>}
 *     Promise that resolves to the response
 */
proto.log.LogServicePromiseClient.prototype.listLogByLastTime =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/log.LogService/ListLogByLastTime',
      request,
      metadata || {},
      methodDescriptor_LogService_ListLogByLastTime);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.log.LogRange,
 *   !proto.log.LogListResponse>}
 */
const methodDescriptor_LogService_ListLogByRangeTime = new grpc.web.MethodDescriptor(
  '/log.LogService/ListLogByRangeTime',
  grpc.web.MethodType.UNARY,
  proto.log.LogRange,
  proto.log.LogListResponse,
  /**
   * @param {!proto.log.LogRange} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.log.LogListResponse.deserializeBinary
);


/**
 * @param {!proto.log.LogRange} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.log.LogListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.log.LogListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.log.LogServiceClient.prototype.listLogByRangeTime =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/log.LogService/ListLogByRangeTime',
      request,
      metadata || {},
      methodDescriptor_LogService_ListLogByRangeTime,
      callback);
};


/**
 * @param {!proto.log.LogRange} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.log.LogListResponse>}
 *     Promise that resolves to the response
 */
proto.log.LogServicePromiseClient.prototype.listLogByRangeTime =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/log.LogService/ListLogByRangeTime',
      request,
      metadata || {},
      methodDescriptor_LogService_ListLogByRangeTime);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.log.LogSchema,
 *   !proto.log.LogChangeResponse>}
 */
const methodDescriptor_LogService_CreateLog = new grpc.web.MethodDescriptor(
  '/log.LogService/CreateLog',
  grpc.web.MethodType.UNARY,
  proto.log.LogSchema,
  proto.log.LogChangeResponse,
  /**
   * @param {!proto.log.LogSchema} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.log.LogChangeResponse.deserializeBinary
);


/**
 * @param {!proto.log.LogSchema} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.log.LogChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.log.LogChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.log.LogServiceClient.prototype.createLog =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/log.LogService/CreateLog',
      request,
      metadata || {},
      methodDescriptor_LogService_CreateLog,
      callback);
};


/**
 * @param {!proto.log.LogSchema} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.log.LogChangeResponse>}
 *     Promise that resolves to the response
 */
proto.log.LogServicePromiseClient.prototype.createLog =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/log.LogService/CreateLog',
      request,
      metadata || {},
      methodDescriptor_LogService_CreateLog);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.log.LogUpdate,
 *   !proto.log.LogChangeResponse>}
 */
const methodDescriptor_LogService_UpdateLog = new grpc.web.MethodDescriptor(
  '/log.LogService/UpdateLog',
  grpc.web.MethodType.UNARY,
  proto.log.LogUpdate,
  proto.log.LogChangeResponse,
  /**
   * @param {!proto.log.LogUpdate} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.log.LogChangeResponse.deserializeBinary
);


/**
 * @param {!proto.log.LogUpdate} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.log.LogChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.log.LogChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.log.LogServiceClient.prototype.updateLog =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/log.LogService/UpdateLog',
      request,
      metadata || {},
      methodDescriptor_LogService_UpdateLog,
      callback);
};


/**
 * @param {!proto.log.LogUpdate} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.log.LogChangeResponse>}
 *     Promise that resolves to the response
 */
proto.log.LogServicePromiseClient.prototype.updateLog =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/log.LogService/UpdateLog',
      request,
      metadata || {},
      methodDescriptor_LogService_UpdateLog);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.log.LogId,
 *   !proto.log.LogChangeResponse>}
 */
const methodDescriptor_LogService_DeleteLog = new grpc.web.MethodDescriptor(
  '/log.LogService/DeleteLog',
  grpc.web.MethodType.UNARY,
  proto.log.LogId,
  proto.log.LogChangeResponse,
  /**
   * @param {!proto.log.LogId} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.log.LogChangeResponse.deserializeBinary
);


/**
 * @param {!proto.log.LogId} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.log.LogChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.log.LogChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.log.LogServiceClient.prototype.deleteLog =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/log.LogService/DeleteLog',
      request,
      metadata || {},
      methodDescriptor_LogService_DeleteLog,
      callback);
};


/**
 * @param {!proto.log.LogId} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.log.LogChangeResponse>}
 *     Promise that resolves to the response
 */
proto.log.LogServicePromiseClient.prototype.deleteLog =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/log.LogService/DeleteLog',
      request,
      metadata || {},
      methodDescriptor_LogService_DeleteLog);
};


module.exports = proto.log;

