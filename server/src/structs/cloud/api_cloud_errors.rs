use serde::{Deserialize, Serialize};
use strum::Display;
use ts_rs::TS;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS, Display)]
#[ts(export)]
pub enum CloudApiErrors {
    TeamDoesNotExist,
    UserDoesNotExist,
    CloudFeatureDisabled,
    InsufficientPermissions,
    TeamHasNoRegisteredApps,
    DatabaseError,
    MaximumUsersPerTeamReached,
    UserAlreadyBelongsToTheTeam,
    IncorrectPassword,
    AccessTokenFailure,
    RefreshTokenFailure,
    AppAlreadyExists,
    MaximumAppsPerTeamReached,
    TeamAlreadyExists,
    PersonalTeamAlreadyExists,
    EmailAlreadyExists,
    InternalServerError,
    UserDoesNotBelongsToTheTeam,
    InvalidName,
    UnauthorizedOriginError,
    AppDoesNotExist,
    UserAlreadyInvitedToTheTeam,
    MaximumInvitesPerTeamReached,
    InviteNotFound,
    ActionForbiddenForPersonalTeam,
    InviteDoesNotExist,
    InvalidPaginationCursor,
    InvalidOrExpiredVerificationCode,
    InvalidOrExpiredAuthCode,
    InvalidDomainName,
    DomainAlreadyVerified,
    DomainVerificationFailure,
    DomainNotFound,
    DomainVerificationNotStarted,
    DomainAlreadyVerifiedByAnotherApp,
    NoPendingDomainVerification,
    WebAuthnError,
    PasswordNotSet,
    UserDoesNotHavePasskey,
    PasskeyAlreadyExists,
    InvalidPasskeyCredential,
    PasskeyDoesNotExist,
    FailedToCreateTeam,
    DashboardImportFail,
    OriginHeaderRequired,
    InvalidOrigin,
    InvalidAction,
    AdminCannotLeaveTeam,
    GrafanaError,
    TeamWithoutGrafanaId,
}
