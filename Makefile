prepare:
	cp pre-commit.sample .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
	echo "pre-commit hook installed successfully."
	cp pre-push.sample .git/hooks/pre-push
	chmod +x .git/hooks/pre-push
	echo "pre-push hook installed successfully."
	cargo install cargo-nextest
	cargo install cargo-edit

build:
	cargo build --all-features

test:
	cargo nextest run

bump-version:
	@current_branch=$$(git rev-parse --abbrev-ref HEAD); \
	if [[ $$current_branch == feat* ]]; then \
		echo "Feature branch detected"; \
		$(MAKE) feat-command; \
	elif [[ $$current_branch == fix* ]]; then \
		echo "Fix branch detected"; \
		$(MAKE) fix-command; \
	else \
		echo "Other branch detected - Do not bump version"; \
	fi

feat-command:
	@echo "Running feature command..."

fix-command:
	@cargo set-version --bump patch; \
	$(MAKE) push-git

push-git:
	@cargo build --all-features; \
	git pull --rebase; \
	git add .; \
	git commit --amend --no-edit --allow-empty --no-verify; \
