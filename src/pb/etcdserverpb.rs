#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseHeader {
    /// cluster_id is the ID of the cluster which sent the response.
    #[prost(uint64, tag = "1")]
    pub cluster_id: u64,
    /// member_id is the ID of the member which sent the response.
    #[prost(uint64, tag = "2")]
    pub member_id: u64,
    /// revision is the key-value store revision when the request was applied.
    /// For watch progress responses, the header.revision indicates progress. All future events
    /// recieved in this stream are guaranteed to have a higher revision number than the
    /// header.revision number.
    #[prost(int64, tag = "3")]
    pub revision: i64,
    /// raft_term is the raft term when the request was applied.
    #[prost(uint64, tag = "4")]
    pub raft_term: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeRequest {
    /// key is the first key for the range. If range_end is not given, the request only looks up key.
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// range_end is the upper bound on the requested range [key, range_end).
    /// If range_end is '\0', the range is all keys >= key.
    /// If range_end is key plus one (e.g., "aa"+1 == "ab", "a\xff"+1 == "b"),
    /// then the range request gets all keys prefixed with key.
    /// If both key and range_end are '\0', then the range request returns all keys.
    #[prost(bytes = "vec", tag = "2")]
    pub range_end: ::prost::alloc::vec::Vec<u8>,
    /// limit is a limit on the number of keys returned for the request. When limit is set to 0,
    /// it is treated as no limit.
    #[prost(int64, tag = "3")]
    pub limit: i64,
    /// revision is the point-in-time of the key-value store to use for the range.
    /// If revision is less or equal to zero, the range is over the newest key-value store.
    /// If the revision has been compacted, ErrCompacted is returned as a response.
    #[prost(int64, tag = "4")]
    pub revision: i64,
    /// sort_order is the order for returned sorted results.
    #[prost(enumeration = "range_request::SortOrder", tag = "5")]
    pub sort_order: i32,
    /// sort_target is the key-value field to use for sorting.
    #[prost(enumeration = "range_request::SortTarget", tag = "6")]
    pub sort_target: i32,
    /// serializable sets the range request to use serializable member-local reads.
    /// Range requests are linearizable by default; linearizable requests have higher
    /// latency and lower throughput than serializable requests but reflect the current
    /// consensus of the cluster. For better performance, in exchange for possible stale reads,
    /// a serializable range request is served locally without needing to reach consensus
    /// with other nodes in the cluster.
    #[prost(bool, tag = "7")]
    pub serializable: bool,
    /// keys_only when set returns only the keys and not the values.
    #[prost(bool, tag = "8")]
    pub keys_only: bool,
    /// count_only when set returns only the count of the keys in the range.
    #[prost(bool, tag = "9")]
    pub count_only: bool,
    /// min_mod_revision is the lower bound for returned key mod revisions; all keys with
    /// lesser mod revisions will be filtered away.
    #[prost(int64, tag = "10")]
    pub min_mod_revision: i64,
    /// max_mod_revision is the upper bound for returned key mod revisions; all keys with
    /// greater mod revisions will be filtered away.
    #[prost(int64, tag = "11")]
    pub max_mod_revision: i64,
    /// min_create_revision is the lower bound for returned key create revisions; all keys with
    /// lesser create revisions will be filtered away.
    #[prost(int64, tag = "12")]
    pub min_create_revision: i64,
    /// max_create_revision is the upper bound for returned key create revisions; all keys with
    /// greater create revisions will be filtered away.
    #[prost(int64, tag = "13")]
    pub max_create_revision: i64,
}
/// Nested message and enum types in `RangeRequest`.
pub mod range_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SortOrder {
        /// default, no sorting
        None = 0,
        /// lowest target value first
        Ascend = 1,
        /// highest target value first
        Descend = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SortTarget {
        Key = 0,
        Version = 1,
        Create = 2,
        Mod = 3,
        Value = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// kvs is the list of key-value pairs matched by the range request.
    /// kvs is empty when count is requested.
    #[prost(message, repeated, tag = "2")]
    pub kvs: ::prost::alloc::vec::Vec<super::mvccpb::KeyValue>,
    /// more indicates if there are more keys to return in the requested range.
    #[prost(bool, tag = "3")]
    pub more: bool,
    /// count is set to the number of keys within the range when requested.
    #[prost(int64, tag = "4")]
    pub count: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutRequest {
    /// key is the key, in bytes, to put into the key-value store.
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// value is the value, in bytes, to associate with the key in the key-value store.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// lease is the lease ID to associate with the key in the key-value store. A lease
    /// value of 0 indicates no lease.
    #[prost(int64, tag = "3")]
    pub lease: i64,
    /// If prev_kv is set, etcd gets the previous key-value pair before changing it.
    /// The previous key-value pair will be returned in the put response.
    #[prost(bool, tag = "4")]
    pub prev_kv: bool,
    /// If ignore_value is set, etcd updates the key using its current value.
    /// Returns an error if the key does not exist.
    #[prost(bool, tag = "5")]
    pub ignore_value: bool,
    /// If ignore_lease is set, etcd updates the key using its current lease.
    /// Returns an error if the key does not exist.
    #[prost(bool, tag = "6")]
    pub ignore_lease: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// if prev_kv is set in the request, the previous key-value pair will be returned.
    #[prost(message, optional, tag = "2")]
    pub prev_kv: ::core::option::Option<super::mvccpb::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRangeRequest {
    /// key is the first key to delete in the range.
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// range_end is the key following the last key to delete for the range [key, range_end).
    /// If range_end is not given, the range is defined to contain only the key argument.
    /// If range_end is one bit larger than the given key, then the range is all the keys
    /// with the prefix (the given key).
    /// If range_end is '\0', the range is all keys greater than or equal to the key argument.
    #[prost(bytes = "vec", tag = "2")]
    pub range_end: ::prost::alloc::vec::Vec<u8>,
    /// If prev_kv is set, etcd gets the previous key-value pairs before deleting it.
    /// The previous key-value pairs will be returned in the delete response.
    #[prost(bool, tag = "3")]
    pub prev_kv: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRangeResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// deleted is the number of keys deleted by the delete range request.
    #[prost(int64, tag = "2")]
    pub deleted: i64,
    /// if prev_kv is set in the request, the previous key-value pairs will be returned.
    #[prost(message, repeated, tag = "3")]
    pub prev_kvs: ::prost::alloc::vec::Vec<super::mvccpb::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestOp {
    /// request is a union of request types accepted by a transaction.
    #[prost(oneof = "request_op::Request", tags = "1, 2, 3, 4")]
    pub request: ::core::option::Option<request_op::Request>,
}
/// Nested message and enum types in `RequestOp`.
pub mod request_op {
    /// request is a union of request types accepted by a transaction.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "1")]
        RequestRange(super::RangeRequest),
        #[prost(message, tag = "2")]
        RequestPut(super::PutRequest),
        #[prost(message, tag = "3")]
        RequestDeleteRange(super::DeleteRangeRequest),
        #[prost(message, tag = "4")]
        RequestTxn(super::TxnRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseOp {
    /// response is a union of response types returned by a transaction.
    #[prost(oneof = "response_op::Response", tags = "1, 2, 3, 4")]
    pub response: ::core::option::Option<response_op::Response>,
}
/// Nested message and enum types in `ResponseOp`.
pub mod response_op {
    /// response is a union of response types returned by a transaction.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        ResponseRange(super::RangeResponse),
        #[prost(message, tag = "2")]
        ResponsePut(super::PutResponse),
        #[prost(message, tag = "3")]
        ResponseDeleteRange(super::DeleteRangeResponse),
        #[prost(message, tag = "4")]
        ResponseTxn(super::TxnResponse),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compare {
    /// result is logical comparison operation for this comparison.
    #[prost(enumeration = "compare::CompareResult", tag = "1")]
    pub result: i32,
    /// target is the key-value field to inspect for the comparison.
    #[prost(enumeration = "compare::CompareTarget", tag = "2")]
    pub target: i32,
    /// key is the subject key for the comparison operation.
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// range_end compares the given target to all keys in the range [key, range_end).
    /// See RangeRequest for more details on key ranges.
    ///
    /// TODO: fill out with most of the rest of RangeRequest fields when needed.
    #[prost(bytes = "vec", tag = "64")]
    pub range_end: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "compare::TargetUnion", tags = "4, 5, 6, 7, 8")]
    pub target_union: ::core::option::Option<compare::TargetUnion>,
}
/// Nested message and enum types in `Compare`.
pub mod compare {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CompareResult {
        Equal = 0,
        Greater = 1,
        Less = 2,
        NotEqual = 3,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CompareTarget {
        Version = 0,
        Create = 1,
        Mod = 2,
        Value = 3,
        Lease = 4,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetUnion {
        /// version is the version of the given key
        #[prost(int64, tag = "4")]
        Version(i64),
        /// create_revision is the creation revision of the given key
        #[prost(int64, tag = "5")]
        CreateRevision(i64),
        /// mod_revision is the last modified revision of the given key.
        #[prost(int64, tag = "6")]
        ModRevision(i64),
        /// value is the value of the given key, in bytes.
        #[prost(bytes, tag = "7")]
        Value(::prost::alloc::vec::Vec<u8>),
        /// lease is the lease id of the given key.
        ///
        /// leave room for more target_union field tags, jump to 64
        #[prost(int64, tag = "8")]
        Lease(i64),
    }
}
/// From google paxosdb paper:
/// Our implementation hinges around a powerful primitive which we call MultiOp. All other database
/// operations except for iteration are implemented as a single call to MultiOp. A MultiOp is applied atomically
/// and consists of three components:
/// 1. A list of tests called guard. Each test in guard checks a single entry in the database. It may check
/// for the absence or presence of a value, or compare with a given value. Two different tests in the guard
/// may apply to the same or different entries in the database. All tests in the guard are applied and
/// MultiOp returns the results. If all tests are true, MultiOp executes t op (see item 2 below), otherwise
/// it executes f op (see item 3 below).
/// 2. A list of database operations called t op. Each operation in the list is either an insert, delete, or
/// lookup operation, and applies to a single database entry. Two different operations in the list may apply
/// to the same or different entries in the database. These operations are executed
/// if guard evaluates to
/// true.
/// 3. A list of database operations called f op. Like t op, but executed if guard evaluates to false.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnRequest {
    /// compare is a list of predicates representing a conjunction of terms.
    /// If the comparisons succeed, then the success requests will be processed in order,
    /// and the response will contain their respective responses in order.
    /// If the comparisons fail, then the failure requests will be processed in order,
    /// and the response will contain their respective responses in order.
    #[prost(message, repeated, tag = "1")]
    pub compare: ::prost::alloc::vec::Vec<Compare>,
    /// success is a list of requests which will be applied when compare evaluates to true.
    #[prost(message, repeated, tag = "2")]
    pub success: ::prost::alloc::vec::Vec<RequestOp>,
    /// failure is a list of requests which will be applied when compare evaluates to false.
    #[prost(message, repeated, tag = "3")]
    pub failure: ::prost::alloc::vec::Vec<RequestOp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// succeeded is set to true if the compare evaluated to true or false otherwise.
    #[prost(bool, tag = "2")]
    pub succeeded: bool,
    /// responses is a list of responses corresponding to the results from applying
    /// success if succeeded is true or failure if succeeded is false.
    #[prost(message, repeated, tag = "3")]
    pub responses: ::prost::alloc::vec::Vec<ResponseOp>,
}
/// CompactionRequest compacts the key-value store up to a given revision. All superseded keys
/// with a revision less than the compaction revision will be removed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactionRequest {
    /// revision is the key-value store revision for the compaction operation.
    #[prost(int64, tag = "1")]
    pub revision: i64,
    /// physical is set so the RPC will wait until the compaction is physically
    /// applied to the local database such that compacted entries are totally
    /// removed from the backend database.
    #[prost(bool, tag = "2")]
    pub physical: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactionResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashKvRequest {
    /// revision is the key-value store revision for the hash operation.
    #[prost(int64, tag = "1")]
    pub revision: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashKvResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// hash is the hash value computed from the responding member's MVCC keys up to a given revision.
    #[prost(uint32, tag = "2")]
    pub hash: u32,
    /// compact_revision is the compacted revision of key-value store when hash begins.
    #[prost(int64, tag = "3")]
    pub compact_revision: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// hash is the hash value computed from the responding member's KV's backend.
    #[prost(uint32, tag = "2")]
    pub hash: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotResponse {
    /// header has the current key-value store information. The first header in the snapshot
    /// stream indicates the point in time of the snapshot.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// remaining_bytes is the number of blob bytes to be sent after this message
    #[prost(uint64, tag = "2")]
    pub remaining_bytes: u64,
    /// blob contains the next chunk of the snapshot in the snapshot stream.
    #[prost(bytes = "vec", tag = "3")]
    pub blob: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchRequest {
    /// request_union is a request to either create a new watcher or cancel an existing watcher.
    #[prost(oneof = "watch_request::RequestUnion", tags = "1, 2, 3")]
    pub request_union: ::core::option::Option<watch_request::RequestUnion>,
}
/// Nested message and enum types in `WatchRequest`.
pub mod watch_request {
    /// request_union is a request to either create a new watcher or cancel an existing watcher.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestUnion {
        #[prost(message, tag = "1")]
        CreateRequest(super::WatchCreateRequest),
        #[prost(message, tag = "2")]
        CancelRequest(super::WatchCancelRequest),
        #[prost(message, tag = "3")]
        ProgressRequest(super::WatchProgressRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchCreateRequest {
    /// key is the key to register for watching.
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// range_end is the end of the range [key, range_end) to watch. If range_end is not given,
    /// only the key argument is watched. If range_end is equal to '\0', all keys greater than
    /// or equal to the key argument are watched.
    /// If the range_end is one bit larger than the given key,
    /// then all keys with the prefix (the given key) will be watched.
    #[prost(bytes = "vec", tag = "2")]
    pub range_end: ::prost::alloc::vec::Vec<u8>,
    /// start_revision is an optional revision to watch from (inclusive). No start_revision is "now".
    #[prost(int64, tag = "3")]
    pub start_revision: i64,
    /// progress_notify is set so that the etcd server will periodically send a WatchResponse with
    /// no events to the new watcher if there are no recent events. It is useful when clients
    /// wish to recover a disconnected watcher starting from a recent known revision.
    /// The etcd server may decide how often it will send notifications based on current load.
    #[prost(bool, tag = "4")]
    pub progress_notify: bool,
    /// filters filter the events at server side before it sends back to the watcher.
    #[prost(enumeration = "watch_create_request::FilterType", repeated, tag = "5")]
    pub filters: ::prost::alloc::vec::Vec<i32>,
    /// If prev_kv is set, created watcher gets the previous KV before the event happens.
    /// If the previous KV is already compacted, nothing will be returned.
    #[prost(bool, tag = "6")]
    pub prev_kv: bool,
    /// If watch_id is provided and non-zero, it will be assigned to this watcher.
    /// Since creating a watcher in etcd is not a synchronous operation,
    /// this can be used ensure that ordering is correct when creating multiple
    /// watchers on the same stream. Creating a watcher with an ID already in
    /// use on the stream will cause an error to be returned.
    #[prost(int64, tag = "7")]
    pub watch_id: i64,
    /// fragment enables splitting large revisions into multiple watch responses.
    #[prost(bool, tag = "8")]
    pub fragment: bool,
}
/// Nested message and enum types in `WatchCreateRequest`.
pub mod watch_create_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FilterType {
        /// filter out put event.
        Noput = 0,
        /// filter out delete event.
        Nodelete = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchCancelRequest {
    /// watch_id is the watcher id to cancel so that no more events are transmitted.
    #[prost(int64, tag = "1")]
    pub watch_id: i64,
}
/// Requests the a watch stream progress status be sent in the watch response stream as soon as
/// possible.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchProgressRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// watch_id is the ID of the watcher that corresponds to the response.
    #[prost(int64, tag = "2")]
    pub watch_id: i64,
    /// created is set to true if the response is for a create watch request.
    /// The client should record the watch_id and expect to receive events for
    /// the created watcher from the same stream.
    /// All events sent to the created watcher will attach with the same watch_id.
    #[prost(bool, tag = "3")]
    pub created: bool,
    /// canceled is set to true if the response is for a cancel watch request.
    /// No further events will be sent to the canceled watcher.
    #[prost(bool, tag = "4")]
    pub canceled: bool,
    /// compact_revision is set to the minimum index if a watcher tries to watch
    /// at a compacted index.
    ///
    /// This happens when creating a watcher at a compacted revision or the watcher cannot
    /// catch up with the progress of the key-value store.
    ///
    /// The client should treat the watcher as canceled and should not try to create any
    /// watcher with the same start_revision again.
    #[prost(int64, tag = "5")]
    pub compact_revision: i64,
    /// cancel_reason indicates the reason for canceling the watcher.
    #[prost(string, tag = "6")]
    pub cancel_reason: ::prost::alloc::string::String,
    /// framgment is true if large watch response was split over multiple responses.
    #[prost(bool, tag = "7")]
    pub fragment: bool,
    #[prost(message, repeated, tag = "11")]
    pub events: ::prost::alloc::vec::Vec<super::mvccpb::Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseGrantRequest {
    /// TTL is the advisory time-to-live in seconds. Expired lease will return -1.
    #[prost(int64, tag = "1")]
    pub ttl: i64,
    /// ID is the requested ID for the lease. If ID is set to 0, the lessor chooses an ID.
    #[prost(int64, tag = "2")]
    pub id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseGrantResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// ID is the lease ID for the granted lease.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// TTL is the server chosen lease time-to-live in seconds.
    #[prost(int64, tag = "3")]
    pub ttl: i64,
    #[prost(string, tag = "4")]
    pub error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseRevokeRequest {
    /// ID is the lease ID to revoke. When the ID is revoked, all associated keys will be deleted.
    #[prost(int64, tag = "1")]
    pub id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseRevokeResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseCheckpoint {
    /// ID is the lease ID to checkpoint.
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// Remaining_TTL is the remaining time until expiry of the lease.
    #[prost(int64, tag = "2")]
    pub remaining_ttl: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseCheckpointRequest {
    #[prost(message, repeated, tag = "1")]
    pub checkpoints: ::prost::alloc::vec::Vec<LeaseCheckpoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseCheckpointResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseKeepAliveRequest {
    /// ID is the lease ID for the lease to keep alive.
    #[prost(int64, tag = "1")]
    pub id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseKeepAliveResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// ID is the lease ID from the keep alive request.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// TTL is the new time-to-live for the lease.
    #[prost(int64, tag = "3")]
    pub ttl: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseTimeToLiveRequest {
    /// ID is the lease ID for the lease.
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// keys is true to query all the keys attached to this lease.
    #[prost(bool, tag = "2")]
    pub keys: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseTimeToLiveResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// ID is the lease ID from the keep alive request.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// TTL is the remaining TTL in seconds for the lease; the lease will expire in under TTL+1 seconds.
    #[prost(int64, tag = "3")]
    pub ttl: i64,
    /// GrantedTTL is the initial granted time in seconds upon lease creation/renewal.
    #[prost(int64, tag = "4")]
    pub granted_ttl: i64,
    /// Keys is the list of keys attached to this lease.
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseLeasesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseStatus {
    /// TODO: int64 TTL = 2;
    #[prost(int64, tag = "1")]
    pub id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseLeasesResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, repeated, tag = "2")]
    pub leases: ::prost::alloc::vec::Vec<LeaseStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Member {
    /// ID is the member ID for this member.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// name is the human-readable name of the member. If the member is not started, the name will be an empty string.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// peerURLs is the list of URLs the member exposes to the cluster for communication.
    #[prost(string, repeated, tag = "3")]
    pub peer_ur_ls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// clientURLs is the list of URLs the member exposes to clients for communication. If the member is not started, clientURLs will be empty.
    #[prost(string, repeated, tag = "4")]
    pub client_ur_ls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// isLearner indicates if the member is raft learner.
    #[prost(bool, tag = "5")]
    pub is_learner: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberAddRequest {
    /// peerURLs is the list of URLs the added member will use to communicate with the cluster.
    #[prost(string, repeated, tag = "1")]
    pub peer_ur_ls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// isLearner indicates if the added member is raft learner.
    #[prost(bool, tag = "2")]
    pub is_learner: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberAddResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// member is the member information for the added member.
    #[prost(message, optional, tag = "2")]
    pub member: ::core::option::Option<Member>,
    /// members is a list of all members after adding the new member.
    #[prost(message, repeated, tag = "3")]
    pub members: ::prost::alloc::vec::Vec<Member>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberRemoveRequest {
    /// ID is the member ID of the member to remove.
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberRemoveResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// members is a list of all members after removing the member.
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<Member>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberUpdateRequest {
    /// ID is the member ID of the member to update.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// peerURLs is the new list of URLs the member will use to communicate with the cluster.
    #[prost(string, repeated, tag = "2")]
    pub peer_ur_ls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberUpdateResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// members is a list of all members after updating the member.
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<Member>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberListRequest {
    #[prost(bool, tag = "1")]
    pub linearizable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberListResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// members is a list of all members associated with the cluster.
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<Member>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberPromoteRequest {
    /// ID is the member ID of the member to promote.
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberPromoteResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// members is a list of all members after promoting the member.
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<Member>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefragmentRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefragmentResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveLeaderRequest {
    /// targetID is the node ID for the new leader.
    #[prost(uint64, tag = "1")]
    pub target_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveLeaderResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlarmRequest {
    /// action is the kind of alarm request to issue. The action
    /// may GET alarm statuses, ACTIVATE an alarm, or DEACTIVATE a
    /// raised alarm.
    #[prost(enumeration = "alarm_request::AlarmAction", tag = "1")]
    pub action: i32,
    /// memberID is the ID of the member associated with the alarm. If memberID is 0, the
    /// alarm request covers all members.
    #[prost(uint64, tag = "2")]
    pub member_id: u64,
    /// alarm is the type of alarm to consider for this request.
    #[prost(enumeration = "AlarmType", tag = "3")]
    pub alarm: i32,
}
/// Nested message and enum types in `AlarmRequest`.
pub mod alarm_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AlarmAction {
        Get = 0,
        Activate = 1,
        Deactivate = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlarmMember {
    /// memberID is the ID of the member associated with the raised alarm.
    #[prost(uint64, tag = "1")]
    pub member_id: u64,
    /// alarm is the type of alarm which has been raised.
    #[prost(enumeration = "AlarmType", tag = "2")]
    pub alarm: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlarmResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// alarms is a list of alarms associated with the alarm request.
    #[prost(message, repeated, tag = "2")]
    pub alarms: ::prost::alloc::vec::Vec<AlarmMember>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DowngradeRequest {
    /// action is the kind of downgrade request to issue. The action may
    /// VALIDATE the target version, DOWNGRADE the cluster version,
    /// or CANCEL the current downgrading job.
    #[prost(enumeration = "downgrade_request::DowngradeAction", tag = "1")]
    pub action: i32,
    /// version is the target version to downgrade.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DowngradeRequest`.
pub mod downgrade_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DowngradeAction {
        Validate = 0,
        Enable = 1,
        Cancel = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DowngradeResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// version is the current cluster version.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// version is the cluster protocol version used by the responding member.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// dbSize is the size of the backend database physically allocated, in bytes, of the responding member.
    #[prost(int64, tag = "3")]
    pub db_size: i64,
    /// leader is the member ID which the responding member believes is the current leader.
    #[prost(uint64, tag = "4")]
    pub leader: u64,
    /// raftIndex is the current raft committed index of the responding member.
    #[prost(uint64, tag = "5")]
    pub raft_index: u64,
    /// raftTerm is the current raft term of the responding member.
    #[prost(uint64, tag = "6")]
    pub raft_term: u64,
    /// raftAppliedIndex is the current raft applied index of the responding member.
    #[prost(uint64, tag = "7")]
    pub raft_applied_index: u64,
    /// errors contains alarm/health information and status.
    #[prost(string, repeated, tag = "8")]
    pub errors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// dbSizeInUse is the size of the backend database logically in use, in bytes, of the responding member.
    #[prost(int64, tag = "9")]
    pub db_size_in_use: i64,
    /// isLearner indicates if the member is raft learner.
    #[prost(bool, tag = "10")]
    pub is_learner: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthEnableRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthDisableRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthStatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserAddRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<super::authpb::UserAddOptions>,
    #[prost(string, tag = "4")]
    pub hashed_password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserGetRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserDeleteRequest {
    /// name is the name of the user to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserChangePasswordRequest {
    /// name is the name of the user whose password is being changed.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// password is the new password for the user. Note that this field will be removed in the API layer.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    /// hashedPassword is the new password for the user. Note that this field will be initialized in the API layer.
    #[prost(string, tag = "3")]
    pub hashed_password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserGrantRoleRequest {
    /// user is the name of the user which should be granted a given role.
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    /// role is the name of the role to grant to the user.
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserRevokeRoleRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleAddRequest {
    /// name is the name of the role to add to the authentication system.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleGetRequest {
    #[prost(string, tag = "1")]
    pub role: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserListRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleListRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleDeleteRequest {
    #[prost(string, tag = "1")]
    pub role: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleGrantPermissionRequest {
    /// name is the name of the role which will be granted the permission.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// perm is the permission to grant to the role.
    #[prost(message, optional, tag = "2")]
    pub perm: ::core::option::Option<super::authpb::Permission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleRevokePermissionRequest {
    #[prost(string, tag = "1")]
    pub role: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub range_end: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthEnableResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthDisableResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
    /// authRevision is the current revision of auth store
    #[prost(uint64, tag = "3")]
    pub auth_revision: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// token is an authorized token that can be used in succeeding RPCs
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserAddResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserGetResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(string, repeated, tag = "2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserDeleteResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserChangePasswordResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserGrantRoleResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserRevokeRoleResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleAddResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleGetResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, repeated, tag = "2")]
    pub perm: ::prost::alloc::vec::Vec<super::authpb::Permission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleListResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(string, repeated, tag = "2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthUserListResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(string, repeated, tag = "2")]
    pub users: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleDeleteResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleGrantPermissionResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRoleRevokePermissionResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AlarmType {
    /// default, used to query if any alarm is active
    None = 0,
    /// space quota is exhausted
    Nospace = 1,
    /// kv store corruption detected
    Corrupt = 2,
}
#[doc = r" Generated client implementations."]
pub mod kv_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct KvClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl KvClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> KvClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> KvClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            KvClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Range gets the keys in the range from the key-value store."]
        pub async fn range(
            &mut self,
            request: impl tonic::IntoRequest<super::RangeRequest>,
        ) -> Result<tonic::Response<super::RangeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/Range");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Put puts the given key into the key-value store."]
        #[doc = " A put request increments the revision of the key-value store"]
        #[doc = " and generates one event in the event history."]
        pub async fn put(
            &mut self,
            request: impl tonic::IntoRequest<super::PutRequest>,
        ) -> Result<tonic::Response<super::PutResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/Put");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DeleteRange deletes the given range from the key-value store."]
        #[doc = " A delete request increments the revision of the key-value store"]
        #[doc = " and generates a delete event in the event history for every deleted key."]
        pub async fn delete_range(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRangeRequest>,
        ) -> Result<tonic::Response<super::DeleteRangeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/DeleteRange");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Txn processes multiple requests in a single transaction."]
        #[doc = " A txn request increments the revision of the key-value store"]
        #[doc = " and generates events with the same revision for every completed request."]
        #[doc = " It is not allowed to modify the same key several times within one txn."]
        pub async fn txn(
            &mut self,
            request: impl tonic::IntoRequest<super::TxnRequest>,
        ) -> Result<tonic::Response<super::TxnResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/Txn");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Compact compacts the event history in the etcd key-value store. The key-value"]
        #[doc = " store should be periodically compacted or the event history will continue to grow"]
        #[doc = " indefinitely."]
        pub async fn compact(
            &mut self,
            request: impl tonic::IntoRequest<super::CompactionRequest>,
        ) -> Result<tonic::Response<super::CompactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/Compact");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod watch_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct WatchClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WatchClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WatchClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> WatchClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            WatchClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Watch watches for events happening or that have happened. Both input and output"]
        #[doc = " are streams; the input stream is for creating and canceling watchers and the output"]
        #[doc = " stream sends events. One watch RPC can watch on multiple key ranges, streaming events"]
        #[doc = " for several watches at once. The entire event history can be watched starting from the"]
        #[doc = " last compaction revision."]
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::WatchRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::WatchResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Watch/Watch");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod lease_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct LeaseClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LeaseClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LeaseClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> LeaseClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            LeaseClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " LeaseGrant creates a lease which expires if the server does not receive a keepAlive"]
        #[doc = " within a given time to live period. All keys attached to the lease will be expired and"]
        #[doc = " deleted if the lease expires. Each expired key generates a delete event in the event history."]
        pub async fn lease_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaseGrantRequest>,
        ) -> Result<tonic::Response<super::LeaseGrantResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseGrant");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " LeaseRevoke revokes a lease. All keys attached to the lease will expire and be deleted."]
        pub async fn lease_revoke(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaseRevokeRequest>,
        ) -> Result<tonic::Response<super::LeaseRevokeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseRevoke");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " LeaseKeepAlive keeps the lease alive by streaming keep alive requests from the client"]
        #[doc = " to the server and streaming keep alive responses from the server to the client."]
        pub async fn lease_keep_alive(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::LeaseKeepAliveRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::LeaseKeepAliveResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseKeepAlive");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " LeaseTimeToLive retrieves lease information."]
        pub async fn lease_time_to_live(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaseTimeToLiveRequest>,
        ) -> Result<tonic::Response<super::LeaseTimeToLiveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseTimeToLive");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " LeaseLeases lists all existing leases."]
        pub async fn lease_leases(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaseLeasesRequest>,
        ) -> Result<tonic::Response<super::LeaseLeasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseLeases");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod cluster_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ClusterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClusterClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ClusterClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ClusterClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ClusterClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " MemberAdd adds a member into the cluster."]
        pub async fn member_add(
            &mut self,
            request: impl tonic::IntoRequest<super::MemberAddRequest>,
        ) -> Result<tonic::Response<super::MemberAddResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Cluster/MemberAdd");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " MemberRemove removes an existing member from the cluster."]
        pub async fn member_remove(
            &mut self,
            request: impl tonic::IntoRequest<super::MemberRemoveRequest>,
        ) -> Result<tonic::Response<super::MemberRemoveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Cluster/MemberRemove");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " MemberUpdate updates the member configuration."]
        pub async fn member_update(
            &mut self,
            request: impl tonic::IntoRequest<super::MemberUpdateRequest>,
        ) -> Result<tonic::Response<super::MemberUpdateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Cluster/MemberUpdate");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " MemberList lists all the members in the cluster."]
        pub async fn member_list(
            &mut self,
            request: impl tonic::IntoRequest<super::MemberListRequest>,
        ) -> Result<tonic::Response<super::MemberListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Cluster/MemberList");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " MemberPromote promotes a member from raft learner (non-voting) to raft voting member."]
        pub async fn member_promote(
            &mut self,
            request: impl tonic::IntoRequest<super::MemberPromoteRequest>,
        ) -> Result<tonic::Response<super::MemberPromoteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Cluster/MemberPromote");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod maintenance_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MaintenanceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MaintenanceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MaintenanceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MaintenanceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MaintenanceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Alarm activates, deactivates, and queries alarms regarding cluster health."]
        pub async fn alarm(
            &mut self,
            request: impl tonic::IntoRequest<super::AlarmRequest>,
        ) -> Result<tonic::Response<super::AlarmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Maintenance/Alarm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Status gets the status of the member."]
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Maintenance/Status");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Defragment defragments a member's backend database to recover storage space."]
        pub async fn defragment(
            &mut self,
            request: impl tonic::IntoRequest<super::DefragmentRequest>,
        ) -> Result<tonic::Response<super::DefragmentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Maintenance/Defragment");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Hash computes the hash of whole backend keyspace,"]
        #[doc = " including key, lease, and other buckets in storage."]
        #[doc = " This is designed for testing ONLY!"]
        #[doc = " Do not rely on this in production with ongoing transactions,"]
        #[doc = " since Hash operation does not hold MVCC locks."]
        #[doc = " Use \"HashKV\" API instead for \"key\" bucket consistency checks."]
        pub async fn hash(
            &mut self,
            request: impl tonic::IntoRequest<super::HashRequest>,
        ) -> Result<tonic::Response<super::HashResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Maintenance/Hash");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " HashKV computes the hash of all MVCC keys up to a given revision."]
        #[doc = " It only iterates \"key\" bucket in backend storage."]
        pub async fn hash_kv(
            &mut self,
            request: impl tonic::IntoRequest<super::HashKvRequest>,
        ) -> Result<tonic::Response<super::HashKvResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Maintenance/HashKV");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Snapshot sends a snapshot of the entire backend from a member over a stream to a client."]
        pub async fn snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::SnapshotRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SnapshotResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Maintenance/Snapshot");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " MoveLeader requests current leader node to transfer its leadership to transferee."]
        pub async fn move_leader(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveLeaderRequest>,
        ) -> Result<tonic::Response<super::MoveLeaderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Maintenance/MoveLeader");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Downgrade requests downgrades, verifies feasibility or cancels downgrade"]
        #[doc = " on the cluster version."]
        #[doc = " Supported since etcd 3.5."]
        pub async fn downgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::DowngradeRequest>,
        ) -> Result<tonic::Response<super::DowngradeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Maintenance/Downgrade");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod auth_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct AuthClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AuthClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> AuthClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            AuthClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " AuthEnable enables authentication."]
        pub async fn auth_enable(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthEnableRequest>,
        ) -> Result<tonic::Response<super::AuthEnableResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/AuthEnable");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " AuthDisable disables authentication."]
        pub async fn auth_disable(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthDisableRequest>,
        ) -> Result<tonic::Response<super::AuthDisableResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/AuthDisable");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " AuthStatus displays authentication status."]
        pub async fn auth_status(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthStatusRequest>,
        ) -> Result<tonic::Response<super::AuthStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/AuthStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate processes an authenticate request."]
        pub async fn authenticate(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateRequest>,
        ) -> Result<tonic::Response<super::AuthenticateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/Authenticate");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UserAdd adds a new user. User name cannot be empty."]
        pub async fn user_add(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthUserAddRequest>,
        ) -> Result<tonic::Response<super::AuthUserAddResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserAdd");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UserGet gets detailed user information."]
        pub async fn user_get(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthUserGetRequest>,
        ) -> Result<tonic::Response<super::AuthUserGetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserGet");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UserList gets a list of all users."]
        pub async fn user_list(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthUserListRequest>,
        ) -> Result<tonic::Response<super::AuthUserListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserList");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UserDelete deletes a specified user."]
        pub async fn user_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthUserDeleteRequest>,
        ) -> Result<tonic::Response<super::AuthUserDeleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserDelete");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UserChangePassword changes the password of a specified user."]
        pub async fn user_change_password(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthUserChangePasswordRequest>,
        ) -> Result<tonic::Response<super::AuthUserChangePasswordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserChangePassword");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UserGrant grants a role to a specified user."]
        pub async fn user_grant_role(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthUserGrantRoleRequest>,
        ) -> Result<tonic::Response<super::AuthUserGrantRoleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserGrantRole");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UserRevokeRole revokes a role of specified user."]
        pub async fn user_revoke_role(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthUserRevokeRoleRequest>,
        ) -> Result<tonic::Response<super::AuthUserRevokeRoleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserRevokeRole");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " RoleAdd adds a new role. Role name cannot be empty."]
        pub async fn role_add(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRoleAddRequest>,
        ) -> Result<tonic::Response<super::AuthRoleAddResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleAdd");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " RoleGet gets detailed role information."]
        pub async fn role_get(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRoleGetRequest>,
        ) -> Result<tonic::Response<super::AuthRoleGetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleGet");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " RoleList gets lists of all roles."]
        pub async fn role_list(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRoleListRequest>,
        ) -> Result<tonic::Response<super::AuthRoleListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleList");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " RoleDelete deletes a specified role."]
        pub async fn role_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRoleDeleteRequest>,
        ) -> Result<tonic::Response<super::AuthRoleDeleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleDelete");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " RoleGrantPermission grants a permission of a specified key or range to a specified role."]
        pub async fn role_grant_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRoleGrantPermissionRequest>,
        ) -> Result<tonic::Response<super::AuthRoleGrantPermissionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleGrantPermission");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " RoleRevokePermission revokes a key or range permission of a specified role."]
        pub async fn role_revoke_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRoleRevokePermissionRequest>,
        ) -> Result<tonic::Response<super::AuthRoleRevokePermissionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleRevokePermission");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
