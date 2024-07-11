use std::fmt::Display;

use tonic::Code;
use tonic::Status;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrKind {
    EmptyKey,
    KeyNotFound,
    ValueProvided,
    LeaseProvided,
    TooManyOps,
    DuplicateKey,
    InvalidSortOption,
    Compacted,
    FutureRev,
    NoSpace,
    LeaseNotFound,
    LeaseExist,
    LeaseTTLTooLarge,
    MemberExist,
    PeerURLExist,
    MemberNotEnoughStarted,
    MemberBadURLs,
    MemberNotFound,
    MemberNotLearner,
    MemberLearnerNotReady,
    TooManyLearners,
    RequestTooLarge,
    TooManyRequests,
    RootUserNotExist,
    RootRoleNotExist,
    UserAlreadyExist,
    UserEmpty,
    UserNotFound,
    RoleAlreadyExist,
    RoleNotFound,
    RoleEmpty,
    AuthFailed,
    PermissionDenied,
    RoleNotGranted,
    PermissionNotGranted,
    AuthNotEnabled,
    InvalidAuthToken,
    AuthOldRevision,
    InvalidAuthMgmt,
    NoLeader,
    NotLeader,
    LeaderChanged,
    NotCapable,
    Stopped,
    Timeout,
    TimeoutDueToLeaderFail,
    TimeoutDueToConnectionLost,
    TimeoutWaitAppliedIndex,
    Unhealthy,
    Corrupt,
    BadLeaderTransferee,
    ClusterVersionUnavailable,
    WrongDowngradeVersionFormat,
    InvalidDowngradeTargetVersion,
    DowngradeInProcess,
    NoInflightDowngrade,

    //
    Grpc,
    Endpoint,
    ConnectFailed,
    InvalidData,
    // lease errors
    LeaseRequestFailed,
    // watch errors
    WatchRequestFailed,
    WatchStartFailed,
    WatchCanceled,
    WatchFinished,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrKind,
    cause: Box<dyn std::error::Error + Send + Sync + 'static>,
}

impl Error {
    pub fn new<E>(kind: ErrKind, cause: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        Error {
            kind,
            cause: cause.into(),
        }
    }

    pub fn from_kind(kind: ErrKind) -> Self {
        Self::new(kind, "")
    }

    pub fn kind(&self) -> ErrKind {
        self.kind
    }

    pub fn is_key_not_found(&self) -> bool {
        self.kind == ErrKind::KeyNotFound
    }

    pub fn is_invalid_auth_token(&self) -> bool {
        self.kind == ErrKind::InvalidAuthToken
    }

    pub fn is_auth_not_enabled(&self) -> bool {
        self.kind == ErrKind::AuthNotEnabled
    }

    pub fn should_refresh_token(&self) -> bool {
        self.kind == ErrKind::InvalidAuthToken || self.kind == ErrKind::AuthOldRevision
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {:?}", self.kind, self.cause)
    }
}

impl std::error::Error for Error {}

impl From<Status> for Error {
    fn from(s: Status) -> Self {
        let kind = match (s.code(), s.message()) {
            (Code::InvalidArgument, "etcdserver: key is not provided") => ErrKind::EmptyKey,
            (Code::InvalidArgument, "etcdserver: key not found") => ErrKind::KeyNotFound,
            (Code::InvalidArgument, "etcdserver: value is provided") => ErrKind::ValueProvided,
            (Code::InvalidArgument, "etcdserver: lease is provided") => ErrKind::LeaseProvided,
            (Code::InvalidArgument, "etcdserver: too many operations in txn request") => {
                ErrKind::TooManyOps
            }
            (Code::InvalidArgument, "etcdserver: duplicate key given in txn request") => {
                ErrKind::DuplicateKey
            }
            (Code::InvalidArgument, "etcdserver: invalid sort option") => {
                ErrKind::InvalidSortOption
            }
            (Code::OutOfRange, "etcdserver: mvcc: required revision has been compacted") => {
                ErrKind::Compacted
            }
            (Code::OutOfRange, "etcdserver: mvcc: required revision is a future revision") => {
                ErrKind::FutureRev
            }
            (Code::ResourceExhausted, "etcdserver: mvcc: database space exceeded") => {
                ErrKind::NoSpace
            }
            (Code::NotFound, "etcdserver: requested lease not found") => ErrKind::LeaseNotFound,
            (Code::FailedPrecondition, "etcdserver: lease already exists") => ErrKind::LeaseExist,
            (Code::OutOfRange, "etcdserver: too large lease TTL") => ErrKind::LeaseTTLTooLarge,
            (Code::FailedPrecondition, "etcdserver: member ID already exist") => {
                ErrKind::MemberExist
            }
            (Code::FailedPrecondition, "etcdserver: Peer URLs already exists") => {
                ErrKind::PeerURLExist
            }
            (
                Code::FailedPrecondition,
                "etcdserver: re-configuration failed due to not enough started members",
            ) => ErrKind::MemberNotEnoughStarted,
            (Code::InvalidArgument, "etcdserver: given member URLs are invalid") => {
                ErrKind::MemberBadURLs
            }
            (Code::NotFound, "etcdserver: member not found") => ErrKind::MemberNotFound,
            (Code::FailedPrecondition, "etcdserver: can only promote a learner member") => {
                ErrKind::MemberNotLearner
            }
            (
                Code::FailedPrecondition,
                "etcdserver: can only promote a learner member which is in sync with leader",
            ) => ErrKind::MemberLearnerNotReady,
            (Code::FailedPrecondition, "etcdserver: too many learner members in cluster") => {
                ErrKind::TooManyLearners
            }
            (Code::InvalidArgument, "etcdserver: request is too large") => ErrKind::RequestTooLarge,
            (Code::ResourceExhausted, "etcdserver: too many requests") => ErrKind::TooManyRequests,
            (Code::FailedPrecondition, "etcdserver: root user does not exist") => {
                ErrKind::RootUserNotExist
            }
            (Code::FailedPrecondition, "etcdserver: root user does not have root role") => {
                ErrKind::RootRoleNotExist
            }
            (Code::FailedPrecondition, "etcdserver: user name already exists") => {
                ErrKind::UserAlreadyExist
            }
            (Code::InvalidArgument, "etcdserver: user name is empty") => ErrKind::UserEmpty,
            (Code::FailedPrecondition, "etcdserver: user name not found") => ErrKind::UserNotFound,
            (Code::FailedPrecondition, "etcdserver: role name already exists") => {
                ErrKind::RoleAlreadyExist
            }
            (Code::FailedPrecondition, "etcdserver: role name not found") => ErrKind::RoleNotFound,
            (Code::InvalidArgument, "etcdserver: role name is empty") => ErrKind::RoleEmpty,
            (
                Code::InvalidArgument,
                "etcdserver: authentication failed, invalid user ID or password",
            ) => ErrKind::AuthFailed,
            (Code::PermissionDenied, "etcdserver: permission denied") => ErrKind::PermissionDenied,
            (Code::FailedPrecondition, "etcdserver: role is not granted to the user") => {
                ErrKind::RoleNotGranted
            }
            (Code::FailedPrecondition, "etcdserver: permission is not granted to the role") => {
                ErrKind::PermissionNotGranted
            }
            (Code::FailedPrecondition, "etcdserver: authentication is not enabled") => {
                ErrKind::AuthNotEnabled
            }
            (Code::Unauthenticated, "etcdserver: invalid auth token") => ErrKind::InvalidAuthToken,
            (Code::InvalidArgument, "etcdserver: revision of auth store is old") => {
                ErrKind::AuthOldRevision
            }
            (Code::InvalidArgument, "etcdserver: invalid auth management") => {
                ErrKind::InvalidAuthMgmt
            }
            (Code::Unavailable, "etcdserver: no leader") => ErrKind::NoLeader,
            (Code::FailedPrecondition, "etcdserver: not leader") => ErrKind::NotLeader,
            (Code::Unavailable, "etcdserver: leader changed") => ErrKind::LeaderChanged,
            (Code::FailedPrecondition, "etcdserver: not capable") => ErrKind::NotCapable,
            (Code::Unavailable, "etcdserver: server stopped") => ErrKind::Stopped,
            (Code::Unavailable, "etcdserver: request timed out") => ErrKind::Timeout,
            (
                Code::Unavailable,
                "etcdserver: request timed out, possibly due to previous leader failure",
            ) => ErrKind::TimeoutDueToLeaderFail,
            (
                Code::Unavailable,
                "etcdserver: request timed out, possibly due to connection lost",
            ) => ErrKind::TimeoutDueToConnectionLost,
            (
                Code::Unavailable,
                "etcdserver: request timed out, waiting for the applied index took too long",
            ) => ErrKind::TimeoutWaitAppliedIndex,
            (Code::Unavailable, "etcdserver: unhealthy cluster") => ErrKind::Unhealthy,
            (Code::DataLoss, "etcdserver: corrupt cluster") => ErrKind::Corrupt,
            (Code::FailedPrecondition, "etcdserver: bad leader transferee") => {
                ErrKind::BadLeaderTransferee
            }
            (
                Code::FailedPrecondition,
                "etcdserver: cluster version not found during downgrade",
            ) => ErrKind::ClusterVersionUnavailable,

            (Code::InvalidArgument, "etcdserver: wrong downgrade target version format") => {
                ErrKind::WrongDowngradeVersionFormat
            }
            (Code::InvalidArgument, "etcdserver: invalid downgrade target version") => {
                ErrKind::InvalidDowngradeTargetVersion
            }
            (Code::FailedPrecondition, "etcdserver: cluster has a downgrade job in progress") => {
                ErrKind::DowngradeInProcess
            }
            (Code::FailedPrecondition, "etcdserver: no inflight downgrade job") => {
                ErrKind::NoInflightDowngrade
            }
            _ => ErrKind::Grpc,
        };

        Error::new(kind, s)
    }
}
