pub mod model {
    pub const DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("model_descriptor");
}

pub mod device {
    pub const DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("device_descriptor");
}

pub mod group {
    pub const DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("group_descriptor");
}

pub mod data {
    pub const DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("data_descriptor");
}

pub mod buffer {
    pub const DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("buffer_descriptor");
}

pub mod slice {
    pub const DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("slice_descriptor");
}

pub mod log {
    pub const DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("log_descriptor");
}

pub mod set {
    pub const DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("set_descriptor");
}
