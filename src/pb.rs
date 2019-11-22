mod mvccpb {
    tonic::include_proto!("mvccpb");
}

mod authpb {
    tonic::include_proto!("authpb");
}

mod etcdserverpb {
    tonic::include_proto!("etcdserverpb");
}

pub use authpb::*;
pub use etcdserverpb::*;
pub use mvccpb::*;
