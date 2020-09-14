use {
    serde::Deserialize,
    serde_json,
    serde_repr::Deserialize_repr,
    std::{collections::BTreeMap, io::Read, fs::File, process::Command},
};

fn main() -> Result<(), Failure> {
    let mut file = File::open("token")?;
    let mut token = String::new();
    file.read_to_string(&mut token)?;
    token.insert_str(0, "token=");
    let output = &Command::new("curl").args(&["https://api.todoist.com/sync/v8/sync", "-d", &token, "-d", "sync_token=*", "-d", "resource_types=[\"all\"]"]).output()?.stdout;
    let sync: Sync = serde_json::from_slice(&output)?;
    let user = User::from(sync);

    for task in user.tasks {
        println!("- {:?}", task);
    }

    Ok(())
}

#[derive(Debug)]
enum Failure {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Utf8(std::str::Utf8Error),
}

impl From<std::io::Error> for Failure {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for Failure {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
    }
}

impl From<std::str::Utf8Error> for Failure {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::Utf8(error)
    }
}

struct User {
    tasks: Vec<Task>,
}

impl From<Sync> for User {
    fn from(sync: Sync) -> Self {
        let mut tasks = Vec::new();

        for item in sync.items {
            tasks.push(item.into());
        }

        Self {
            tasks,
        }
    }
}

#[derive(Debug)]
struct Task {
    item: Item,
}

impl From<Item> for Task {
    fn from(item: Item) -> Self {
        Self { item }
    }
}

#[derive(Debug, Deserialize)]
struct Sync {
    /// A new synchronization token.
    sync_token: String,
    /// If this contains all data.
    full_sync: bool,
    /// A [`UserData`].
    user: UserData,
    /// An array of [`Project`]s.
    projects: Vec<Project>,
    /// An array of [`Item`]s.
    items: Vec<Item>,
    /// An array of [`Note`]s.
    notes: Vec<Note>,
    /// An array of [`ProjectNote`]s.
    project_notes: Vec<ProjectNote>,
    /// An array of [`Section`]s.
    sections: Vec<Section>,
    /// An array of [`Label`]s.
    labels: Vec<Label>,
    /// An array of [`Filter`]s.
    filters: Vec<Filter>,
    /// Maps items to their order in the daily agenda.
    day_orders: BTreeMap<ItemId, Order>,
    /// An array of [`Reminder`]s.
    reminders: Vec<Reminder>,
    /// The collaborators for all shared projects.
    collaborators: Vec<Collaborator>,
    /// An array of [`CollaboratorState`]s.
    #[serde(default)]
    collaborators_states: Vec<CollaboratorState>,
    /// An array of [`LiveNotification`]s.
    live_notifications: Vec<LiveNotification>,
    /// The id of the last [`LiveNotification`] seen by the user.
    live_notifications_last_read_id: LiveNotificationId,
    /// The [`UserSettings`].
    user_settings: UserSettings,
}

#[derive(Debug, Deserialize)]
struct Order(i64);

#[derive(Debug, Deserialize)]
/// A Todoist user.
struct UserData {
    /// The default number of minutes for set automatic reminders.
    auto_reminder: u64,
    /// Link to a 195x195 image of the user's avatar.
    avatar_big: String,
    /// Link to a 60x60 image of the user's avatar.
    avatar_medium: String,
    /// Link to a 640x640 image of the user's avatar.
    avatar_s640: String,
    /// Link to a 35x35 image of the user's avatar.
    avatar_small: String,
    /// The user's [`BusinessAccountId`].
    #[serde(default)]
    business_account_id: Option<BusinessAccountId>,
    /// The number of tasks set as the user's daily goal.
    daily_goal: u64,
    /// The user's desired date format.
    date_format: DateFormat,
    /// If smart date recognition has been disabled.
    dateist_inline_disabled: bool,
    /// The language expected for the date recognition.
    dateist_lang: Option<Language>,
    /// The days that the user is off.
    days_off: Vec<Day>,
    /// The default reminder for the user.
    default_reminder: Reminder,
    /// The user's email.
    email: String,
    /// Special internal features that apply to the user.
    features: Features,
    full_name: String,
    id: UserId,
    #[serde(default)]
    image_id: Option<String>,
    inbox_project: ProjectId,
    is_biz_admin: bool,
    is_premium: bool,
    join_date: String,
    karma: f64,
    karma_trend: KarmaTrend,
    lang: Language,
    mobile_host: Option<String>,
    mobile_number: Option<String>,
    next_week: Day,
    premium_until: Option<String>,
    sort_order: SortOrder,
    start_day: Day,
    start_page: Page,
    #[serde(default)]
    team_inbox: Option<ProjectId>,
    theme: Theme,
    time_format: TimeFormat,
    token: String,
    tz_info: TimezoneInfo,
    weekly_goal: u64,
}

#[derive(Debug, Deserialize)]
struct UserId(u64);

#[derive(Debug, Deserialize)]
struct BusinessAccountId(u64);

#[derive(Debug, Deserialize)]
struct Project {
    id: ProjectId,
    name: String,
    color: Color,
    parent_id: Option<ProjectId>,
    child_order: Order,
    collapsed: Flag,
    shared: bool,
    is_deleted: Flag,
    is_archived: Flag,
    is_favorite: Flag,
    sync_id: Option<ProjectSyncId>,
    #[serde(default)]
    inbox_project: bool,
    #[serde(default)]
    team_inbox: bool,
}

#[derive(Debug, Deserialize)]
struct ProjectId(u64);

#[derive(Debug, Deserialize)]
struct ProjectSyncId(u64);

#[derive(Debug, Deserialize)]
struct Item {
    id: ItemId,
    user_id: UserId,
    project_id: ProjectId,
    content: String,
    due: Option<Date>,
    priority: Priority,
    parent_id: Option<ItemId>,
    child_order: Order,
    section_id: Option<SectionId>,
    day_order: Order,
    collapsed: Flag,
    labels: Vec<LabelId>,
    added_by_uid: Option<UserId>,
    assigned_by_uid: Option<UserId>,
    responsible_uid: Option<UserId>,
    checked: Flag,
    in_history: Flag,
    is_deleted: Flag,
    sync_id: Option<ItemSyncId>,
    date_completed: Option<String>,
    date_added: String,
}

#[derive(Debug, Deserialize)]
struct ItemSyncId(u64);

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
struct ItemId(u64);

#[derive(Debug, Deserialize)]
struct Note {
    id: NoteId,
    posted_uid: UserId,
    item_id: ItemId,
    project_id: ProjectId,
    content: String,
    file_attachment: FileAttachment,
    uids_to_notify: Vec<UserId>,
    is_deleted: Flag,
    posted: String,
    reactions: BTreeMap<String, Vec<UserId>>,
}

#[derive(Debug, Deserialize)]
struct NoteId(u64);

#[derive(Debug, Deserialize)]
struct ProjectNote {
    id: ProjectNoteId,
    posted_uid: UserId,
    project_id: ProjectId,
    content: String,
    file_attachment: FileAttachment,
    uids_to_notify: Vec<UserId>,
    is_deleted: Flag,
    posted: String,
    reactions: BTreeMap<String, Vec<UserId>>,
}

#[derive(Debug, Deserialize)]
struct ProjectNoteId(u64);

#[derive(Debug, Deserialize)]
struct Section {
    id: SectionId,
    name: String,
    project_id: ProjectId,
    section_order: Order,
    collapsed: bool,
    sync_id: Option<SectionSyncId>,
    is_deleted: bool,
    is_archived: bool,
    date_archived: Option<String>,
    date_added: String,
}

#[derive(Debug, Deserialize)]
struct SectionId(u64);

#[derive(Debug, Deserialize)]
struct SectionSyncId(u64);

#[derive(Debug, Deserialize)]
struct Label {
    id: LabelId,
    name: String,
    color: Color,
    item_order: Order,
    is_deleted: Flag,
    is_favorite: Flag,
}

#[derive(Debug, Deserialize)]
struct LabelId(u64);

#[derive(Debug, Deserialize)]
struct Filter {
    id: FilterId,
    name: String,
    query: String,
    color: Color,
    item_order: Order,
    is_deleted: Flag,
    is_favorite: Flag,
}

#[derive(Debug, Deserialize)]
struct FilterId(u64);

#[derive(Debug, Deserialize)]
struct Collaborator {
    id: CollaboratorId,
    email: String,
    full_name: String,
    timezone: String,
    #[serde(default)]
    image_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CollaboratorId(u64);

#[derive(Debug, Deserialize)]
struct CollaboratorState {
    project_id: ProjectId,
    user_id: UserId,
    state: CollaboratorStatus,
    is_deleted: bool,
}

#[derive(Debug, Deserialize)]
// Note: v8 api says there should be a `seq_no` field that holds an integer.
struct LiveNotification {
    id: LiveNotificationId,
    // Note: v8 api says that created should be an integer that is the epoch timestamp.
    created: String,
    // Note: v8 api does not say from_uid is optional.
    #[serde(default)]
    from_uid: Option<UserId>,
    notification_key: String,
    notification_type: String,
    is_unread: Flag,
}

#[derive(Debug, Deserialize)]
struct LiveNotificationId(u64);

#[derive(Debug, Deserialize)]
struct UserSettings {
    reminder_push: bool,
    #[serde(default)]
    reminder_sms: bool,
    reminder_desktop: bool,
    reminder_email: bool,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum Color {
    Crimson = 30,
    Red = 31,
    Orange = 32,
    Yellow = 33,
    Olive = 34,
    LightGreen = 35,
    DarkGreen = 36,
    SeaGreen = 37,
    SteelBlue = 38,
    SkyBlue = 39,
    BabyBlue = 40,
    Blue = 41,
    RoyalPurple = 42,
    Violet = 43,
    Pink = 44,
    Mulberry = 45,
    Salmon = 46,
    Gray = 47,
    LightGray = 48,
    Tan = 49,
}

#[derive(Debug, Deserialize)]
enum CollaboratorStatus {
    Active,
    Invited,
}

#[derive(Debug, Deserialize)]
struct FileAttachment {
    file_type: String,
    file_name: String,
    file_size: u64,
    file_url: String,
    upload_state: UploadState,
}

#[derive(Debug, Deserialize)]
enum UploadState {
    Pending,
    Completed,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum Priority {
    Natural = 1,
    High = 2,
    Urgent = 3,
    VeryUrgent = 4,
}

#[derive(Debug, Deserialize)]
struct Date {
    date: String,
    timezone: Option<String>,
    string: String,
    lang: Language,
    is_recurring: bool,
}

#[derive(Debug, Deserialize)]
struct TimezoneInfo {
    gmt_string: String,
    hours: i8,
    is_dst: Flag,
    minutes: u8,
    timezone: String,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum TimeFormat {
    TwentyFour = 0,
    Twelve = 1,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum Theme {
    Theme0 = 0,
    Theme1 = 1,
    Theme2 = 2,
    Theme3 = 3,
    Theme4 = 4,
    Theme5 = 5,
    Theme6 = 6,
    Theme7 = 7,
    Theme8 = 8,
    Theme9 = 9,
    Theme10 = 10,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Page {
    InfoPage,
    Blank,
    Query(String),
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum SortOrder {
    OldestDatesFirst = 0,
    OldestDatesLast = 1,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum KarmaTrend {
    Up,
}

#[derive(Debug, Deserialize)]
struct Features {
    /// If the user has enabled beta.
    beta: Flag,
    /// If inline date parsing is enabled.
    dateist_inline_disabled: bool,
    dateist_lang: Option<Language>,
    #[serde(default)]
    gold_theme: bool,
    has_push_reminders: bool,
    karma_disabled: bool,
    karma_vacation: bool,
    restriction: u64,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum Flag {
    False = 0,
    True = 1,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum Day {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Reminder {
    Email,
    Mobile,
    Push,
    NoDefault,
}

/// The format of a date.
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum DateFormat {
    /// dd-mm-yyyy
    DayMonth = 0,
    /// mm-dd-yyyy
    MonthDay = 1,
}

#[derive(Debug, Deserialize)]
enum Language {
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "ge")]
    German,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt_Br")]
    BrazilianPortuguese,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "sv")]
    Sweedish,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "zh_CN")]
    MainlandChinese,
    #[serde(rename = "zh_TW")]
    TaiwanChinese,
}
