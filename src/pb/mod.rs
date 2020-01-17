pub(crate) mod mvccpb {
    include!("mvccpb.rs");
}

pub(crate) mod authpb {
    include!("authpb.rs");
}

pub(crate) mod etcdserverpb {
    include!("etcdserverpb.rs");
}

pub use authpb::*;
pub use etcdserverpb::*;
pub use mvccpb::*;
