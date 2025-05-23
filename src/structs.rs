

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Repo {
    /*id: u8,
    owner: User,
    name: String,*/
    pub full_name: String,
    /*description: String,
    empty: bool,
    private: bool,
    fork: bool,
    template: bool,
    parent: Option<String>,
    mirror: bool,
    size: usize,
    language: String,
    languages_url: String,
    html_url: String,
    url: String,
    link: String,
    ssh_url: String,
    clone_url: String,
    original_url: String,
    website: String,
    stars_count: u8,
    forks_count: u8,
    watchers_count: u8,
    open_issues_count: u8,
    open_pr_counter: u8,
    release_counter: u8,
    default_branch: String,
    archived: bool,
    created_at: String,
    updated_at: String,
    archived_at: String,
    permissions: Permissions,
    has_issues: bool,
    internal_tracker: Tracker,
    has_wiki: bool,
    wiki_branch: String,
    globally_editable_wiki: bool,
    has_pull_requests: bool,
    has_projects: bool,
    has_releases: bool,
    has_packages: bool,
    has_actions: bool,
    ignore_whitespace_conflicts: bool,
    allow_merge_commits: bool,
    allow_rebase: bool,
    allow_rebase_explicit: bool,
    allow_squash_merge: bool,
    allow_fast_forward_only_merge: bool,
    allow_rebase_update: bool,
    default_delete_branch_after_merge: bool,
    default_merge_style: String,
    default_allow_maintainer_edit: bool,
    default_update_style: String,
    avatar_url: String,
    internal: bool,
    mirror_interval: String,
    object_format_name: String,
    mirror_updated: String,
    repo_transfer: Option<String>,
    topics: Option<Vec<String>>,*/
}
/*
#[derive(Deserialize, Debug)]
struct User {
    id: u8,
    login: String,
    login_name: String,
    source_id: u8,
    full_name: String,
    email: String,
    avatar_url: String,
    html_url: String,
    language: String,
    is_admin: bool,
    last_login: String,
    created: String,
    restricted: bool,
    active: bool,
    prohibit_login: bool,
    location: String,
    pronouns: String,
    website: String,
    description: String,
    visibility: String,
    followers_count: u8,
    following_count: u8,
    starred_repos_count: u8,
    username: String,
}

#[derive(Deserialize, Debug)]
struct Permissions {
    admin: bool,
    push: bool,
    pull: bool,
}

#[derive(Deserialize, Debug)]
struct Tracker {
    enable_time_tracker: bool,
    allow_only_contributors_to_track_time: bool,
    enable_issue_dependencies: bool,
}
*/