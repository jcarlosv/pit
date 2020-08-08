# Rust project template

Personal template to create rust projects using `cargo-generate`

How to use:
```sh
REPOSITORY="git@github.com:repo/repo.git"
git_name=$(basename "${REPOSITORY}")
git_name="${git_name/.git/}"
cargo install cargo-generate --features vendored-openssl
cargo generate --git https://github.com/jcvenegas/rust-template.git  --name "${git_name}"
cd "${git_name}"
if command -v nix-shell; then
        nix-shell
fi
git remote add origin "${REPOSITORY}"
./.ci/setup.sh
./.ci/run.sh
git add .ci/ .github/ .gitignore .travis.yml *
git commit -s -m "init"
git push origin master
```

License: Apache-2.0 OR MIT
