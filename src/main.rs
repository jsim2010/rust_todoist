use std::{collections::BTreeMap, io::Read, fs::File, process::Command};

fn main() -> std::io::Result<()> {
    let mut file = File::open("token")?;
    let mut token = String::new();
    file.read_to_string(&mut token)?;
    token.insert_str(0, "token=");
    let output = Command::new("curl").args(&["https://api.todoist.com/sync/v8/sync", "-d", &token, "-d", "sync_token=*", "-d", "resource_types=[\"all\"]"]).output()?;
    println!("{:?}", output);
    Ok(())
}

struct Sync {
    /// A new synchronization token.
    sync_token: String,
    /// If this contains all data.
    full_sync: bool,
    /// A [`User`].
    user: User,
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
    /// Maps [`Item`] ids to their order in the daily agenda.
    day_orders: BTreeMap<u64, u64>,
    /// An array of [`Reminder`]s.
    reminders: Vec<Reminder>,
    /// The collaborators for all shared projects.
    collaborators: Vec<Collaborator>,
    /// An array of [`CollaboratorState`]s.
    collaborators_states: Vec<CollaboratorState>,
    /// An array of [`LiveNotification`]s.
    live_notifications: Vec<LiveNotification>,
    /// The id of the last [`LiveNotification`] seen by the user.
    live_notifications_last_read_id: u64,
    /// The [`UserSettings`].
    user_settings: UserSettings,
}

/// A Todoist user.
struct User {
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
    business_account_id: u64,
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
    id: u64,
    image_id: String,
    inbox_project: u64,
    is_biz_admin: bool,
    is_premium: bool,
    join_date: String,
    karma: u64,
    karma_trend: KarmaTrend,
    lang: Language,
    legacy_inbox_project: u64,
    legacy_team_inbox: u64,
    mobile_host: Option<String>,
    mobile_number: Option<String>,
    next_week: Day,
    premium_until: Option<String>,
    sort_order: SortOrder,
    start_day: Day,
    start_page: Page,
    team_inbox: u64,
    theme: Theme,
    time_format: TimeFormat,
    token: String,
    tz_info: TimezoneInfo,
    weekly_goal: u64,
}

struct Project {
    id: u64,
    legacy_id: u64,
    name: String,
    color: u64,
    parent_id: Option<u64>,
    legacy_parent_id: Option<u64>,
    child_order: u64,
    collapsed: Flag,
    shared: bool,
    is_deleted: Flag,
    is_archived: Flag,
    is_favorite: Flag,
    sync_id: Option<u64>,
    inbox_project: bool,
    team_inbox: bool,
}

struct Item {
    id: u64,
    legacy_id: u64,
    user_id: u64,
    project_id: u64,
    legacy_project_id: u64,
    content: String,
    due: Date,
    priority: Priority,
    parent_id: Option<u64>,
    legacy_parent_id: Option<u64>,
    child_order: u64,
    section_id: Option<u64>,
    day_order: u64,
    collapsed: Flag,
    labels: Vec<u64>,
    added_by_uid: u64,
    assigned_by_uid: u64,
    responsible_uid: u64,
    checked: Flag,
    in_history: Flag,
    is_deleted: Flag,
    sync_id: Option<u64>,
    date_completed: Option<String>,
    date_added: String,
}

struct Note {
    id: u64,
    legacy_id: u64,
    posted_uid: u64,
    item_id: u64,
    legacy_item_id: u64,
    project_id: u64,
    legacy_project_id: u64,
    content: String,
    file_attachment: FileAttachment,
    uids_to_notify: Vec<u64>,
    is_deleted: Flag,
    posted: String,
    reactions: BTreeMap<String, Vec<u64>>,
}

struct ProjectNote {
    id: u64,
    posted_uid: u64,
    project_id: u64,
    content: String,
    file_attachment: FileAttachment,
    uids_to_notify: Vec<u64>,
    is_deleted: Flag,
    posted: String,
    reactions: BTreeMap<String, Vec<u64>>,
}

struct Section {
    id: u64,
    name: String,
    project_id: u64,
    legacy_project_id: u64,
    section_order: u64,
    collapsed: bool,
    sync_id: Option<u64>,
    is_deleted: bool,
    is_archived: bool,
    date_archived: Option<String>,
    date_added: String,
}

struct Label {
    id: u64,
    name: String,
    color: Color,
    item_order: u64,
    is_deleted: Flag,
    is_favorite: Flag,
}

struct Filter {
    id: u64,
    name: String,
    query: String,
    color: Color,
    item_order: u64,
    is_deleted: Flag,
    is_favorite: Flag,
}

struct Collaborator {
    id: u64,
    email: String,
    full_name: String,
    timezone: String,
    image_id: u64,
}

struct CollaboratorState {
    project_id: u64,
    user_id: u64,
    state: CollaboratorStatus,
    is_deleted: bool,
}

struct LiveNotification {
    id: u64,
    legacy_id: u64,
    created: u64,
    from_uid: u64,
    notification_key: String,
    notification_type: String,
    seq_no: u64,
    is_unread: Flag,
}

struct UserSettings {
    reminder_push: bool,
    reminder_sms: bool,
    reminder_desktop: bool,
    reminder_email: bool,
}

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

enum CollaboratorStatus {
    Active,
    Invited,
}

struct FileAttachment {
    file_type: String,
    file_name: String,
    file_size: u64,
    file_url: String,
    upload_state: UploadState,
}

enum UploadState {
    Pending,
    Completed,
}

enum Priority {
    Natural = 1,
    High = 2,
    Urgent = 3,
    VeryUrgent = 4,
}

struct Date {
    date: String,
    timezone: Option<String>,
    string: String,
    lang: Language,
    is_recurring: bool,
}

struct TimezoneInfo {
    gmt_string: String,
    hours: u64,
    is_dst: u64,
    minutes: u64,
    timezone: String,
}

enum TimeFormat {
    TwentyFour = 0,
    Twelve = 1,
}

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

enum Page {
    InfoPage,
    Blank,
    Project(String),
    Query(String),
}

enum SortOrder {
    OldestDatesFirst = 0,
    OldestDatesLast = 1,
}

enum KarmaTrend {
    Up,
}

struct Features {
    /// If the user has enabled beta.
    beta: Flag,
    /// If inline date parsing is enabled.
    datist_inline_disabled: bool,
    datist_lang: Option<Language>,
    gold_theme: bool,
    has_push_reminders: bool,
    karma_disabled: bool,
    karma_vacation: bool,
    restriction: u64,
}

enum Flag {
    False = 0,
    True = 1,
}

enum Day {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

enum Reminder {
    Email,
    Mobile,
    Push,
    NoDefault,
}

/// The format of a date.
enum DateFormat {
    /// dd-mm-yyyy
    DayMonth = 0,
    /// mm-dd-yyyy
    MonthDay = 1,
}

enum Language {
    /// da
    Danish,
    /// ge
    German,
    /// en
    English,
    /// es
    Spanish,
    /// fi
    Finnish,
    /// fr
    French,
    /// it
    Italian,
    /// ja
    Japanese,
    /// ko
    Korean,
    /// nl
    Dutch,
    /// pl
    Polish,
    /// pt_BR
    BrazilianPortuguese,
    /// ru
    Russian,
    /// sv
    Sweedish,
    /// tr
    Turkish,
    /// zh_CN
    MainlandChinese,
    /// zh_TW
    TaiwanChinese,
}
