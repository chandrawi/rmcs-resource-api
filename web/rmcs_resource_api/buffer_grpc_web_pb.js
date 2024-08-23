/**
 * @fileoverview gRPC-Web generated client stub for buffer
 * @enhanceable
 * @public
 */

// Code generated by protoc-gen-grpc-web. DO NOT EDIT.
// versions:
// 	protoc-gen-grpc-web v1.5.0
// 	protoc              v5.27.3
// source: rmcs_resource_api/buffer.proto


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');


var rmcs_resource_api_common_pb = require('../rmcs_resource_api/common_pb.js')
const proto = {};
proto.buffer = require('./buffer_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.buffer.BufferServiceClient =
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
proto.buffer.BufferServicePromiseClient =
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
 *   !proto.buffer.BufferId,
 *   !proto.buffer.BufferReadResponse>}
 */
const methodDescriptor_BufferService_ReadBuffer = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/ReadBuffer',
  grpc.web.MethodType.UNARY,
  proto.buffer.BufferId,
  proto.buffer.BufferReadResponse,
  /**
   * @param {!proto.buffer.BufferId} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferReadResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BufferId} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferReadResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferReadResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.readBuffer =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/ReadBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_ReadBuffer,
      callback);
};


/**
 * @param {!proto.buffer.BufferId} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferReadResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.readBuffer =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/ReadBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_ReadBuffer);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BufferTime,
 *   !proto.buffer.BufferReadResponse>}
 */
const methodDescriptor_BufferService_ReadBufferByTime = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/ReadBufferByTime',
  grpc.web.MethodType.UNARY,
  proto.buffer.BufferTime,
  proto.buffer.BufferReadResponse,
  /**
   * @param {!proto.buffer.BufferTime} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferReadResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BufferTime} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferReadResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferReadResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.readBufferByTime =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/ReadBufferByTime',
      request,
      metadata || {},
      methodDescriptor_BufferService_ReadBufferByTime,
      callback);
};


/**
 * @param {!proto.buffer.BufferTime} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferReadResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.readBufferByTime =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/ReadBufferByTime',
      request,
      metadata || {},
      methodDescriptor_BufferService_ReadBufferByTime);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BufferSelector,
 *   !proto.buffer.BufferReadResponse>}
 */
const methodDescriptor_BufferService_ReadBufferFirst = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/ReadBufferFirst',
  grpc.web.MethodType.UNARY,
  proto.buffer.BufferSelector,
  proto.buffer.BufferReadResponse,
  /**
   * @param {!proto.buffer.BufferSelector} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferReadResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BufferSelector} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferReadResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferReadResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.readBufferFirst =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/ReadBufferFirst',
      request,
      metadata || {},
      methodDescriptor_BufferService_ReadBufferFirst,
      callback);
};


/**
 * @param {!proto.buffer.BufferSelector} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferReadResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.readBufferFirst =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/ReadBufferFirst',
      request,
      metadata || {},
      methodDescriptor_BufferService_ReadBufferFirst);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BufferSelector,
 *   !proto.buffer.BufferReadResponse>}
 */
const methodDescriptor_BufferService_ReadBufferLast = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/ReadBufferLast',
  grpc.web.MethodType.UNARY,
  proto.buffer.BufferSelector,
  proto.buffer.BufferReadResponse,
  /**
   * @param {!proto.buffer.BufferSelector} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferReadResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BufferSelector} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferReadResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferReadResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.readBufferLast =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/ReadBufferLast',
      request,
      metadata || {},
      methodDescriptor_BufferService_ReadBufferLast,
      callback);
};


/**
 * @param {!proto.buffer.BufferSelector} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferReadResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.readBufferLast =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/ReadBufferLast',
      request,
      metadata || {},
      methodDescriptor_BufferService_ReadBufferLast);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BuffersSelector,
 *   !proto.buffer.BufferListResponse>}
 */
const methodDescriptor_BufferService_ListBufferFirst = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/ListBufferFirst',
  grpc.web.MethodType.UNARY,
  proto.buffer.BuffersSelector,
  proto.buffer.BufferListResponse,
  /**
   * @param {!proto.buffer.BuffersSelector} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferListResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BuffersSelector} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.listBufferFirst =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/ListBufferFirst',
      request,
      metadata || {},
      methodDescriptor_BufferService_ListBufferFirst,
      callback);
};


/**
 * @param {!proto.buffer.BuffersSelector} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferListResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.listBufferFirst =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/ListBufferFirst',
      request,
      metadata || {},
      methodDescriptor_BufferService_ListBufferFirst);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BuffersSelector,
 *   !proto.buffer.BufferListResponse>}
 */
const methodDescriptor_BufferService_ListBufferFirstOffset = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/ListBufferFirstOffset',
  grpc.web.MethodType.UNARY,
  proto.buffer.BuffersSelector,
  proto.buffer.BufferListResponse,
  /**
   * @param {!proto.buffer.BuffersSelector} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferListResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BuffersSelector} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.listBufferFirstOffset =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/ListBufferFirstOffset',
      request,
      metadata || {},
      methodDescriptor_BufferService_ListBufferFirstOffset,
      callback);
};


/**
 * @param {!proto.buffer.BuffersSelector} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferListResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.listBufferFirstOffset =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/ListBufferFirstOffset',
      request,
      metadata || {},
      methodDescriptor_BufferService_ListBufferFirstOffset);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BuffersSelector,
 *   !proto.buffer.BufferListResponse>}
 */
const methodDescriptor_BufferService_ListBufferLast = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/ListBufferLast',
  grpc.web.MethodType.UNARY,
  proto.buffer.BuffersSelector,
  proto.buffer.BufferListResponse,
  /**
   * @param {!proto.buffer.BuffersSelector} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferListResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BuffersSelector} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.listBufferLast =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/ListBufferLast',
      request,
      metadata || {},
      methodDescriptor_BufferService_ListBufferLast,
      callback);
};


/**
 * @param {!proto.buffer.BuffersSelector} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferListResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.listBufferLast =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/ListBufferLast',
      request,
      metadata || {},
      methodDescriptor_BufferService_ListBufferLast);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BuffersSelector,
 *   !proto.buffer.BufferListResponse>}
 */
const methodDescriptor_BufferService_ListBufferLastOffset = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/ListBufferLastOffset',
  grpc.web.MethodType.UNARY,
  proto.buffer.BuffersSelector,
  proto.buffer.BufferListResponse,
  /**
   * @param {!proto.buffer.BuffersSelector} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferListResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BuffersSelector} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.listBufferLastOffset =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/ListBufferLastOffset',
      request,
      metadata || {},
      methodDescriptor_BufferService_ListBufferLastOffset,
      callback);
};


/**
 * @param {!proto.buffer.BuffersSelector} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferListResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.listBufferLastOffset =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/ListBufferLastOffset',
      request,
      metadata || {},
      methodDescriptor_BufferService_ListBufferLastOffset);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BufferSchema,
 *   !proto.buffer.BufferCreateResponse>}
 */
const methodDescriptor_BufferService_CreateBuffer = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/CreateBuffer',
  grpc.web.MethodType.UNARY,
  proto.buffer.BufferSchema,
  proto.buffer.BufferCreateResponse,
  /**
   * @param {!proto.buffer.BufferSchema} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferCreateResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BufferSchema} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferCreateResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferCreateResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.createBuffer =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/CreateBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_CreateBuffer,
      callback);
};


/**
 * @param {!proto.buffer.BufferSchema} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferCreateResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.createBuffer =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/CreateBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_CreateBuffer);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BufferUpdate,
 *   !proto.buffer.BufferChangeResponse>}
 */
const methodDescriptor_BufferService_UpdateBuffer = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/UpdateBuffer',
  grpc.web.MethodType.UNARY,
  proto.buffer.BufferUpdate,
  proto.buffer.BufferChangeResponse,
  /**
   * @param {!proto.buffer.BufferUpdate} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferChangeResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BufferUpdate} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.updateBuffer =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/UpdateBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_UpdateBuffer,
      callback);
};


/**
 * @param {!proto.buffer.BufferUpdate} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferChangeResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.updateBuffer =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/UpdateBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_UpdateBuffer);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BufferId,
 *   !proto.buffer.BufferChangeResponse>}
 */
const methodDescriptor_BufferService_DeleteBuffer = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/DeleteBuffer',
  grpc.web.MethodType.UNARY,
  proto.buffer.BufferId,
  proto.buffer.BufferChangeResponse,
  /**
   * @param {!proto.buffer.BufferId} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferChangeResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BufferId} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.deleteBuffer =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/DeleteBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_DeleteBuffer,
      callback);
};


/**
 * @param {!proto.buffer.BufferId} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferChangeResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.deleteBuffer =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/DeleteBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_DeleteBuffer);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.buffer.BufferCount,
 *   !proto.buffer.BufferCountResponse>}
 */
const methodDescriptor_BufferService_CountBuffer = new grpc.web.MethodDescriptor(
  '/buffer.BufferService/CountBuffer',
  grpc.web.MethodType.UNARY,
  proto.buffer.BufferCount,
  proto.buffer.BufferCountResponse,
  /**
   * @param {!proto.buffer.BufferCount} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.buffer.BufferCountResponse.deserializeBinary
);


/**
 * @param {!proto.buffer.BufferCount} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.buffer.BufferCountResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.buffer.BufferCountResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.buffer.BufferServiceClient.prototype.countBuffer =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/buffer.BufferService/CountBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_CountBuffer,
      callback);
};


/**
 * @param {!proto.buffer.BufferCount} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.buffer.BufferCountResponse>}
 *     Promise that resolves to the response
 */
proto.buffer.BufferServicePromiseClient.prototype.countBuffer =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/buffer.BufferService/CountBuffer',
      request,
      metadata || {},
      methodDescriptor_BufferService_CountBuffer);
};


module.exports = proto.buffer;

