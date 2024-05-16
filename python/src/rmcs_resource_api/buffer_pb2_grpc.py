# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc
import warnings

from rmcs_resource_api import buffer_pb2 as rmcs__resource__api_dot_buffer__pb2

GRPC_GENERATED_VERSION = '1.63.0'
GRPC_VERSION = grpc.__version__
EXPECTED_ERROR_RELEASE = '1.65.0'
SCHEDULED_RELEASE_DATE = 'June 25, 2024'
_version_not_supported = False

try:
    from grpc._utilities import first_version_is_lower
    _version_not_supported = first_version_is_lower(GRPC_VERSION, GRPC_GENERATED_VERSION)
except ImportError:
    _version_not_supported = True

if _version_not_supported:
    warnings.warn(
        f'The grpc package installed is at version {GRPC_VERSION},'
        + f' but the generated code in rmcs_resource_api/buffer_pb2_grpc.py depends on'
        + f' grpcio>={GRPC_GENERATED_VERSION}.'
        + f' Please upgrade your grpc module to grpcio>={GRPC_GENERATED_VERSION}'
        + f' or downgrade your generated code using grpcio-tools<={GRPC_VERSION}.'
        + f' This warning will become an error in {EXPECTED_ERROR_RELEASE},'
        + f' scheduled for release on {SCHEDULED_RELEASE_DATE}.',
        RuntimeWarning
    )


class BufferServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.ReadBuffer = channel.unary_unary(
                '/buffer.BufferService/ReadBuffer',
                request_serializer=rmcs__resource__api_dot_buffer__pb2.BufferId.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.FromString,
                _registered_method=True)
        self.ReadBufferByTime = channel.unary_unary(
                '/buffer.BufferService/ReadBufferByTime',
                request_serializer=rmcs__resource__api_dot_buffer__pb2.BufferTime.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.FromString,
                _registered_method=True)
        self.ReadBufferFirst = channel.unary_unary(
                '/buffer.BufferService/ReadBufferFirst',
                request_serializer=rmcs__resource__api_dot_buffer__pb2.BufferSelector.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.FromString,
                _registered_method=True)
        self.ReadBufferLast = channel.unary_unary(
                '/buffer.BufferService/ReadBufferLast',
                request_serializer=rmcs__resource__api_dot_buffer__pb2.BufferSelector.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.FromString,
                _registered_method=True)
        self.ListBufferFirst = channel.unary_unary(
                '/buffer.BufferService/ListBufferFirst',
                request_serializer=rmcs__resource__api_dot_buffer__pb2.BuffersSelector.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferListResponse.FromString,
                _registered_method=True)
        self.ListBufferLast = channel.unary_unary(
                '/buffer.BufferService/ListBufferLast',
                request_serializer=rmcs__resource__api_dot_buffer__pb2.BuffersSelector.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferListResponse.FromString,
                _registered_method=True)
        self.CreateBuffer = channel.unary_unary(
                '/buffer.BufferService/CreateBuffer',
                request_serializer=rmcs__resource__api_dot_buffer__pb2.BufferSchema.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferCreateResponse.FromString,
                _registered_method=True)
        self.UpdateBuffer = channel.unary_unary(
                '/buffer.BufferService/UpdateBuffer',
                request_serializer=rmcs__resource__api_dot_buffer__pb2.BufferUpdate.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferChangeResponse.FromString,
                _registered_method=True)
        self.DeleteBuffer = channel.unary_unary(
                '/buffer.BufferService/DeleteBuffer',
                request_serializer=rmcs__resource__api_dot_buffer__pb2.BufferId.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferChangeResponse.FromString,
                _registered_method=True)


class BufferServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def ReadBuffer(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ReadBufferByTime(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ReadBufferFirst(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ReadBufferLast(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListBufferFirst(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListBufferLast(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateBuffer(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UpdateBuffer(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def DeleteBuffer(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_BufferServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'ReadBuffer': grpc.unary_unary_rpc_method_handler(
                    servicer.ReadBuffer,
                    request_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferId.FromString,
                    response_serializer=rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.SerializeToString,
            ),
            'ReadBufferByTime': grpc.unary_unary_rpc_method_handler(
                    servicer.ReadBufferByTime,
                    request_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferTime.FromString,
                    response_serializer=rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.SerializeToString,
            ),
            'ReadBufferFirst': grpc.unary_unary_rpc_method_handler(
                    servicer.ReadBufferFirst,
                    request_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferSelector.FromString,
                    response_serializer=rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.SerializeToString,
            ),
            'ReadBufferLast': grpc.unary_unary_rpc_method_handler(
                    servicer.ReadBufferLast,
                    request_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferSelector.FromString,
                    response_serializer=rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.SerializeToString,
            ),
            'ListBufferFirst': grpc.unary_unary_rpc_method_handler(
                    servicer.ListBufferFirst,
                    request_deserializer=rmcs__resource__api_dot_buffer__pb2.BuffersSelector.FromString,
                    response_serializer=rmcs__resource__api_dot_buffer__pb2.BufferListResponse.SerializeToString,
            ),
            'ListBufferLast': grpc.unary_unary_rpc_method_handler(
                    servicer.ListBufferLast,
                    request_deserializer=rmcs__resource__api_dot_buffer__pb2.BuffersSelector.FromString,
                    response_serializer=rmcs__resource__api_dot_buffer__pb2.BufferListResponse.SerializeToString,
            ),
            'CreateBuffer': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateBuffer,
                    request_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferSchema.FromString,
                    response_serializer=rmcs__resource__api_dot_buffer__pb2.BufferCreateResponse.SerializeToString,
            ),
            'UpdateBuffer': grpc.unary_unary_rpc_method_handler(
                    servicer.UpdateBuffer,
                    request_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferUpdate.FromString,
                    response_serializer=rmcs__resource__api_dot_buffer__pb2.BufferChangeResponse.SerializeToString,
            ),
            'DeleteBuffer': grpc.unary_unary_rpc_method_handler(
                    servicer.DeleteBuffer,
                    request_deserializer=rmcs__resource__api_dot_buffer__pb2.BufferId.FromString,
                    response_serializer=rmcs__resource__api_dot_buffer__pb2.BufferChangeResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'buffer.BufferService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class BufferService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def ReadBuffer(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/buffer.BufferService/ReadBuffer',
            rmcs__resource__api_dot_buffer__pb2.BufferId.SerializeToString,
            rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def ReadBufferByTime(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/buffer.BufferService/ReadBufferByTime',
            rmcs__resource__api_dot_buffer__pb2.BufferTime.SerializeToString,
            rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def ReadBufferFirst(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/buffer.BufferService/ReadBufferFirst',
            rmcs__resource__api_dot_buffer__pb2.BufferSelector.SerializeToString,
            rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def ReadBufferLast(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/buffer.BufferService/ReadBufferLast',
            rmcs__resource__api_dot_buffer__pb2.BufferSelector.SerializeToString,
            rmcs__resource__api_dot_buffer__pb2.BufferReadResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def ListBufferFirst(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/buffer.BufferService/ListBufferFirst',
            rmcs__resource__api_dot_buffer__pb2.BuffersSelector.SerializeToString,
            rmcs__resource__api_dot_buffer__pb2.BufferListResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def ListBufferLast(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/buffer.BufferService/ListBufferLast',
            rmcs__resource__api_dot_buffer__pb2.BuffersSelector.SerializeToString,
            rmcs__resource__api_dot_buffer__pb2.BufferListResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def CreateBuffer(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/buffer.BufferService/CreateBuffer',
            rmcs__resource__api_dot_buffer__pb2.BufferSchema.SerializeToString,
            rmcs__resource__api_dot_buffer__pb2.BufferCreateResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def UpdateBuffer(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/buffer.BufferService/UpdateBuffer',
            rmcs__resource__api_dot_buffer__pb2.BufferUpdate.SerializeToString,
            rmcs__resource__api_dot_buffer__pb2.BufferChangeResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def DeleteBuffer(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/buffer.BufferService/DeleteBuffer',
            rmcs__resource__api_dot_buffer__pb2.BufferId.SerializeToString,
            rmcs__resource__api_dot_buffer__pb2.BufferChangeResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)
