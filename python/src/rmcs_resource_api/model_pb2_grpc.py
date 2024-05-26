# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc
import warnings

from rmcs_resource_api import model_pb2 as rmcs__resource__api_dot_model__pb2

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
        + f' but the generated code in rmcs_resource_api/model_pb2_grpc.py depends on'
        + f' grpcio>={GRPC_GENERATED_VERSION}.'
        + f' Please upgrade your grpc module to grpcio>={GRPC_GENERATED_VERSION}'
        + f' or downgrade your generated code using grpcio-tools<={GRPC_VERSION}.'
        + f' This warning will become an error in {EXPECTED_ERROR_RELEASE},'
        + f' scheduled for release on {SCHEDULED_RELEASE_DATE}.',
        RuntimeWarning
    )


class ModelServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.ReadModel = channel.unary_unary(
                '/model.ModelService/ReadModel',
                request_serializer=rmcs__resource__api_dot_model__pb2.ModelId.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ModelReadResponse.FromString,
                _registered_method=True)
        self.ListModelByName = channel.unary_unary(
                '/model.ModelService/ListModelByName',
                request_serializer=rmcs__resource__api_dot_model__pb2.ModelName.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ModelListResponse.FromString,
                _registered_method=True)
        self.ListModelByCategory = channel.unary_unary(
                '/model.ModelService/ListModelByCategory',
                request_serializer=rmcs__resource__api_dot_model__pb2.ModelCategory.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ModelListResponse.FromString,
                _registered_method=True)
        self.ListModelByNameCategory = channel.unary_unary(
                '/model.ModelService/ListModelByNameCategory',
                request_serializer=rmcs__resource__api_dot_model__pb2.ModelNameCategory.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ModelListResponse.FromString,
                _registered_method=True)
        self.ListModelByType = channel.unary_unary(
                '/model.ModelService/ListModelByType',
                request_serializer=rmcs__resource__api_dot_model__pb2.TypeId.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ModelListResponse.FromString,
                _registered_method=True)
        self.CreateModel = channel.unary_unary(
                '/model.ModelService/CreateModel',
                request_serializer=rmcs__resource__api_dot_model__pb2.ModelSchema.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ModelCreateResponse.FromString,
                _registered_method=True)
        self.UpdateModel = channel.unary_unary(
                '/model.ModelService/UpdateModel',
                request_serializer=rmcs__resource__api_dot_model__pb2.ModelUpdate.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ModelChangeResponse.FromString,
                _registered_method=True)
        self.DeleteModel = channel.unary_unary(
                '/model.ModelService/DeleteModel',
                request_serializer=rmcs__resource__api_dot_model__pb2.ModelId.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ModelChangeResponse.FromString,
                _registered_method=True)
        self.ReadModelConfig = channel.unary_unary(
                '/model.ModelService/ReadModelConfig',
                request_serializer=rmcs__resource__api_dot_model__pb2.ConfigId.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ConfigReadResponse.FromString,
                _registered_method=True)
        self.ListModelConfig = channel.unary_unary(
                '/model.ModelService/ListModelConfig',
                request_serializer=rmcs__resource__api_dot_model__pb2.ModelId.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ConfigListResponse.FromString,
                _registered_method=True)
        self.CreateModelConfig = channel.unary_unary(
                '/model.ModelService/CreateModelConfig',
                request_serializer=rmcs__resource__api_dot_model__pb2.ConfigSchema.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ConfigCreateResponse.FromString,
                _registered_method=True)
        self.UpdateModelConfig = channel.unary_unary(
                '/model.ModelService/UpdateModelConfig',
                request_serializer=rmcs__resource__api_dot_model__pb2.ConfigUpdate.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ConfigChangeResponse.FromString,
                _registered_method=True)
        self.DeleteModelConfig = channel.unary_unary(
                '/model.ModelService/DeleteModelConfig',
                request_serializer=rmcs__resource__api_dot_model__pb2.ConfigId.SerializeToString,
                response_deserializer=rmcs__resource__api_dot_model__pb2.ConfigChangeResponse.FromString,
                _registered_method=True)


class ModelServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def ReadModel(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListModelByName(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListModelByCategory(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListModelByNameCategory(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListModelByType(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateModel(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UpdateModel(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def DeleteModel(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ReadModelConfig(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListModelConfig(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateModelConfig(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UpdateModelConfig(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def DeleteModelConfig(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_ModelServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'ReadModel': grpc.unary_unary_rpc_method_handler(
                    servicer.ReadModel,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ModelId.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ModelReadResponse.SerializeToString,
            ),
            'ListModelByName': grpc.unary_unary_rpc_method_handler(
                    servicer.ListModelByName,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ModelName.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ModelListResponse.SerializeToString,
            ),
            'ListModelByCategory': grpc.unary_unary_rpc_method_handler(
                    servicer.ListModelByCategory,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ModelCategory.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ModelListResponse.SerializeToString,
            ),
            'ListModelByNameCategory': grpc.unary_unary_rpc_method_handler(
                    servicer.ListModelByNameCategory,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ModelNameCategory.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ModelListResponse.SerializeToString,
            ),
            'ListModelByType': grpc.unary_unary_rpc_method_handler(
                    servicer.ListModelByType,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.TypeId.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ModelListResponse.SerializeToString,
            ),
            'CreateModel': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateModel,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ModelSchema.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ModelCreateResponse.SerializeToString,
            ),
            'UpdateModel': grpc.unary_unary_rpc_method_handler(
                    servicer.UpdateModel,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ModelUpdate.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ModelChangeResponse.SerializeToString,
            ),
            'DeleteModel': grpc.unary_unary_rpc_method_handler(
                    servicer.DeleteModel,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ModelId.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ModelChangeResponse.SerializeToString,
            ),
            'ReadModelConfig': grpc.unary_unary_rpc_method_handler(
                    servicer.ReadModelConfig,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ConfigId.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ConfigReadResponse.SerializeToString,
            ),
            'ListModelConfig': grpc.unary_unary_rpc_method_handler(
                    servicer.ListModelConfig,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ModelId.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ConfigListResponse.SerializeToString,
            ),
            'CreateModelConfig': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateModelConfig,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ConfigSchema.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ConfigCreateResponse.SerializeToString,
            ),
            'UpdateModelConfig': grpc.unary_unary_rpc_method_handler(
                    servicer.UpdateModelConfig,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ConfigUpdate.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ConfigChangeResponse.SerializeToString,
            ),
            'DeleteModelConfig': grpc.unary_unary_rpc_method_handler(
                    servicer.DeleteModelConfig,
                    request_deserializer=rmcs__resource__api_dot_model__pb2.ConfigId.FromString,
                    response_serializer=rmcs__resource__api_dot_model__pb2.ConfigChangeResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'model.ModelService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class ModelService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def ReadModel(request,
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
            '/model.ModelService/ReadModel',
            rmcs__resource__api_dot_model__pb2.ModelId.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ModelReadResponse.FromString,
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
    def ListModelByName(request,
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
            '/model.ModelService/ListModelByName',
            rmcs__resource__api_dot_model__pb2.ModelName.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ModelListResponse.FromString,
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
    def ListModelByCategory(request,
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
            '/model.ModelService/ListModelByCategory',
            rmcs__resource__api_dot_model__pb2.ModelCategory.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ModelListResponse.FromString,
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
    def ListModelByNameCategory(request,
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
            '/model.ModelService/ListModelByNameCategory',
            rmcs__resource__api_dot_model__pb2.ModelNameCategory.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ModelListResponse.FromString,
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
    def ListModelByType(request,
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
            '/model.ModelService/ListModelByType',
            rmcs__resource__api_dot_model__pb2.TypeId.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ModelListResponse.FromString,
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
    def CreateModel(request,
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
            '/model.ModelService/CreateModel',
            rmcs__resource__api_dot_model__pb2.ModelSchema.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ModelCreateResponse.FromString,
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
    def UpdateModel(request,
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
            '/model.ModelService/UpdateModel',
            rmcs__resource__api_dot_model__pb2.ModelUpdate.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ModelChangeResponse.FromString,
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
    def DeleteModel(request,
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
            '/model.ModelService/DeleteModel',
            rmcs__resource__api_dot_model__pb2.ModelId.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ModelChangeResponse.FromString,
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
    def ReadModelConfig(request,
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
            '/model.ModelService/ReadModelConfig',
            rmcs__resource__api_dot_model__pb2.ConfigId.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ConfigReadResponse.FromString,
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
    def ListModelConfig(request,
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
            '/model.ModelService/ListModelConfig',
            rmcs__resource__api_dot_model__pb2.ModelId.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ConfigListResponse.FromString,
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
    def CreateModelConfig(request,
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
            '/model.ModelService/CreateModelConfig',
            rmcs__resource__api_dot_model__pb2.ConfigSchema.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ConfigCreateResponse.FromString,
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
    def UpdateModelConfig(request,
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
            '/model.ModelService/UpdateModelConfig',
            rmcs__resource__api_dot_model__pb2.ConfigUpdate.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ConfigChangeResponse.FromString,
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
    def DeleteModelConfig(request,
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
            '/model.ModelService/DeleteModelConfig',
            rmcs__resource__api_dot_model__pb2.ConfigId.SerializeToString,
            rmcs__resource__api_dot_model__pb2.ConfigChangeResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)
