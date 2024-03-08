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
}
