# Main UI Elements
workspace-title = Your Git workspace
placeholder-no-repos = No repositories yet. Clone your first repository above.
publish-hint = All changes will be committed and pushed to the remote.
title-mock = 🐱 NekoSource
btn-refresh = Refresh
btn-clone = Clone
btn-settings = Settings
btn-about = About
btn-close = Close
btn-delete = Delete Selected
btn-commit = Publish Changes
btn-save = Save
btn-ok = OK
btn-cancel = Cancel

# Labels & Placeholders
label-url = URL:
hint-branch = branch
label-local-repos = Local Repositories:
label-selected-repo = Selected Repository:
label-commit-msg = Commit Message:
label-username = Username:
label-email = Email:
label-gpg = Sign commits via GPG
label-language = Language:
label-dark-theme = Dark theme
label-gpg-id = GPG Key ID:
label-recent-commits = Recent commits
label-changed-files = Changed files

placeholder-select = Select a repository from the list on the left to manage changes
placeholder-no-commits = No commits yet
placeholder-no-changes = Working tree clean

# Profile & About Windows
settings-title = Settings
about-title = About NekoSource
about-description = Thanks for using NekoSource! ^^
about-copyright = © 2026-present OctoBanon
about-development = Development:
contributor-octobanon = - Lead developer
contributor-shadowcj = - Ukrainian translation & belarusian translation
contributor-xelframe = - Belarusian translation

# Statuses & GPG keys
status-loading = Working
status-success = Success
status-error = Error
status-cloning = Cloning...
status-committing = Committing...
gpg-no-keys = No keys found
gpg-refresh-keys = Refresh key list

# SSH Passphrase
passphrase-title = SSH Passphrase
passphrase-prompt = Enter passphrase for your SSH key:
passphrase-hint = Leave empty if the key has no passphrase

# Errors: SSH
err-ssh-cancelled = SSH passphrase was cancelled

# Errors: Git
err-git-dest-exists = Directory already exists: '{ $path }'
err-git-open = Failed to open repository: { $detail }
err-git-clone = Clone failed: { $detail }
err-git-stage = Failed to stage changes: { $detail }
err-git-write-tree = Failed to write index tree: { $detail }
err-git-find-tree = Failed to find tree object: { $detail }
err-git-signature = Failed to create author signature: { $detail }
err-git-commit = Failed to create commit: { $detail }
err-git-commit-buffer = Failed to create commit buffer: { $detail }
err-git-commit-encoding = Commit buffer contains invalid UTF-8
err-git-signed-commit = Failed to create signed commit: { $detail }
err-git-ref-update = Failed to update branch reference: { $detail }
err-git-head-set = Failed to move HEAD: { $detail }
err-git-push = Push failed: { $detail }
err-git-branch-unknown = Cannot determine current branch name

# Errors: Git Config
err-git-config-not-found = Global git config (~/.gitconfig) not found: { $detail }
err-git-config-open = Failed to open git config for writing: { $detail }
err-git-config-write-name = Failed to write user.name: { $detail }
err-git-config-write-email = Failed to write user.email: { $detail }

# Errors: GPG
err-gpg-launch = Failed to launch gpg (is GnuPG in PATH?): { $detail }
err-gpg-stdin = Failed to write to GPG stdin: { $detail }
err-gpg-wait = Failed to wait for GPG process: { $detail }
err-gpg-failure = GPG exited with error: { $detail }
err-gpg-encoding = GPG output is not valid UTF-8: { $detail }
