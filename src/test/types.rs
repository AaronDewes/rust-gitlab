// Copyright 2016 Kitware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crates::chrono::{NaiveDate, TimeZone, Utc};
use crates::serde::de::DeserializeOwned;
use crates::serde_json::from_reader;

use types::*;

use std::fs::File;

fn read_test_file<T: DeserializeOwned>(name: &str) -> T {
    let fin = File::open(format!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/{}.json"), name))
        .unwrap();

    from_reader::<File, T>(fin).unwrap()
}

#[test]
fn test_read_award_emoji() {
    let award_emoji: AwardEmoji = read_test_file("award_emoji");

    assert_eq!(award_emoji.id, AwardId::new(335));
    assert_eq!(award_emoji.name, "8ball");
    assert_eq!(award_emoji.user.username, "ben.boeckel");
    assert_eq!(award_emoji.user.web_url,
               "https://gitlab.kitware.com/ben.boeckel");
    assert_eq!(award_emoji.user.name, "Ben Boeckel");
    assert_eq!(award_emoji.user.state, UserState::Active);
    assert_eq!(award_emoji.user.avatar_url,
               "https://secure.gravatar.com/avatar/2f5f7e99190174edb5a2f66b8653b0b2?s=80&d=identicon");
    assert_eq!(award_emoji.user.id, UserId::new(13));
    assert_eq!(award_emoji.created_at,
               Utc.ymd(2016, 12, 7)
                   .and_hms_milli(16, 23, 46, 742));
    assert_eq!(award_emoji.updated_at,
               Utc.ymd(2016, 12, 7)
                   .and_hms_milli(16, 23, 46, 742));
    assert_eq!(award_emoji.awardable_id(),
               AwardableId::Note(NoteId::new(177359)));
    assert_eq!(award_emoji.awardable_type, AwardableType::Note);
}

#[test]
fn test_read_commit_note() {
    let commit_note: CommitNote = read_test_file("commit_note");

    assert_eq!(commit_note.note, "Example commit note for data fetching.");
    assert_eq!(commit_note.path, None);
    assert_eq!(commit_note.line, None);
    assert_eq!(commit_note.line_type, None);
    assert_eq!(commit_note.author.username, "ben.boeckel");
    assert_eq!(commit_note.author.web_url,
               "https://gitlab.kitware.com/ben.boeckel");
    assert_eq!(commit_note.author.name, "Ben Boeckel");
    assert_eq!(commit_note.author.state, UserState::Active);
    assert_eq!(commit_note.author.avatar_url,
               "https://secure.gravatar.com/avatar/2f5f7e99190174edb5a2f66b8653b0b2?s=80&d=identicon");
    assert_eq!(commit_note.author.id, UserId::new(13));
    assert_eq!(commit_note.created_at,
               Utc.ymd(2016, 12, 7)
                   .and_hms_milli(16, 28, 33, 966));
}

#[test]
fn test_read_commit_status() {
    let commit_status: CommitStatus = read_test_file("commit_status");

    assert_eq!(commit_status.id, CommitStatusId::new(931434));
    assert_eq!(commit_status.sha,
               ObjectId::new("de4ac3cf96cb8a0893be22b03f5171d934f9d392".to_string()));
    assert_eq!(commit_status.ref_, Some("master".to_string()));
    assert_eq!(commit_status.status, StatusState::Success);
    assert_eq!(commit_status.name, "rust-gitlab-megas-linux-debug");
    assert_eq!(commit_status.target_url,
               Some("https://buildbot.kitware.com/builders/rust-gitlab-megas-linux-debug/builds/41"
                   .to_string()));
    assert_eq!(commit_status.description, Some("expected".to_string()));
    assert_eq!(commit_status.created_at,
               Utc.ymd(2016, 11, 8)
                   .and_hms_milli(14, 35, 32, 627));
    assert_eq!(commit_status.started_at, None);
    assert_eq!(commit_status.finished_at,
               Some(Utc.ymd(2016, 11, 8)
                   .and_hms_milli(14, 35, 32, 629)));
    assert_eq!(commit_status.allow_failure, false);
    assert_eq!(commit_status.author.username, "buildbot");
    assert_eq!(commit_status.author.web_url,
               "https://gitlab.kitware.com/buildbot");
    assert_eq!(commit_status.author.name, "buildbot");
    assert_eq!(commit_status.author.state, UserState::Active);
    assert_eq!(commit_status.author.avatar_url,
               "https://gitlab.kitware.com/uploads/-/system/user/avatar/35/buildbot-logo.png");
    assert_eq!(commit_status.author.id, UserId::new(35));
    assert_eq!(commit_status.coverage, None);
}

#[test]
fn test_read_issue() {
    let issue: Issue = read_test_file("issue");

    assert_eq!(issue.id, IssueId::new(69328));
    assert_eq!(issue.iid, IssueInternalId::new(6));
    assert_eq!(issue.project_id, ProjectId::new(855));
    assert_eq!(issue.title, "fix documentation warnings");
    assert_eq!(issue.description, Some("".to_string()));
    assert_eq!(issue.state, IssueState::Closed);
    assert_eq!(issue.created_at,
               Utc.ymd(2016, 10, 30)
                   .and_hms_milli(18, 54, 28, 954));
    assert_eq!(issue.updated_at,
               Utc.ymd(2017, 7, 7)
                   .and_hms_milli(6, 31, 03, 757));
    assert_eq!(issue.closed_at, None);
    assert!(issue.labels.is_empty());
    assert!(issue.milestone.is_none());
    assert_eq!(issue.author.username, "ben.boeckel");
    assert_eq!(issue.author.web_url,
               "https://gitlab.kitware.com/ben.boeckel");
    assert_eq!(issue.author.name, "Ben Boeckel");
    assert_eq!(issue.author.state, UserState::Active);
    assert_eq!(issue.author.avatar_url,
               "https://secure.gravatar.com/avatar/2f5f7e99190174edb5a2f66b8653b0b2?s=80&d=identicon");
    assert_eq!(issue.author.id, UserId::new(13));
    if let Some(ref assignee) = issue.assignee {
        assert_eq!(assignee.username, "ben.boeckel");
        assert_eq!(assignee.web_url, "https://gitlab.kitware.com/ben.boeckel");
        assert_eq!(assignee.name, "Ben Boeckel");
        assert_eq!(assignee.state, UserState::Active);
        assert_eq!(assignee.avatar_url,
                   "https://secure.gravatar.com/avatar/2f5f7e99190174edb5a2f66b8653b0b2?s=80&d=identicon");
        assert_eq!(assignee.id, UserId::new(13));
    } else {
        panic!("expected to have an assignee for the issue");
    }
    if let Some(ref assignees) = issue.assignees {
        assert_eq!(assignees.len(), 1);
        let assignee = &assignees[0];
        assert_eq!(assignee.username, "ben.boeckel");
        assert_eq!(assignee.web_url, "https://gitlab.kitware.com/ben.boeckel");
        assert_eq!(assignee.name, "Ben Boeckel");
        assert_eq!(assignee.state, UserState::Active);
        assert_eq!(assignee.avatar_url,
                   "https://secure.gravatar.com/avatar/2f5f7e99190174edb5a2f66b8653b0b2?s=80&d=identicon");
        assert_eq!(assignee.id, UserId::new(13));
    } else {
        panic!("expected to have assignees for the issue");
    }
    assert_eq!(issue.subscribed, Some(true));
    assert_eq!(issue.user_notes_count, 0);
    assert_eq!(issue.upvotes, 0);
    assert_eq!(issue.downvotes, 0);
    assert_eq!(issue.due_date, None);
    assert_eq!(issue.confidential, false);
    assert_eq!(issue.discussion_locked, None);
    assert_eq!(issue.web_url,
               "https://gitlab.kitware.com/utils/rust-gitlab/issues/6");
    assert!(issue.has_links());
}

#[test]
fn test_read_issue_reference() {
    let issue_reference: IssueReference = read_test_file("issue_reference");

    if let IssueReference::Internal(issue) = issue_reference {
        assert_eq!(issue.id, IssueId::new(69075));
        assert_eq!(issue.iid, IssueInternalId::new(5));
        assert_eq!(issue.project_id, ProjectId::new(855));
        assert_eq!(issue.title, "Add project hook APIs");
        assert_eq!(issue.description,
                   Some("The workflow currently requires that the robot be able to register itself as \
                         a webhook for new projects. An API needs added for this.\n\nCc: @brad.king".to_string()));
        assert_eq!(issue.state, IssueState::Closed);
        assert_eq!(issue.created_at,
                   Utc.ymd(2016, 10, 4)
                       .and_hms_milli(18, 59, 37, 178));
        assert_eq!(issue.updated_at,
                   Utc.ymd(2017, 7, 7)
                       .and_hms_milli(6, 31, 5, 370));
        assert_eq!(issue.closed_at, None);
        assert!(issue.labels.is_empty());
        assert!(issue.milestone.is_none());
        assert_eq!(issue.author.username, "ben.boeckel");
        assert_eq!(issue.author.web_url,
                   "https://gitlab.kitware.com/ben.boeckel");
        assert_eq!(issue.author.name, "Ben Boeckel");
        assert_eq!(issue.author.state, UserState::Active);
        assert_eq!(issue.author.avatar_url,
                   "https://secure.gravatar.com/avatar/2f5f7e99190174edb5a2f66b8653b0b2?s=80&d=identicon");
        assert_eq!(issue.author.id, UserId::new(13));
        if let Some(ref assignee) = issue.assignee {
            assert_eq!(assignee.username, "ben.boeckel");
            assert_eq!(assignee.web_url, "https://gitlab.kitware.com/ben.boeckel");
            assert_eq!(assignee.name, "Ben Boeckel");
            assert_eq!(assignee.state, UserState::Active);
            assert_eq!(assignee.avatar_url,
                       "https://secure.gravatar.com/avatar/2f5f7e99190174edb5a2f66b8653b0b2?s=80&d=identicon");
            assert_eq!(assignee.id, UserId::new(13));
        } else {
            panic!("expected to have an assignee for the issue");
        }
        assert_eq!(issue.subscribed, None);
        assert_eq!(issue.time_stats.time_estimate, 0);
        assert_eq!(issue.time_stats.total_time_spent, 0);
        assert_eq!(issue.time_stats.human_time_estimate, None);
        assert_eq!(issue.time_stats.human_total_time_spent, None);
        assert_eq!(issue.user_notes_count, 0);
        assert_eq!(issue.upvotes, 0);
        assert_eq!(issue.downvotes, 0);
        assert_eq!(issue.due_date, None);
        assert_eq!(issue.confidential, false);
        assert_eq!(issue.discussion_locked, None);
        assert_eq!(issue.web_url,
                   "https://gitlab.kitware.com/utils/rust-gitlab/issues/5");
    } else {
        panic!("expected to have an internal issue reference");
    }
}

#[test]
fn test_read_member() {
    let member: Member = read_test_file("member");

    assert_eq!(member.username, "kwrobot");
    assert_eq!(member.name, "Kitware Robot");
    assert_eq!(member.id, UserId::new(11));
    assert_eq!(member.state, UserState::Active);
    assert_eq!(member.avatar_url,
               "https://secure.gravatar.com/avatar/9ddcd45fcb89d966aab95b1f1002f84c?s=80&d=identicon");
    assert_eq!(member.web_url, "https://gitlab.kitware.com/kwrobot");
    assert_eq!(member.access_level, 50);
    assert_eq!(member.expires_at, None);
}

#[test]
fn test_read_merge_request() {
    let merge_request: MergeRequest = read_test_file("merge_request");

    assert_eq!(merge_request.id, MergeRequestId::new(20215));
    assert_eq!(merge_request.iid, MergeRequestInternalId::new(35));
    assert_eq!(merge_request.project_id, ProjectId::new(855));
    assert_eq!(merge_request.title, "gitlab: expose hook addition API");
    assert_eq!(merge_request.description, Some("Fixes #5.".to_string()));
    assert_eq!(merge_request.state, MergeRequestState::Merged);
    assert_eq!(merge_request.created_at,
               Utc.ymd(2016, 10, 4)
                   .and_hms_milli(19, 56, 43, 276));
    assert_eq!(merge_request.updated_at,
               Utc.ymd(2016, 10, 4)
                   .and_hms_milli(20, 18, 57, 940));
    assert_eq!(merge_request.target_branch, "master");
    assert_eq!(merge_request.source_branch, "add_hook-api");
    assert_eq!(merge_request.upvotes, 0);
    assert_eq!(merge_request.downvotes, 0);
    assert_eq!(merge_request.author.username, "ben.boeckel");
    assert_eq!(merge_request.author.web_url,
               "https://gitlab.kitware.com/ben.boeckel");
    assert_eq!(merge_request.author.name, "Ben Boeckel");
    assert_eq!(merge_request.author.state, UserState::Active);
    assert_eq!(merge_request.author.avatar_url,
               "https://secure.gravatar.com/avatar/2f5f7e99190174edb5a2f66b8653b0b2?s=80&d=identicon");
    assert_eq!(merge_request.author.id, UserId::new(13));
    if let Some(ref assignee) = merge_request.assignee {
        assert_eq!(assignee.username, "brad.king");
        assert_eq!(assignee.web_url, "https://gitlab.kitware.com/brad.king");
        assert_eq!(assignee.name, "Brad King");
        assert_eq!(assignee.state, UserState::Active);
        assert_eq!(assignee.avatar_url,
                   "https://secure.gravatar.com/avatar/0617392a2f9fd505720d0c42cefc1a10?s=80&d=identicon");
        assert_eq!(assignee.id, UserId::new(10));
    } else {
        panic!("expected to have an assignee for the merge request");
    }
    assert_eq!(merge_request.source_project_id, ProjectId::new(856));
    assert_eq!(merge_request.target_project_id, ProjectId::new(855));
    assert!(merge_request.labels.is_empty());
    assert_eq!(merge_request.work_in_progress, false);
    assert!(merge_request.milestone.is_none());
    assert_eq!(merge_request.merge_when_pipeline_succeeds, false);
    assert_eq!(merge_request.merge_status, MergeStatus::CanBeMerged);
    assert_eq!(merge_request.sha,
               Some(ObjectId::new("04e94ae667024a62a90179f395bfdc2b35f3efd2")));
    assert_eq!(merge_request.merge_commit_sha, None);
    assert_eq!(merge_request.subscribed, Some(true));
    assert_eq!(merge_request.time_stats.time_estimate, 0);
    assert_eq!(merge_request.time_stats.total_time_spent, 0);
    assert_eq!(merge_request.time_stats.human_time_estimate, None);
    assert_eq!(merge_request.time_stats.human_total_time_spent, None);
    assert_eq!(merge_request.changes_count, Some("3".to_string()));
    assert_eq!(merge_request.user_notes_count, 3);
    assert_eq!(merge_request.discussion_locked, None);
    assert_eq!(merge_request.should_remove_source_branch, None);
    assert_eq!(merge_request.force_remove_source_branch, Some(true));
    assert_eq!(merge_request.web_url,
               "https://gitlab.kitware.com/utils/rust-gitlab/merge_requests/35");
}

#[test]
fn test_read_note() {
    let note: Note = read_test_file("note");

    assert_eq!(note.id, NoteId::new(177371));
    assert_eq!(note.body,
               "Mentioned in commit 47d475d8625424bd37efd27f7097354306842b93");
    assert_eq!(note.attachment, None);
    assert_eq!(note.author.username, "brad.king");
    assert_eq!(note.author.web_url, "https://gitlab.kitware.com/brad.king");
    assert_eq!(note.author.name, "Brad King");
    assert_eq!(note.author.state, UserState::Active);
    assert_eq!(note.author.avatar_url,
               "https://secure.gravatar.com/avatar/0617392a2f9fd505720d0c42cefc1a10?s=80&d=identicon");
    assert_eq!(note.author.id, UserId::new(10));
    assert_eq!(note.created_at,
               Utc.ymd(2016, 10, 4)
                   .and_hms_milli(20, 18, 57, 786));
    assert_eq!(note.updated_at,
               Utc.ymd(2016, 10, 4)
                   .and_hms_milli(20, 18, 57, 786));
    assert_eq!(note.system, true);
    assert_eq!(note.noteable_id(),
               Some(NoteableId::MergeRequest(MergeRequestId::new(20215))));
    assert_eq!(note.noteable_iid(),
               Some(NoteableInternalId::MergeRequest(MergeRequestInternalId::new(35))));
    assert_eq!(note.noteable_type, NoteType::MergeRequest);
}

#[test]
fn test_read_project() {
    let project: Project = read_test_file("project");

    assert_eq!(project.id, ProjectId::new(855));
    assert_eq!(project.description,
               Some("Rust library for communicating with a Gitlab instance.".to_string()));
    assert_eq!(project.default_branch, Some("master".to_string()));
    assert!(project.tag_list.is_empty());
    assert_eq!(project.archived, false);
    assert_eq!(project.visibility, VisibilityLevel::Public);
    assert_eq!(project.ssh_url_to_repo,
               "git@gitlab.kitware.com:utils/rust-gitlab.git");
    assert_eq!(project.http_url_to_repo,
               "https://gitlab.kitware.com/utils/rust-gitlab.git");
    assert_eq!(project.web_url,
               "https://gitlab.kitware.com/utils/rust-gitlab");
    assert!(project.owner.is_none());
    assert_eq!(project.name, "rust-gitlab");
    assert_eq!(project.name_with_namespace, "Utils / rust-gitlab");
    assert_eq!(project.path, "rust-gitlab");
    assert_eq!(project.path_with_namespace, "utils/rust-gitlab");
    assert_eq!(project.container_registry_enabled, Some(true));
    assert_eq!(project.created_at,
               Utc.ymd(2016, 6, 29)
                   .and_hms_milli(17, 35, 12, 495));
    assert_eq!(project.last_activity_at,
               Utc.ymd(2017, 10, 5)
                   .and_hms_milli(15, 18, 57, 875));
    assert_eq!(project.import_error, None);
    assert_eq!(project.shared_runners_enabled, true);
    assert_eq!(project.lfs_enabled, true);
    assert_eq!(project.creator_id, UserId::new(13));
    assert_eq!(project.namespace.name, "Utils");
    assert_eq!(project.namespace.path, "utils");
    assert_eq!(project.namespace.id(),
               NamespaceId::Group(GroupId::new(498)));
    assert_eq!(project.namespace.kind, NamespaceKind::Group);
    assert_eq!(project.namespace.full_path, "utils");
    assert_eq!(project.namespace.members_count_with_descendants, Some(3));
    assert!(project.forked_from_project.is_none());
    assert_eq!(project.avatar_url, None);
    assert_eq!(project.ci_config_path, None);
    assert_eq!(project.star_count, 0);
    assert_eq!(project.forks_count, 5);
    assert_eq!(project.open_issues_count, Some(4));
    assert_eq!(project.public_jobs, true);
    assert!(project.shared_with_groups.is_empty());
    assert_eq!(project.only_allow_merge_if_pipeline_succeeds, Some(false));
    assert_eq!(project.only_allow_merge_if_all_discussions_are_resolved,
               None);
    assert_eq!(project.printing_merge_request_link_enabled, Some(true));
    assert_eq!(project.request_access_enabled, true);
    assert_eq!(project.jobs_enabled, false);
    assert_eq!(project.resolve_outdated_diff_discussions, None);
    assert_eq!(project.issues_enabled, true);
    assert_eq!(project.merge_requests_enabled, true);
    assert_eq!(project.snippets_enabled, false);
    assert_eq!(project.wiki_enabled, true);
    if let Some(ref permissions) = project.permissions {
        if let Some(ref group_access) = permissions.group_access {
            assert_eq!(group_access.access_level, 50);
            assert_eq!(group_access.notification_level, Some(3));
        } else {
            panic!("expected to have group access on the permissions");
        }
        assert!(permissions.project_access.is_none());
    assert!(project.has_links());
    } else {
        panic!("expected to have permissions available");
    }
}

#[test]
fn test_read_project_hook() {
    let project_hook: ProjectHook = read_test_file("project_hook");

    assert_eq!(project_hook.id, HookId::new(1262));
    assert_eq!(project_hook.url, "http://kwrobot02:8082/gitlab.kitware.com");
    assert_eq!(project_hook.created_at,
               Utc.ymd(2016, 12, 16)
                   .and_hms_milli(16, 37, 24, 589));
    assert_eq!(project_hook.push_events, true);
    assert_eq!(project_hook.tag_push_events, true);
    assert_eq!(project_hook.issues_events, true);
    assert_eq!(project_hook.merge_requests_events, true);
    assert_eq!(project_hook.note_events, true);
    assert_eq!(project_hook.repository_update_events, false);
    assert_eq!(project_hook.enable_ssl_verification, true);
    assert_eq!(project_hook.job_events, true);
    assert_eq!(project_hook.pipeline_events, true);
    assert_eq!(project_hook.wiki_page_events, true);
}

#[test]
fn test_read_repo_branch() {
    let repo_branch: RepoBranch = read_test_file("repo_branch");

    assert_eq!(repo_branch.name, "master");
    if let Some(ref commit) = repo_branch.commit {
        assert_eq!(commit.author_email, "ben.boeckel@kitware.com");
        assert_eq!(commit.author_name, "Ben Boeckel");
        assert_eq!(commit.authored_date,
                   Utc.ymd(2017, 7, 10)
                       .and_hms_milli(18, 39, 4, 0));
        assert_eq!(commit.committed_date,
                   Utc.ymd(2017, 7, 10)
                       .and_hms_milli(18, 39, 6, 0));
        assert_eq!(commit.created_at,
                   Utc.ymd(2017, 7, 10)
                       .and_hms_milli(18, 39, 6, 0));
        assert_eq!(commit.committer_email, "kwrobot@kitware.com");
        assert_eq!(commit.committer_name, "Kitware Robot");
        assert_eq!(commit.id,
                   ObjectId::new("a84836ab97dc687ca5a241205aff6a713255f767"));
        assert_eq!(commit.short_id, ObjectId::new("a84836ab"));
        assert_eq!(commit.title, "Merge topic 'gitlab-9.3'");
        assert_eq!(commit.message,
                   "Merge topic 'gitlab-9.3'\n\
                    \n\
                    e5a23653 systemhooks: update for 9.3 changes\n\
                    \n\
                    Acked-by: Kitware Robot <kwrobot@kitware.com>\n\
                    Reviewed-by: Ben Boeckel <ben.boeckel@kitware.com>\n\
                    Merge-request: !100\n\
                    ");
        assert_eq!(commit.parent_ids,
                   vec![ObjectId::new("45aadb02640f6672fb4a382fc5b1ab5efec4f1df"),
                        ObjectId::new("e5a23653f3d6532fefad0dffd90fb101ef94d42f")]);
    } else {
        panic!("expected to have a commit for the branch");
    }
    assert_eq!(repo_branch.merged, Some(false));
    assert_eq!(repo_branch.protected, Some(true));
    assert_eq!(repo_branch.developers_can_push, Some(false));
    assert_eq!(repo_branch.developers_can_merge, Some(false));
}

#[test]
fn test_read_repo_commit_detail() {
    let repo_commit_detail: RepoCommitDetail = read_test_file("repo_commit_detail");

    assert_eq!(repo_commit_detail.id,
               ObjectId::new("de4ac3cf96cb8a0893be22b03f5171d934f9d392"));
    assert_eq!(repo_commit_detail.short_id, ObjectId::new("de4ac3cf"));
    assert_eq!(repo_commit_detail.title, "Merge topic 'mr-awards'");
    assert_eq!(repo_commit_detail.author_name, "Brad King");
    assert_eq!(repo_commit_detail.author_email, "brad.king@kitware.com");
    assert_eq!(repo_commit_detail.committer_name, "Kitware Robot");
    assert_eq!(repo_commit_detail.committer_email, "kwrobot@kitware.com");
    assert_eq!(repo_commit_detail.created_at,
               Utc.ymd(2016, 11, 8)
                   .and_hms_milli(14, 30, 13, 0));
    assert_eq!(repo_commit_detail.message,
               "Merge topic 'mr-awards'\n\na222c553 gitlab: add a method for MR award \
                queries\n\nAcked-by: Kitware Robot <kwrobot@kitware.com>\nReviewed-by: Brad King \
                <brad.king@kitware.com>\nMerge-request: !46\n");
    assert_eq!(repo_commit_detail.parent_ids,
               vec![ObjectId::new("559f5f4a2bfe1f48e9e95afa09c029deb655cf7d"),
                    ObjectId::new("a222c5539569cda6999b8069f1e51a5202c30711")]);
    assert_eq!(repo_commit_detail.committed_date,
               Utc.ymd(2016, 11, 8)
                   .and_hms_milli(14, 30, 13, 0));
    assert_eq!(repo_commit_detail.authored_date,
               Utc.ymd(2016, 11, 8)
                   .and_hms_milli(14, 30, 13, 0));
    if let Some(ref stats) = repo_commit_detail.stats {
        assert_eq!(stats.additions, 8);
        assert_eq!(stats.deletions, 0);
        assert_eq!(stats.total, 8);
    } else {
        panic!("expected to have stats for this commit");
    }
    if let Some(ref last_pipeline) = repo_commit_detail.last_pipeline {
        assert_eq!(last_pipeline.id, PipelineId::new(34289));
        assert_eq!(last_pipeline.ref_, Some("master".to_string()));
        assert_eq!(last_pipeline.sha, ObjectId::new("de4ac3cf96cb8a0893be22b03f5171d934f9d392"));
        assert_eq!(last_pipeline.status, StatusState::Success);
    } else {
        panic!("expected to have a last_pipeline for this commit");
    }
}

#[test]
fn test_read_user() {
    let user: User = read_test_file("user");

    assert_eq!(user.username, "kwrobot");
    assert_eq!(user.name, "Kitware Robot");
    assert_eq!(user.id, UserId::new(11));
    assert_eq!(user.state, UserState::Active);
    assert_eq!(user.avatar_url,
               "https://secure.gravatar.com/avatar/9ddcd45fcb89d966aab95b1f1002f84c?s=80&d=identicon");
    assert_eq!(user.web_url, "https://gitlab.kitware.com/kwrobot");
    assert_eq!(user.created_at,
               Utc.ymd(2015, 2, 26)
                   .and_hms_milli(15, 58, 34, 670));
    assert_eq!(user.is_admin, None);
    assert_eq!(user.bio, Some("".to_string()));
    assert_eq!(user.location, None);
    assert_eq!(user.skype, "");
    assert_eq!(user.linkedin, "");
    assert_eq!(user.twitter, "");
    assert_eq!(user.website_url, "");
    assert_eq!(user.organization, None);
}

#[test]
fn test_read_user_public() {
    let user_public: UserPublic = read_test_file("user_public");

    assert_eq!(user_public.username, "kwrobot");
    assert_eq!(user_public.name, "Kitware Robot");
    assert_eq!(user_public.id, UserId::new(11));
    assert_eq!(user_public.state, UserState::Active);
    assert_eq!(user_public.avatar_url,
               "https://secure.gravatar.com/avatar/9ddcd45fcb89d966aab95b1f1002f84c?s=80&d=identicon");
    assert_eq!(user_public.web_url,
               "https://gitlab.kitware.com/kwrobot");
    assert_eq!(user_public.created_at,
               Utc.ymd(2015, 2, 26)
                   .and_hms_milli(15, 58, 34, 670));
    assert_eq!(user_public.is_admin, Some(true));
    assert_eq!(user_public.bio, Some("".to_string()));
    assert_eq!(user_public.location, None);
    assert_eq!(user_public.skype, "");
    assert_eq!(user_public.linkedin, "");
    assert_eq!(user_public.twitter, "");
    assert_eq!(user_public.website_url, "");
    assert_eq!(user_public.organization, None);
    assert_eq!(user_public.last_sign_in_at,
               Some(Utc.ymd(2017, 6, 5)
                   .and_hms_milli(18, 46, 29, 512)));
    assert_eq!(user_public.last_activity_on,
               Some(NaiveDate::from_ymd(2017, 10, 10)));
    assert_eq!(user_public.confirmed_at,
               Some(Utc.ymd(2015, 2, 26)
                   .and_hms_milli(15, 58, 34, 660)));
    assert_eq!(user_public.email, "kwrobot@kitware.com");
    assert_eq!(user_public.theme_id, None);
    assert_eq!(user_public.color_scheme_id, ColorSchemeId::new(4));
    assert_eq!(user_public.projects_limit, 50);
    assert_eq!(user_public.current_sign_in_at,
               Some(Utc.ymd(2017, 7, 10)
                   .and_hms_milli(15, 35, 58, 336)));
    assert!(user_public.identities.is_empty());
    assert_eq!(user_public.can_create_group, true);
    assert_eq!(user_public.can_create_project, true);
    assert_eq!(user_public.two_factor_enabled, true);
    assert_eq!(user_public.external, false);
}
