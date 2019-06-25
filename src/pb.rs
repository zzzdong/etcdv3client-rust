// comm.rs

pub mod mvccpb {
    include!(concat!(env!("OUT_DIR"), "/mvccpb.rs"));
}

pub mod authpb {
    include!(concat!(env!("OUT_DIR"), "/authpb.rs"));
}

pub mod etcdserverpb {
    include!(concat!(env!("OUT_DIR"), "/etcdserverpb.rs"));
}
