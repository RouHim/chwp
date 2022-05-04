#!/bin/sh
AUR_PROJECT_DIR=/home/rouven/projects/chwp-git-aur
PROJECT_DIR=$(pwd)
CURRENT_VERSION=$(cat VERSION)
NEXT_VERSION=$CURRENT_VERSION

build_application() {
  cd "$PROJECT_DIR"
  cargo build --release
}

calculate_next_version() {
  NEXT_VERSION=$(echo "$CURRENT_VERSION" | awk -F. '{$NF = $NF + 1;} 1' | sed 's/ /./g')
  echo "next version is $NEXT_VERSION"
}

create_branch() {
  echo "creating new branch"
  cd "$PROJECT_DIR"
  git checkout -b "$NEXT_VERSION"
}

update_PKGBUILD_version() {
  echo "incrementing PKDBUILD file:"
  cd "$PROJECT_DIR"
  sed -i "s/pkgver=.*/pkgver=$NEXT_VERSION/g" PKGBUILD
  cat PKGBUILD | grep 'pkgver='
}

commit_and_push() {
  cd "$PROJECT_DIR"
  git commit -a -m "Publishing new release: $NEXT_VERSION"
  git push --set-upstream origin "$NEXT_VERSION"
  git checkout develop
  git merge "$NEXT_VERSION"
  git push
}

copy_pkgbuild_to_aur_project() {
  cd "$PROJECT_DIR"
  cp PKGBUILD $AUR_PROJECT_DIR
}

generate_srcinfo_file() {
  echo "Generating .SRCINFO"
  cd $AUR_PROJECT_DIR
  makepkg --printsrcinfo >.SRCINFO
}

commit_and_push_aur() {
  cd $AUR_PROJECT_DIR
  git commit -a -m "Publishing new release: $NEXT_VERSION"
  git push
}

write_to_version_file() {
  cd "$PROJECT_DIR"
  echo "$NEXT_VERSION" >VERSION
}

# preperation
build_application
calculate_next_version

# project
create_branch
update_PKGBUILD_version
write_to_version_file
commit_and_push

# aur
copy_pkgbuild_to_aur_project
generate_srcinfo_file
commit_and_push_aur

echo "Released version: $NEXT_VERSION"
