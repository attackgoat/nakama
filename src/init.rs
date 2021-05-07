use {super::{sys::{str_as_nk_string,NkInitializer}, Context, Db, NakamaModule, Logger}, std::mem::MaybeUninit};

pub struct Initializer(NkInitializer);

impl Initializer {
    // pub fn register_rpc(&self, id: String, f: extern "C" fn(&Context, &Logger, &Db, &NakamaModule, )) -> Result<(), String>
    // {
    //     let mut out_err = MaybeUninit::uninit();

    //     let res = {
    //         let id_str = id.as_ref();
    //         let id_nk_string = str_as_nk_string(id_str);
    //         let out_err_ptr = out_err.as_mut_ptr();

    //         let register_rpc = self.0.registerrpc.unwrap();

    //         unsafe {
    //             register_rpc(self.0.ptr, id_nk_string, Some(f), out_err_ptr)
    //         }
    //     };

    //     Ok(())
    // }

    // RegisterRpc(id string, fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, payload string) (string, error)) error
    // RegisterBeforeRt(id string, fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, envelope *rtapi.Envelope) (*rtapi.Envelope, error)) error
    // RegisterAfterRt(id string, fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, envelope *rtapi.Envelope) error) error
    // RegisterMatchmakerMatched(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, entries []MatchmakerEntry) (string, error)) error
    // RegisterMatch(name string, fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule) (Match, error)) error
    // RegisterTournamentEnd(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, tournament *api.Tournament, end, reset int64) error) error
    // RegisterTournamentReset(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, tournament *api.Tournament, end, reset int64) error) error
    // RegisterLeaderboardReset(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, leaderboard Leaderboard, reset int64) error) error
    // RegisterBeforeGetAccount(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule) error) error
    // RegisterAfterGetAccount(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Account) error) error
    // RegisterBeforeUpdateAccount(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.UpdateAccountRequest) (*api.UpdateAccountRequest, error)) error
    // RegisterAfterUpdateAccount(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.UpdateAccountRequest) error) error
    // RegisterBeforeSessionRefresh(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.SessionRefreshRequest) (*api.SessionRefreshRequest, error)) error
    // RegisterAfterSessionRefresh(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.SessionRefreshRequest) error) error
    // RegisterBeforeAuthenticateApple(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AuthenticateAppleRequest) (*api.AuthenticateAppleRequest, error)) error
    // RegisterAfterAuthenticateApple(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.AuthenticateAppleRequest) error) error
    // RegisterBeforeAuthenticateCustom(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AuthenticateCustomRequest) (*api.AuthenticateCustomRequest, error)) error
    // RegisterAfterAuthenticateCustom(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.AuthenticateCustomRequest) error) error
    // RegisterBeforeAuthenticateDevice(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AuthenticateDeviceRequest) (*api.AuthenticateDeviceRequest, error)) error
    // RegisterAfterAuthenticateDevice(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.AuthenticateDeviceRequest) error) error
    // RegisterBeforeAuthenticateEmail(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AuthenticateEmailRequest) (*api.AuthenticateEmailRequest, error)) error
    // RegisterAfterAuthenticateEmail(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.AuthenticateEmailRequest) error) error
    // RegisterBeforeAuthenticateFacebook(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AuthenticateFacebookRequest) (*api.AuthenticateFacebookRequest, error)) error
    // RegisterAfterAuthenticateFacebook(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.AuthenticateFacebookRequest) error) error
    // RegisterBeforeAuthenticateFacebookInstantGame(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AuthenticateFacebookInstantGameRequest) (*api.AuthenticateFacebookInstantGameRequest, error)) error
    // RegisterAfterAuthenticateFacebookInstantGame(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.AuthenticateFacebookInstantGameRequest) error) error
    // RegisterBeforeAuthenticateGameCenter(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AuthenticateGameCenterRequest) (*api.AuthenticateGameCenterRequest, error)) error
    // RegisterAfterAuthenticateGameCenter(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.AuthenticateGameCenterRequest) error) error
    // RegisterBeforeAuthenticateGoogle(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AuthenticateGoogleRequest) (*api.AuthenticateGoogleRequest, error)) error
    // RegisterAfterAuthenticateGoogle(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.AuthenticateGoogleRequest) error) error
    // RegisterBeforeAuthenticateSteam(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AuthenticateSteamRequest) (*api.AuthenticateSteamRequest, error)) error
    // RegisterAfterAuthenticateSteam(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Session, in *api.AuthenticateSteamRequest) error) error
    // RegisterBeforeListChannelMessages(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListChannelMessagesRequest) (*api.ListChannelMessagesRequest, error)) error
    // RegisterAfterListChannelMessages(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.ChannelMessageList, in *api.ListChannelMessagesRequest) error) error
    // RegisterBeforeListFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListFriendsRequest) (*api.ListFriendsRequest, error)) error
    // RegisterAfterListFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.FriendList) error) error
    // RegisterBeforeAddFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AddFriendsRequest) (*api.AddFriendsRequest, error)) error
    // RegisterAfterAddFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AddFriendsRequest) error) error
    // RegisterBeforeDeleteFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteFriendsRequest) (*api.DeleteFriendsRequest, error)) error
    // RegisterAfterDeleteFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteFriendsRequest) error) error
    // RegisterBeforeBlockFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.BlockFriendsRequest) (*api.BlockFriendsRequest, error)) error
    // RegisterAfterBlockFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.BlockFriendsRequest) error) error
    // RegisterBeforeImportFacebookFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ImportFacebookFriendsRequest) (*api.ImportFacebookFriendsRequest, error)) error
    // RegisterAfterImportFacebookFriends(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ImportFacebookFriendsRequest) error) error
    // RegisterBeforeCreateGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.CreateGroupRequest) (*api.CreateGroupRequest, error)) error
    // RegisterAfterCreateGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Group, in *api.CreateGroupRequest) error) error
    // RegisterBeforeUpdateGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.UpdateGroupRequest) (*api.UpdateGroupRequest, error)) error
    // RegisterAfterUpdateGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.UpdateGroupRequest) error) error
    // RegisterBeforeDeleteGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteGroupRequest) (*api.DeleteGroupRequest, error)) error
    // RegisterAfterDeleteGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteGroupRequest) error) error
    // RegisterBeforeJoinGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.JoinGroupRequest) (*api.JoinGroupRequest, error)) error
    // RegisterAfterJoinGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.JoinGroupRequest) error) error
    // RegisterBeforeLeaveGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.LeaveGroupRequest) (*api.LeaveGroupRequest, error)) error
    // RegisterAfterLeaveGroup(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.LeaveGroupRequest) error) error
    // RegisterBeforeAddGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AddGroupUsersRequest) (*api.AddGroupUsersRequest, error)) error
    // RegisterAfterAddGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AddGroupUsersRequest) error) error
    // RegisterBeforeBanGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.BanGroupUsersRequest) (*api.BanGroupUsersRequest, error)) error
    // RegisterAfterBanGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.BanGroupUsersRequest) error) error
    // RegisterBeforeKickGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.KickGroupUsersRequest) (*api.KickGroupUsersRequest, error)) error
    // RegisterAfterKickGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.KickGroupUsersRequest) error) error
    // RegisterBeforePromoteGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.PromoteGroupUsersRequest) (*api.PromoteGroupUsersRequest, error)) error
    // RegisterAfterPromoteGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.PromoteGroupUsersRequest) error) error
    // RegisterBeforeDemoteGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DemoteGroupUsersRequest) (*api.DemoteGroupUsersRequest, error)) error
    // RegisterAfterDemoteGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DemoteGroupUsersRequest) error) error
    // RegisterBeforeListGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListGroupUsersRequest) (*api.ListGroupUsersRequest, error)) error
    // RegisterAfterListGroupUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.GroupUserList, in *api.ListGroupUsersRequest) error) error
    // RegisterBeforeListUserGroups(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListUserGroupsRequest) (*api.ListUserGroupsRequest, error)) error
    // RegisterAfterListUserGroups(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.UserGroupList, in *api.ListUserGroupsRequest) error) error
    // RegisterBeforeListGroups(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListGroupsRequest) (*api.ListGroupsRequest, error)) error
    // RegisterAfterListGroups(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.GroupList, in *api.ListGroupsRequest) error) error
    // RegisterBeforeDeleteLeaderboardRecord(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteLeaderboardRecordRequest) (*api.DeleteLeaderboardRecordRequest, error)) error
    // RegisterAfterDeleteLeaderboardRecord(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteLeaderboardRecordRequest) error) error
    // RegisterBeforeListLeaderboardRecords(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListLeaderboardRecordsRequest) (*api.ListLeaderboardRecordsRequest, error)) error
    // RegisterAfterListLeaderboardRecords(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.LeaderboardRecordList, in *api.ListLeaderboardRecordsRequest) error) error
    // RegisterBeforeWriteLeaderboardRecord(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.WriteLeaderboardRecordRequest) (*api.WriteLeaderboardRecordRequest, error)) error
    // RegisterAfterWriteLeaderboardRecord(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.LeaderboardRecord, in *api.WriteLeaderboardRecordRequest) error) error
    // RegisterBeforeListLeaderboardRecordsAroundOwner(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListLeaderboardRecordsAroundOwnerRequest) (*api.ListLeaderboardRecordsAroundOwnerRequest, error)) error
    // RegisterAfterListLeaderboardRecordsAroundOwner(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.LeaderboardRecordList, in *api.ListLeaderboardRecordsAroundOwnerRequest) error) error
    // RegisterBeforeLinkApple(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountApple) (*api.AccountApple, error)) error
    // RegisterAfterLinkApple(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountApple) error) error
    // RegisterBeforeLinkCustom(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountCustom) (*api.AccountCustom, error)) error
    // RegisterAfterLinkCustom(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountCustom) error) error
    // RegisterBeforeLinkDevice(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountDevice) (*api.AccountDevice, error)) error
    // RegisterAfterLinkDevice(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountDevice) error) error
    // RegisterBeforeLinkEmail(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountEmail) (*api.AccountEmail, error)) error
    // RegisterAfterLinkEmail(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountEmail) error) error
    // RegisterBeforeLinkFacebook(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.LinkFacebookRequest) (*api.LinkFacebookRequest, error)) error
    // RegisterAfterLinkFacebook(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.LinkFacebookRequest) error) error
    // RegisterBeforeLinkFacebookInstantGame(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountFacebookInstantGame) (*api.AccountFacebookInstantGame, error)) error
    // RegisterAfterLinkFacebookInstantGame(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountFacebookInstantGame) error) error
    // RegisterBeforeLinkGameCenter(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountGameCenter) (*api.AccountGameCenter, error)) error
    // RegisterAfterLinkGameCenter(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountGameCenter) error) error
    // RegisterBeforeLinkGoogle(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountGoogle) (*api.AccountGoogle, error)) error
    // RegisterAfterLinkGoogle(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountGoogle) error) error
    // RegisterBeforeLinkSteam(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountSteam) (*api.AccountSteam, error)) error
    // RegisterAfterLinkSteam(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountSteam) error) error
    // RegisterBeforeListMatches(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListMatchesRequest) (*api.ListMatchesRequest, error)) error
    // RegisterAfterListMatches(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.MatchList, in *api.ListMatchesRequest) error) error
    // RegisterBeforeListNotifications(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListNotificationsRequest) (*api.ListNotificationsRequest, error)) error
    // RegisterAfterListNotifications(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.NotificationList, in *api.ListNotificationsRequest) error) error
    // RegisterBeforeDeleteNotification(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteNotificationsRequest) (*api.DeleteNotificationsRequest, error)) error
    // RegisterAfterDeleteNotification(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteNotificationsRequest) error) error
    // RegisterBeforeListStorageObjects(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListStorageObjectsRequest) (*api.ListStorageObjectsRequest, error)) error
    // RegisterAfterListStorageObjects(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.StorageObjectList, in *api.ListStorageObjectsRequest) error) error
    // RegisterBeforeReadStorageObjects(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ReadStorageObjectsRequest) (*api.ReadStorageObjectsRequest, error)) error
    // RegisterAfterReadStorageObjects(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.StorageObjects, in *api.ReadStorageObjectsRequest) error) error
    // RegisterBeforeWriteStorageObjects(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.WriteStorageObjectsRequest) (*api.WriteStorageObjectsRequest, error)) error
    // RegisterAfterWriteStorageObjects(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.StorageObjectAcks, in *api.WriteStorageObjectsRequest) error) error
    // RegisterBeforeDeleteStorageObjects(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteStorageObjectsRequest) (*api.DeleteStorageObjectsRequest, error)) error
    // RegisterAfterDeleteStorageObjects(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.DeleteStorageObjectsRequest) error) error
    // RegisterBeforeJoinTournament(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.JoinTournamentRequest) (*api.JoinTournamentRequest, error)) error
    // RegisterAfterJoinTournament(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.JoinTournamentRequest) error) error
    // RegisterBeforeListTournamentRecords(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListTournamentRecordsRequest) (*api.ListTournamentRecordsRequest, error)) error
    // RegisterAfterListTournamentRecords(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.TournamentRecordList, in *api.ListTournamentRecordsRequest) error) error
    // RegisterBeforeListTournaments(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListTournamentsRequest) (*api.ListTournamentsRequest, error)) error
    // RegisterAfterListTournaments(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.TournamentList, in *api.ListTournamentsRequest) error) error
    // RegisterBeforeWriteTournamentRecord(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.WriteTournamentRecordRequest) (*api.WriteTournamentRecordRequest, error)) error
    // RegisterAfterWriteTournamentRecord(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.LeaderboardRecord, in *api.WriteTournamentRecordRequest) error) error
    // RegisterBeforeListTournamentRecordsAroundOwner(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.ListTournamentRecordsAroundOwnerRequest) (*api.ListTournamentRecordsAroundOwnerRequest, error)) error
    // RegisterAfterListTournamentRecordsAroundOwner(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.TournamentRecordList, in *api.ListTournamentRecordsAroundOwnerRequest) error) error
    // RegisterBeforeUnlinkApple(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountApple) (*api.AccountApple, error)) error
    // RegisterAfterUnlinkApple(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountApple) error) error
    // RegisterBeforeUnlinkCustom(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountCustom) (*api.AccountCustom, error)) error
    // RegisterAfterUnlinkCustom(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountCustom) error) error
    // RegisterBeforeUnlinkDevice(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountDevice) (*api.AccountDevice, error)) error
    // RegisterAfterUnlinkDevice(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountDevice) error) error
    // RegisterBeforeUnlinkEmail(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountEmail) (*api.AccountEmail, error)) error
    // RegisterAfterUnlinkEmail(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountEmail) error) error
    // RegisterBeforeUnlinkFacebook(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountFacebook) (*api.AccountFacebook, error)) error
    // RegisterAfterUnlinkFacebook(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountFacebook) error) error
    // RegisterBeforeUnlinkFacebookInstantGame(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountFacebookInstantGame) (*api.AccountFacebookInstantGame, error)) error
    // RegisterAfterUnlinkFacebookInstantGame(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountFacebookInstantGame) error) error
    // RegisterBeforeUnlinkGameCenter(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountGameCenter) (*api.AccountGameCenter, error)) error
    // RegisterAfterUnlinkGameCenter(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountGameCenter) error) error
    // RegisterBeforeUnlinkGoogle(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountGoogle) (*api.AccountGoogle, error)) error
    // RegisterAfterUnlinkGoogle(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountGoogle) error) error
    // RegisterBeforeUnlinkSteam(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountSteam) (*api.AccountSteam, error)) error
    // RegisterAfterUnlinkSteam(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.AccountSteam) error) error
    // RegisterBeforeGetUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, in *api.GetUsersRequest) (*api.GetUsersRequest, error)) error
    // RegisterAfterGetUsers(fn func(ctx context.Context, logger Logger, db *sql.DB, nk NakamaModule, out *api.Users, in *api.GetUsersRequest) error) error
    // RegisterEvent(fn func(ctx context.Context, logger Logger, evt *api.Event)) error
    // RegisterEventSessionStart(fn func(ctx context.Context, logger Logger, evt *api.Event)) error
    // RegisterEventSessionEnd(fn func(ctx context.Context, logger Logger, evt *api.Event)) error
}

impl From<&NkInitializer> for Initializer {
    fn from(initializer: &NkInitializer) -> Self {
        Self(*initializer)
    }
}