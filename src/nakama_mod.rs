use {
    super::{
        sys::{str_as_nk_string, NkModule},
        Context,
    },
    std::{ffi::CStr, mem::MaybeUninit},
};

pub struct Authentication {
    pub created: bool,
    pub user_id: String,
    pub user_name: String,
}

pub type AuthenticateResult = Result<Authentication, String>;

pub struct NakamaModule(NkModule);

impl NakamaModule {
    /// Not implemented
    pub fn authenticate_apple(&self) {
        // ctx context.Context, token, username string, create bool) (string, string, bool, error)
        todo!();
    }

    /// Not implemented
    pub fn authenticate_custom(&self) {
        // ctx context.Context, id, username string, create bool) (string, string, bool, error)
        todo!();
    }

    /// Not implemented
    pub fn authenticate_device<S1, S2>(
        &self,
        ctx: &Context,
        user_id: S1,
        user_name: S2,
        create: bool,
    ) -> AuthenticateResult
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut out_user_id = MaybeUninit::uninit();
        let out_user_id_ptr = out_user_id.as_mut_ptr();

        let mut out_username = MaybeUninit::uninit();
        let out_username_ptr = out_username.as_mut_ptr();

        let mut out_err = MaybeUninit::uninit();
        let out_err_ptr = out_err.as_mut_ptr();

        let mut out_created = MaybeUninit::uninit();
        let out_created_ptr = out_created.as_mut_ptr();

        let res = unsafe {
            self.0.authenticatedevice.unwrap()(
                self.0.ptr,
                ctx.as_ptr(),
                str_as_nk_string(user_id),
                str_as_nk_string(user_name),
                create,
                out_user_id_ptr,
                out_username_ptr,
                out_err_ptr,
                out_created_ptr,
            )
        };

        if res == 0 {
            Ok(Authentication {
                created: unsafe { *out_created.assume_init() },
                user_id: unsafe {
                    CStr::from_ptr(*out_user_id_ptr)
                        .to_str()
                        .unwrap()
                        .to_owned()
                }
                .clone(),
                user_name: unsafe {
                    CStr::from_ptr(*out_username_ptr)
                        .to_str()
                        .unwrap()
                        .to_owned()
                }
                .clone(),
            })
        } else {
            Err(unsafe { CStr::from_ptr(*out_err_ptr).to_str().unwrap().to_owned() }.clone())
        }
    }

    /// Not implemented
    pub fn authenticate_email(&self) {
        // ctx context.Context, email, password, username string, create bool) (string, string, bool, error)
        todo!();
    }

    /// Not implemented
    pub fn authenticate_facebook(&self) {
        // ctx context.Context, token string, importFriends bool, username string, create bool) (string, string, bool, error)
        todo!();
    }

    /// Not implemented
    pub fn authenticate_facebook_instant_game(&self) {
        // ctx context.Context, signedPlayerInfo string, username string, create bool) (string, string, bool, error)
        todo!();
    }

    /// Not implemented
    pub fn authenticate_game_center(&self) {
        // ctx context.Context, playerID, bundleID string, timestamp int64, salt, signature, publicKeyUrl, username string, create bool) (string, string, bool, error)
        todo!();
    }

    /// Not implemented
    pub fn authenticate_google(&self) {
        // ctx context.Context, token, username string, create bool) (string, string, bool, error)
        todo!();
    }

    /// Not implemented
    pub fn authenticate_steam(&self) {
        // ctx context.Context, token, username string, create bool) (string, string, bool, error)
        todo!();
    }

    /// Not implemented
    pub fn authenticate_token_generate(&self) {
        // userID, username string, exp int64, vars map[string]string) (string, int64, error)
        todo!();
    }

    /// Not implemented
    pub fn account_get_id(&self) {
        // ctx context.Context, userID string) (*api.Account, error)
        todo!();
    }

    /// Not implemented
    pub fn account_update_id(&self) {
        // ctx context.Context, userID, username string, metadata map[string]interface{}, displayName, timezone, location, langTag, avatarUrl string) error
        todo!();
    }

    /// Not implemented
    pub fn account_delete_id(&self) {
        // ctx context.Context, userID string, recorded bool) error
        todo!();
    }

    /// Not implemented
    pub fn account_export_id(&self) {
        // ctx context.Context, userID string) (string, error)
        todo!();
    }

    /// Not implemented
    pub fn accounts_get_id(&self) {
        // ctx context.Context, userIDs []string) ([]*api.Account, error)
        todo!();
    }

    /// Not implemented
    pub fn event(&self) {
        // ctx context.Context, evt *api.Event) error
        todo!();
    }

    /// Not implemented
    pub fn friends_list(&self) {
        // ctx context.Context, userID string, limit int, state *int, cursor string) ([]*api.Friend, string, error)
        todo!();
    }

    /// Not implemented
    pub fn group_create(&self) {
        // ctx context.Context, userID, name, creatorID, langTag, description, avatarUrl string, open bool, metadata map[string]interface{}, maxCount int) (*api.Group, error)
        todo!();
    }

    /// Not implemented
    pub fn group_delete(&self) {
        // ctx context.Context, id string) error
        todo!();
    }

    /// Not implemented
    pub fn group_update(&self) {
        // ctx context.Context, id, name, creatorID, langTag, description, avatarUrl string, open bool, metadata map[string]interface{}, maxCount int) error
        todo!();
    }

    /// Not implemented
    pub fn group_user_join(&self) {
        // ctx context.Context, groupID, userID, username string) error
        todo!();
    }

    /// Not implemented
    pub fn group_user_leave(&self) {
        // ctx context.Context, groupID, userID, username string) error
        todo!();
    }

    /// Not implemented
    pub fn group_users_add(&self) {
        // ctx context.Context, groupID string, userIDs []string) error
        todo!();
    }

    /// Not implemented
    pub fn group_users_demote(&self) {
        // ctx context.Context, groupID string, userIDs []string) error
        todo!();
    }

    /// Not implemented
    pub fn group_users_kick(&self) {
        // ctx context.Context, groupID string, userIDs []string) error
        todo!();
    }

    /// Not implemented
    pub fn group_users_list(&self) {
        // ctx context.Context, id string, limit int, state *int, cursor string) ([]*api.GroupUserList_GroupUser, string, error)
        todo!();
    }

    /// Not implemented
    pub fn group_users_promote(&self) {
        // ctx context.Context, groupID string, userIDs []string) error
        todo!();
    }

    /// Not implemented
    pub fn groups_get_id(&self) {
        // ctx context.Context, groupIDs []string) ([]*api.Group, error)
        todo!();
    }

    /// Not implemented
    pub fn leaderboard_create(&self) {
        // ctx context.Context, id string, authoritative bool, sortOrder, operator, resetSchedule string, metadata map[string]interface{}) error
        todo!();
    }

    /// Not implemented
    pub fn leaderboard_delete(&self) {
        // ctx context.Context, id string) error
        todo!();
    }

    /// Not implemented
    pub fn leaderboard_record_write(&self) {
        // ctx context.Context, id, ownerID, username string, score, subscore int64, metadata map[string]interface{}) (*api.LeaderboardRecord, error)
        todo!();
    }

    /// Not implemented
    pub fn leaderboard_record_delete(&self) {
        // ctx context.Context, id, ownerID string) error
        todo!();
    }

    /// Not implemented
    pub fn leaderboard_records_list(&self) {
        // ctx context.Context, id string, ownerIDs []string, limit int, cursor string, expiry int64) ([]*api.LeaderboardRecord, []*api.LeaderboardRecord, string, string, error)
        todo!();
    }

    /// Not implemented
    pub fn link_apple(&self) {
        // ctx context.Context, userID, token string) error
        todo!();
    }

    /// Not implemented
    pub fn link_custom(&self) {
        // ctx context.Context, userID, customID string) error
        todo!();
    }

    /// Not implemented
    pub fn link_device(&self) {
        // ctx context.Context, userID, deviceID string) error
        todo!();
    }

    /// Not implemented
    pub fn link_email(&self) {
        // ctx context.Context, userID, email, password string) error
        todo!();
    }

    /// Not implemented
    pub fn link_facebook(&self) {
        // ctx context.Context, userID, username, token string, importFriends bool) error
        todo!();
    }

    /// Not implemented
    pub fn link_facebook_instant_game(&self) {
        // ctx context.Context, userID, signedPlayerInfo string) error
        todo!();
    }

    /// Not implemented
    pub fn link_game_center(&self) {
        // ctx context.Context, userID, playerID, bundleID string, timestamp int64, salt, signature, publicKeyUrl string) error
        todo!();
    }

    /// Not implemented
    pub fn link_google(&self) {
        // ctx context.Context, userID, token string) error
        todo!();
    }

    /// Not implemented
    pub fn link_steam(&self) {
        // ctx context.Context, userID, token string) error
        todo!();
    }

    /// Not implemented
    pub fn match_create(&self) {
        // ctx context.Context, module string, params map[string]interface{}) (string, error)
        todo!();
    }

    /// Not implemented
    pub fn match_get(&self) {
        // ctx context.Context, id string) (*api.Match, error)
        todo!();
    }

    /// Not implemented
    pub fn match_list(&self) {
        // ctx context.Context, limit int, authoritative bool, label string, minSize, maxSize *int, query string) ([]*api.Match, error)
        todo!();
    }

    /// Not implemented
    pub fn multi_update(&self) {
        // ctx context.Context, accountUpdates []*AccountUpdate, storageWrites []*StorageWrite, walletUpdates []*WalletUpdate, updateLedger bool) ([]*api.StorageObjectAck, []*WalletUpdateResult, error)
        todo!();
    }

    /// Not implemented
    pub fn notification_send(&self) {
        // ctx context.Context, userID, subject string, content map[string]interface{}, code int, sender string, persistent bool) error
        todo!();
    }

    /// Not implemented
    pub fn notifications_send(&self) {
        // ctx context.Context, notifications []*NotificationSend) error
        todo!();
    }

    /// Not implemented
    pub fn read_file(&self) {
        // path string) (*os.File, error)
        todo!();
    }

    /// Not implemented
    pub fn session_disconnect(&self) {
        // ctx context.Context, sessionID string) error
        todo!();
    }

    /// Not implemented
    pub fn storage_delete(&self) {
        // ctx context.Context, deletes []*StorageDelete) error
        todo!();
    }

    /// Not implemented
    pub fn storage_list(&self) {
        // ctx context.Context, userID, collection string, limit int, cursor string) ([]*api.StorageObject, string, error)
        todo!();
    }

    /// Not implemented
    pub fn storage_read(&self) {
        // ctx context.Context, reads []*StorageRead) ([]*api.StorageObject, error)
        todo!();
    }

    /// Not implemented
    pub fn storage_write(&self) {
        // ctx context.Context, writes []*StorageWrite) ([]*api.StorageObjectAck, error)
        todo!();
    }

    /// Not implemented
    pub fn stream_count(&self) {
        // mode uint8, subject, subcontext, label string) (int, error)
        todo!();
    }

    /// Not implemented
    pub fn stream_close(&self) {
        // mode uint8, subject, subcontext, label string) error
        todo!();
    }

    /// Not implemented
    pub fn stream_send(&self) {
        // mode uint8, subject, subcontext, label, data string, presences []Presence, reliable bool) error
        todo!();
    }

    /// Not implemented
    pub fn stream_send_raw(&self) {
        // mode uint8, subject, subcontext, label string, msg *rtapi.Envelope, presences []Presence, reliable bool) error
        todo!();
    }

    /// Not implemented
    pub fn stream_user_get(&self) {
        // mode uint8, subject, subcontext, label, userID, sessionID string) (PresenceMeta, error)
        todo!();
    }

    /// Not implemented
    pub fn stream_user_join(&self) {
        // mode uint8, subject, subcontext, label, userID, sessionID string, hidden, persistence bool, status string) (bool, error)
        todo!();
    }

    /// Not implemented
    pub fn stream_user_kick(&self) {
        // mode uint8, subject, subcontext, label string, presence Presence) error
        todo!();
    }

    /// Not implemented
    pub fn stream_user_leave(&self) {
        // mode uint8, subject, subcontext, label, userID, sessionID string) error
        todo!();
    }

    /// Not implemented
    pub fn stream_user_list(&self) {
        // mode uint8, subject, subcontext, label string, includeHidden, includeNotHidden bool) ([]Presence, error)
        todo!();
    }

    /// Not implemented
    pub fn stream_user_update(&self) {
        // mode uint8, subject, subcontext, label, userID, sessionID string, hidden, persistence bool, status string) error
        todo!();
    }

    /// Not implemented
    pub fn tournament_add_attempt(&self) {
        // ctx context.Context, id, ownerID string, count int) error
        todo!();
    }

    /// Not implemented
    pub fn tournament_create(&self) {
        // ctx context.Context, id string, sortOrder, operator, resetSchedule string, metadata map[string]interface{}, title, description string, category, startTime, endTime, duration, maxSize, maxNumScore int, joinRequired bool) error
        todo!();
    }

    /// Not implemented
    pub fn tournament_delete(&self) {
        // ctx context.Context, id string) error
        todo!();
    }

    /// Not implemented
    pub fn tournament_join(&self) {
        // ctx context.Context, id, ownerID, username string) error
        todo!();
    }

    /// Not implemented
    pub fn tournament_list(&self) {
        // ctx context.Context, categoryStart, categoryEnd, startTime, endTime, limit int, cursor string) (*api.TournamentList, error)
        todo!();
    }

    /// Not implemented
    pub fn tournament_record_write(&self) {
        // ctx context.Context, id, ownerID, username string, score, subscore int64, metadata map[string]interface{}) (*api.LeaderboardRecord, error)
        todo!();
    }

    /// Not implemented
    pub fn tournament_records_haystack(&self) {
        // ctx context.Context, id, ownerID string, limit int, expiry int64) ([]*api.LeaderboardRecord, error)
        todo!();
    }

    /// Not implemented
    pub fn tournament_records_list(&self) {
        // ctx context.Context, tournamentId string, ownerIDs []string, limit int, cursor string, overrideExpiry int64) ([]*api.LeaderboardRecord, []*api.LeaderboardRecord, string, string, error)
        todo!();
    }

    /// Not implemented
    pub fn tournaments_get_id(&self) {
        // ctx context.Context, tournamentIDs []string) ([]*api.Tournament, error)
        todo!();
    }

    /// Not implemented
    pub fn unlink_apple(&self) {
        // ctx context.Context, userID, token string) error
        todo!();
    }

    /// Not implemented
    pub fn unlink_custom(&self) {
        // ctx context.Context, userID, customID string) error
        todo!();
    }

    /// Not implemented
    pub fn unlink_device(&self) {
        // ctx context.Context, userID, deviceID string) error
        todo!();
    }

    /// Not implemented
    pub fn unlink_email(&self) {
        // ctx context.Context, userID, email string) error
        todo!();
    }

    /// Not implemented
    pub fn unlink_facebook(&self) {
        // ctx context.Context, userID, token string) error
        todo!();
    }

    /// Not implemented
    pub fn unlink_facebook_instant_game(&self) {
        // ctx context.Context, userID, signedPlayerInfo string) error
        todo!();
    }

    /// Not implemented
    pub fn unlink_game_center(&self) {
        // ctx context.Context, userID, playerID, bundleID string, timestamp int64, salt, signature, publicKeyUrl string) error
        todo!();
    }

    /// Not implemented
    pub fn unlink_google(&self) {
        // ctx context.Context, userID, token string) error
        todo!();
    }

    /// Not implemented
    pub fn unlink_steam(&self) {
        // ctx context.Context, userID, token string) error
        todo!();
    }

    /// Not implemented
    pub fn user_groups_list(&self) {
        // ctx context.Context, userID string, limit int, state *int, cursor string) ([]*api.UserGroupList_UserGroup, string, error)
        todo!();
    }

    /// Not implemented
    pub fn users_get_id(&self) {
        // ctx context.Context, userIDs []string) ([]*api.User, error)
        todo!();
    }

    /// Not implemented
    pub fn users_get_username(&self) {
        // ctx context.Context, usernames []string) ([]*api.User, error)
        todo!();
    }

    /// Not implemented
    pub fn users_ban_id(&self) {
        // ctx context.Context, userIDs []string) error
        todo!();
    }

    /// Not implemented
    pub fn users_unban_id(&self) {
        // ctx context.Context, userIDs []string) error
        todo!();
    }

    /// Not implemented
    pub fn wallet_update(&self) {
        // ctx context.Context, userID string, changeset map[string]int64, metadata map[string]interface{}, updateLedger bool) (map[string]int64, map[string]int64, error)
        todo!();
    }

    /// Not implemented
    pub fn wallets_update(&self) {
        // ctx context.Context, updates []*WalletUpdate, updateLedger bool) ([]*WalletUpdateResult, error)
        todo!();
    }

    /// Not implemented
    pub fn wallet_ledger_update(&self) {
        // ctx context.Context, itemID string, metadata map[string]interface{}) (WalletLedgerItem, error)
        todo!();
    }

    /// Not implemented
    pub fn wallet_ledger_list(&self) {
        // ctx context.Context, userID string, limit int, cursor string) ([]WalletLedgerItem, string, error)
        todo!();
    }
}

impl From<&NkModule> for NakamaModule {
    fn from(nk: &NkModule) -> Self {
        Self(nk.clone())
    }
}
