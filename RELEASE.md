# How to release

1. Create branch for next version e.g.: 0.0.3 from develop
2. Adjust all versions in files: VERSION, PKGBUILD
3. (Optional) Test installation of package with: `makepkg -iCcfs`
4. Copy PKGBUILD to local repository of:  `ssh://aur@aur.archlinux.org/chwp-git`
5. Generate `.SRCINFO` with `./generate-srcinfo.sh`
6. Commit changes & push to AUR Repository