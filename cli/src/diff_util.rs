use jj_lib::merge::Merge;
use jj_lib::merged_tree::{MergedTree, TreeDiffIterator};
use jj_lib::{conflicts, diff, files, rewrite};
    from_tree: &MergedTree,
    to_tree: &MergedTree,
    let from_tree = MergedTree::legacy(rewrite::merge_commit_trees(
        workspace_command.repo().as_ref(),
        &parents,
    )?);
    let to_tree = commit.merged_tree()?;
    value: &Merge<Option<TreeValue>>,
    match value.as_resolved() {
        Some(None) => Ok(vec![]),
        Some(Some(TreeValue::File { id, .. })) => {
        Some(Some(TreeValue::Symlink(id))) => {
        Some(Some(TreeValue::GitSubmodule(id))) => {
        None => {
            conflicts::materialize(value, repo.store(), path, &mut content).unwrap();
        Some(Some(TreeValue::Tree(_))) | Some(Some(TreeValue::Conflict(_))) => {
            panic!("Unexpected {value:?} in diff at path {path:?}",);
        }
fn basic_diff_file_type(values: &Merge<Option<TreeValue>>) -> String {
    match values.as_resolved() {
        Some(None) => {
            panic!("absent path in diff");
        }
        Some(Some(TreeValue::File { executable, .. })) => {
        Some(Some(TreeValue::Symlink(_))) => "symlink".to_string(),
        Some(Some(TreeValue::Tree(_))) => "tree".to_string(),
        Some(Some(TreeValue::GitSubmodule(_))) => "Git submodule".to_string(),
        Some(Some(TreeValue::Conflict(_))) => {
            panic!("conflict in diff");
        }
        None => "conflict".to_string(),
    for (path, left_value, right_value) in tree_diff {
        if left_value.is_absent() {
            let right_content = diff_content(repo, &path, &right_value)?;
            let description = basic_diff_file_type(&right_value);
            writeln!(
                formatter.labeled("header"),
                "Added {description} {ui_path}:"
            )?;
            if right_content.is_empty() {
                writeln!(formatter.labeled("empty"), "    (empty)")?;
            } else {
                show_color_words_diff_hunks(&[], &right_content, formatter)?;
        } else if right_value.is_present() {
            let left_content = diff_content(repo, &path, &left_value)?;
            let right_content = diff_content(repo, &path, &right_value)?;
            let description = match (left_value.into_resolved(), right_value.into_resolved()) {
                (
                    Ok(Some(TreeValue::File {
                        executable: left_executable,
                        ..
                    })),
                    Ok(Some(TreeValue::File {
                        executable: right_executable,
                        ..
                    })),
                ) => {
                    if left_executable && right_executable {
                        "Modified executable file".to_string()
                    } else if left_executable {
                        "Executable file became non-executable at".to_string()
                    } else if right_executable {
                        "Non-executable file became executable at".to_string()
                    } else {
                        "Modified regular file".to_string()
                (Err(_), Err(_)) => "Modified conflict in".to_string(),
                (Err(_), _) => "Resolved conflict in".to_string(),
                (_, Err(_)) => "Created conflict in".to_string(),
                (Ok(Some(TreeValue::Symlink(_))), Ok(Some(TreeValue::Symlink(_)))) => {
                    "Symlink target changed at".to_string()
                }
                (Ok(left_value), Ok(right_value)) => {
                    let left_type = basic_diff_file_type(&Merge::resolved(left_value));
                    let right_type = basic_diff_file_type(&Merge::resolved(right_value));
                    let (first, rest) = left_type.split_at(1);
                    format!(
                        "{}{} became {} at",
                        first.to_ascii_uppercase(),
                        rest,
                        right_type
                    )
                }
            };
            writeln!(formatter.labeled("header"), "{description} {ui_path}:")?;
            show_color_words_diff_hunks(&left_content, &right_content, formatter)?;
        } else {
            let left_content = diff_content(repo, &path, &left_value)?;
            let description = basic_diff_file_type(&left_value);
            writeln!(
                formatter.labeled("header"),
                "Removed {description} {ui_path}:"
            )?;
            if left_content.is_empty() {
                writeln!(formatter.labeled("empty"), "    (empty)")?;
            } else {
                show_color_words_diff_hunks(&left_content, &[], formatter)?;
    value: &Merge<Option<TreeValue>>,
    match value.as_resolved() {
        Some(Some(TreeValue::File { id, executable })) => {
        Some(Some(TreeValue::Symlink(id))) => {
        Some(Some(TreeValue::GitSubmodule(id))) => {
        None => {
            hash = "0000000000".to_string();
            conflicts::materialize(value, repo.store(), path, &mut content).unwrap();
        }
        Some(Some(TreeValue::Tree(_))) | Some(Some(TreeValue::Conflict(_))) | Some(None) => {
            panic!("Unexpected {value:?} in diff at path {path:?}");
    for (path, left_value, right_value) in tree_diff {
        if left_value.is_absent() {
            let right_part = git_diff_part(repo, &path, &right_value)?;
            formatter.with_label("file_header", |formatter| {
                writeln!(formatter, "diff --git a/{path_string} b/{path_string}")?;
                writeln!(formatter, "new file mode {}", &right_part.mode)?;
                writeln!(formatter, "index 0000000000..{}", &right_part.hash)?;
                writeln!(formatter, "--- /dev/null")?;
                writeln!(formatter, "+++ b/{path_string}")
            })?;
            show_unified_diff_hunks(formatter, &[], &right_part.content)?;
        } else if right_value.is_present() {
            let left_part = git_diff_part(repo, &path, &left_value)?;
            let right_part = git_diff_part(repo, &path, &right_value)?;
            formatter.with_label("file_header", |formatter| {
                writeln!(formatter, "diff --git a/{path_string} b/{path_string}")?;
                if left_part.mode != right_part.mode {
                    writeln!(formatter, "old mode {}", &left_part.mode)?;
                    writeln!(formatter, "new mode {}", &right_part.mode)?;
                    if left_part.hash != right_part.hash {
                        writeln!(formatter, "index {}...{}", &left_part.hash, right_part.hash)?;
                } else if left_part.hash != right_part.hash {
                    writeln!(
                        formatter,
                        "index {}...{} {}",
                        &left_part.hash, right_part.hash, left_part.mode
                    )?;
                }
                if left_part.content != right_part.content {
                    writeln!(formatter, "+++ b/{path_string}")?;
                }
                Ok(())
            })?;
            show_unified_diff_hunks(formatter, &left_part.content, &right_part.content)?;
        } else {
            let left_part = git_diff_part(repo, &path, &left_value)?;
            formatter.with_label("file_header", |formatter| {
                writeln!(formatter, "diff --git a/{path_string} b/{path_string}")?;
                writeln!(formatter, "deleted file mode {}", &left_part.mode)?;
                writeln!(formatter, "index {}..0000000000", &left_part.hash)?;
                writeln!(formatter, "--- a/{path_string}")?;
                writeln!(formatter, "+++ /dev/null")
            })?;
            show_unified_diff_hunks(formatter, &left_part.content, &[])?;
        for (repo_path, before, after) in tree_diff {
            if before.is_present() && after.is_present() {
                writeln!(
                    formatter.labeled("modified"),
                    "M {}",
                    workspace_command.format_file_path(&repo_path)
                )?;
            } else if before.is_absent() {
                writeln!(
                    formatter.labeled("added"),
                    "A {}",
                    workspace_command.format_file_path(&repo_path)
                )?;
            } else {
                writeln!(
                    formatter.labeled("removed"),
                    "R {}",
                    workspace_command.format_file_path(&repo_path)
                )?;
    for (repo_path, left, right) in tree_diff {
        let left_content = diff_content(workspace_command.repo(), &repo_path, &left)?;
        let right_content = diff_content(workspace_command.repo(), &repo_path, &right)?;
        for (repo_path, before, after) in tree_diff {
                diff_summary_char(&before),
                diff_summary_char(&after),
fn diff_summary_char(value: &Merge<Option<TreeValue>>) -> char {
    match value.as_resolved() {
        Some(None) => '-',
        Some(Some(TreeValue::File { .. })) => 'F',
        Some(Some(TreeValue::Symlink(_))) => 'L',
        Some(Some(TreeValue::GitSubmodule(_))) => 'G',
        None => 'C',
        Some(Some(TreeValue::Tree(_))) | Some(Some(TreeValue::Conflict(_))) => {
            panic!("Unexpected {value:?} in diff")
        }