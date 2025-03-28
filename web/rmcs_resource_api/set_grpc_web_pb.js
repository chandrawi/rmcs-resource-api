/**
 * @fileoverview gRPC-Web generated client stub for set
 * @enhanceable
 * @public
 */

// Code generated by protoc-gen-grpc-web. DO NOT EDIT.
// versions:
// 	protoc-gen-grpc-web v1.5.0
// 	protoc              v6.30.1
// source: rmcs_resource_api/set.proto


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');

const proto = {};
proto.set = require('./set_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.set.SetServiceClient =
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
proto.set.SetServicePromiseClient =
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
 *   !proto.set.SetId,
 *   !proto.set.SetReadResponse>}
 */
const methodDescriptor_SetService_ReadSet = new grpc.web.MethodDescriptor(
  '/set.SetService/ReadSet',
  grpc.web.MethodType.UNARY,
  proto.set.SetId,
  proto.set.SetReadResponse,
  /**
   * @param {!proto.set.SetId} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetReadResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetId} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetReadResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetReadResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.readSet =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/ReadSet',
      request,
      metadata || {},
      methodDescriptor_SetService_ReadSet,
      callback);
};


/**
 * @param {!proto.set.SetId} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetReadResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.readSet =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/ReadSet',
      request,
      metadata || {},
      methodDescriptor_SetService_ReadSet);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetIds,
 *   !proto.set.SetListResponse>}
 */
const methodDescriptor_SetService_ListSetByIds = new grpc.web.MethodDescriptor(
  '/set.SetService/ListSetByIds',
  grpc.web.MethodType.UNARY,
  proto.set.SetIds,
  proto.set.SetListResponse,
  /**
   * @param {!proto.set.SetIds} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetListResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetIds} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.listSetByIds =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/ListSetByIds',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetByIds,
      callback);
};


/**
 * @param {!proto.set.SetIds} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetListResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.listSetByIds =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/ListSetByIds',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetByIds);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateId,
 *   !proto.set.SetListResponse>}
 */
const methodDescriptor_SetService_ListSetByTemplate = new grpc.web.MethodDescriptor(
  '/set.SetService/ListSetByTemplate',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateId,
  proto.set.SetListResponse,
  /**
   * @param {!proto.set.SetTemplateId} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetListResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateId} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.listSetByTemplate =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/ListSetByTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetByTemplate,
      callback);
};


/**
 * @param {!proto.set.SetTemplateId} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetListResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.listSetByTemplate =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/ListSetByTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetByTemplate);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetName,
 *   !proto.set.SetListResponse>}
 */
const methodDescriptor_SetService_ListSetByName = new grpc.web.MethodDescriptor(
  '/set.SetService/ListSetByName',
  grpc.web.MethodType.UNARY,
  proto.set.SetName,
  proto.set.SetListResponse,
  /**
   * @param {!proto.set.SetName} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetListResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetName} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.listSetByName =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/ListSetByName',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetByName,
      callback);
};


/**
 * @param {!proto.set.SetName} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetListResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.listSetByName =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/ListSetByName',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetByName);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetOption,
 *   !proto.set.SetListResponse>}
 */
const methodDescriptor_SetService_ListSetOption = new grpc.web.MethodDescriptor(
  '/set.SetService/ListSetOption',
  grpc.web.MethodType.UNARY,
  proto.set.SetOption,
  proto.set.SetListResponse,
  /**
   * @param {!proto.set.SetOption} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetListResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetOption} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.listSetOption =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/ListSetOption',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetOption,
      callback);
};


/**
 * @param {!proto.set.SetOption} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetListResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.listSetOption =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/ListSetOption',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetOption);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetSchema,
 *   !proto.set.SetCreateResponse>}
 */
const methodDescriptor_SetService_CreateSet = new grpc.web.MethodDescriptor(
  '/set.SetService/CreateSet',
  grpc.web.MethodType.UNARY,
  proto.set.SetSchema,
  proto.set.SetCreateResponse,
  /**
   * @param {!proto.set.SetSchema} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetCreateResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetSchema} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetCreateResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetCreateResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.createSet =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/CreateSet',
      request,
      metadata || {},
      methodDescriptor_SetService_CreateSet,
      callback);
};


/**
 * @param {!proto.set.SetSchema} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetCreateResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.createSet =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/CreateSet',
      request,
      metadata || {},
      methodDescriptor_SetService_CreateSet);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetUpdate,
 *   !proto.set.SetChangeResponse>}
 */
const methodDescriptor_SetService_UpdateSet = new grpc.web.MethodDescriptor(
  '/set.SetService/UpdateSet',
  grpc.web.MethodType.UNARY,
  proto.set.SetUpdate,
  proto.set.SetChangeResponse,
  /**
   * @param {!proto.set.SetUpdate} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetUpdate} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.updateSet =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/UpdateSet',
      request,
      metadata || {},
      methodDescriptor_SetService_UpdateSet,
      callback);
};


/**
 * @param {!proto.set.SetUpdate} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.updateSet =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/UpdateSet',
      request,
      metadata || {},
      methodDescriptor_SetService_UpdateSet);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetId,
 *   !proto.set.SetChangeResponse>}
 */
const methodDescriptor_SetService_DeleteSet = new grpc.web.MethodDescriptor(
  '/set.SetService/DeleteSet',
  grpc.web.MethodType.UNARY,
  proto.set.SetId,
  proto.set.SetChangeResponse,
  /**
   * @param {!proto.set.SetId} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetId} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.deleteSet =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/DeleteSet',
      request,
      metadata || {},
      methodDescriptor_SetService_DeleteSet,
      callback);
};


/**
 * @param {!proto.set.SetId} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.deleteSet =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/DeleteSet',
      request,
      metadata || {},
      methodDescriptor_SetService_DeleteSet);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetMemberRequest,
 *   !proto.set.SetChangeResponse>}
 */
const methodDescriptor_SetService_AddSetMember = new grpc.web.MethodDescriptor(
  '/set.SetService/AddSetMember',
  grpc.web.MethodType.UNARY,
  proto.set.SetMemberRequest,
  proto.set.SetChangeResponse,
  /**
   * @param {!proto.set.SetMemberRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetMemberRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.addSetMember =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/AddSetMember',
      request,
      metadata || {},
      methodDescriptor_SetService_AddSetMember,
      callback);
};


/**
 * @param {!proto.set.SetMemberRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.addSetMember =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/AddSetMember',
      request,
      metadata || {},
      methodDescriptor_SetService_AddSetMember);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetMemberRequest,
 *   !proto.set.SetChangeResponse>}
 */
const methodDescriptor_SetService_RemoveSetMember = new grpc.web.MethodDescriptor(
  '/set.SetService/RemoveSetMember',
  grpc.web.MethodType.UNARY,
  proto.set.SetMemberRequest,
  proto.set.SetChangeResponse,
  /**
   * @param {!proto.set.SetMemberRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetMemberRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.removeSetMember =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/RemoveSetMember',
      request,
      metadata || {},
      methodDescriptor_SetService_RemoveSetMember,
      callback);
};


/**
 * @param {!proto.set.SetMemberRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.removeSetMember =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/RemoveSetMember',
      request,
      metadata || {},
      methodDescriptor_SetService_RemoveSetMember);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetMemberSwap,
 *   !proto.set.SetChangeResponse>}
 */
const methodDescriptor_SetService_SwapSetMember = new grpc.web.MethodDescriptor(
  '/set.SetService/SwapSetMember',
  grpc.web.MethodType.UNARY,
  proto.set.SetMemberSwap,
  proto.set.SetChangeResponse,
  /**
   * @param {!proto.set.SetMemberSwap} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.SetChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetMemberSwap} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.SetChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.SetChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.swapSetMember =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/SwapSetMember',
      request,
      metadata || {},
      methodDescriptor_SetService_SwapSetMember,
      callback);
};


/**
 * @param {!proto.set.SetMemberSwap} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.SetChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.swapSetMember =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/SwapSetMember',
      request,
      metadata || {},
      methodDescriptor_SetService_SwapSetMember);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateId,
 *   !proto.set.TemplateReadResponse>}
 */
const methodDescriptor_SetService_ReadSetTemplate = new grpc.web.MethodDescriptor(
  '/set.SetService/ReadSetTemplate',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateId,
  proto.set.TemplateReadResponse,
  /**
   * @param {!proto.set.SetTemplateId} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateReadResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateId} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateReadResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateReadResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.readSetTemplate =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/ReadSetTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_ReadSetTemplate,
      callback);
};


/**
 * @param {!proto.set.SetTemplateId} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateReadResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.readSetTemplate =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/ReadSetTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_ReadSetTemplate);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateIds,
 *   !proto.set.TemplateListResponse>}
 */
const methodDescriptor_SetService_ListSetTemplateByIds = new grpc.web.MethodDescriptor(
  '/set.SetService/ListSetTemplateByIds',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateIds,
  proto.set.TemplateListResponse,
  /**
   * @param {!proto.set.SetTemplateIds} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateListResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateIds} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.listSetTemplateByIds =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/ListSetTemplateByIds',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetTemplateByIds,
      callback);
};


/**
 * @param {!proto.set.SetTemplateIds} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateListResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.listSetTemplateByIds =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/ListSetTemplateByIds',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetTemplateByIds);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateName,
 *   !proto.set.TemplateListResponse>}
 */
const methodDescriptor_SetService_ListSetTemplateByName = new grpc.web.MethodDescriptor(
  '/set.SetService/ListSetTemplateByName',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateName,
  proto.set.TemplateListResponse,
  /**
   * @param {!proto.set.SetTemplateName} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateListResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateName} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.listSetTemplateByName =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/ListSetTemplateByName',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetTemplateByName,
      callback);
};


/**
 * @param {!proto.set.SetTemplateName} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateListResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.listSetTemplateByName =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/ListSetTemplateByName',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetTemplateByName);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateOption,
 *   !proto.set.TemplateListResponse>}
 */
const methodDescriptor_SetService_ListSetTemplateOption = new grpc.web.MethodDescriptor(
  '/set.SetService/ListSetTemplateOption',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateOption,
  proto.set.TemplateListResponse,
  /**
   * @param {!proto.set.SetTemplateOption} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateListResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateOption} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateListResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateListResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.listSetTemplateOption =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/ListSetTemplateOption',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetTemplateOption,
      callback);
};


/**
 * @param {!proto.set.SetTemplateOption} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateListResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.listSetTemplateOption =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/ListSetTemplateOption',
      request,
      metadata || {},
      methodDescriptor_SetService_ListSetTemplateOption);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateSchema,
 *   !proto.set.TemplateCreateResponse>}
 */
const methodDescriptor_SetService_CreateSetTemplate = new grpc.web.MethodDescriptor(
  '/set.SetService/CreateSetTemplate',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateSchema,
  proto.set.TemplateCreateResponse,
  /**
   * @param {!proto.set.SetTemplateSchema} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateCreateResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateSchema} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateCreateResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateCreateResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.createSetTemplate =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/CreateSetTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_CreateSetTemplate,
      callback);
};


/**
 * @param {!proto.set.SetTemplateSchema} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateCreateResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.createSetTemplate =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/CreateSetTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_CreateSetTemplate);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateUpdate,
 *   !proto.set.TemplateChangeResponse>}
 */
const methodDescriptor_SetService_UpdateSetTemplate = new grpc.web.MethodDescriptor(
  '/set.SetService/UpdateSetTemplate',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateUpdate,
  proto.set.TemplateChangeResponse,
  /**
   * @param {!proto.set.SetTemplateUpdate} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateUpdate} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.updateSetTemplate =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/UpdateSetTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_UpdateSetTemplate,
      callback);
};


/**
 * @param {!proto.set.SetTemplateUpdate} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.updateSetTemplate =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/UpdateSetTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_UpdateSetTemplate);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateId,
 *   !proto.set.TemplateChangeResponse>}
 */
const methodDescriptor_SetService_DeleteSetTemplate = new grpc.web.MethodDescriptor(
  '/set.SetService/DeleteSetTemplate',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateId,
  proto.set.TemplateChangeResponse,
  /**
   * @param {!proto.set.SetTemplateId} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateId} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.deleteSetTemplate =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/DeleteSetTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_DeleteSetTemplate,
      callback);
};


/**
 * @param {!proto.set.SetTemplateId} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.deleteSetTemplate =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/DeleteSetTemplate',
      request,
      metadata || {},
      methodDescriptor_SetService_DeleteSetTemplate);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateMemberRequest,
 *   !proto.set.TemplateChangeResponse>}
 */
const methodDescriptor_SetService_AddSetTemplateMember = new grpc.web.MethodDescriptor(
  '/set.SetService/AddSetTemplateMember',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateMemberRequest,
  proto.set.TemplateChangeResponse,
  /**
   * @param {!proto.set.SetTemplateMemberRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateMemberRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.addSetTemplateMember =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/AddSetTemplateMember',
      request,
      metadata || {},
      methodDescriptor_SetService_AddSetTemplateMember,
      callback);
};


/**
 * @param {!proto.set.SetTemplateMemberRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.addSetTemplateMember =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/AddSetTemplateMember',
      request,
      metadata || {},
      methodDescriptor_SetService_AddSetTemplateMember);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateMemberRequest,
 *   !proto.set.TemplateChangeResponse>}
 */
const methodDescriptor_SetService_RemoveSetTemplateMember = new grpc.web.MethodDescriptor(
  '/set.SetService/RemoveSetTemplateMember',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateMemberRequest,
  proto.set.TemplateChangeResponse,
  /**
   * @param {!proto.set.SetTemplateMemberRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateMemberRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.removeSetTemplateMember =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/RemoveSetTemplateMember',
      request,
      metadata || {},
      methodDescriptor_SetService_RemoveSetTemplateMember,
      callback);
};


/**
 * @param {!proto.set.SetTemplateMemberRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.removeSetTemplateMember =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/RemoveSetTemplateMember',
      request,
      metadata || {},
      methodDescriptor_SetService_RemoveSetTemplateMember);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.set.SetTemplateMemberSwap,
 *   !proto.set.TemplateChangeResponse>}
 */
const methodDescriptor_SetService_SwapSetTemplateMember = new grpc.web.MethodDescriptor(
  '/set.SetService/SwapSetTemplateMember',
  grpc.web.MethodType.UNARY,
  proto.set.SetTemplateMemberSwap,
  proto.set.TemplateChangeResponse,
  /**
   * @param {!proto.set.SetTemplateMemberSwap} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.set.TemplateChangeResponse.deserializeBinary
);


/**
 * @param {!proto.set.SetTemplateMemberSwap} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.set.TemplateChangeResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.set.TemplateChangeResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.set.SetServiceClient.prototype.swapSetTemplateMember =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/set.SetService/SwapSetTemplateMember',
      request,
      metadata || {},
      methodDescriptor_SetService_SwapSetTemplateMember,
      callback);
};


/**
 * @param {!proto.set.SetTemplateMemberSwap} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.set.TemplateChangeResponse>}
 *     Promise that resolves to the response
 */
proto.set.SetServicePromiseClient.prototype.swapSetTemplateMember =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/set.SetService/SwapSetTemplateMember',
      request,
      metadata || {},
      methodDescriptor_SetService_SwapSetTemplateMember);
};


module.exports = proto.set;

