# dir-tree-html

[![깃허브 코드스페이스](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo=1411208249)

Renders a directory tree as a single HTML page. Written in [Spiral](https://github.com/i574n/spiral), compiled to Rust.

```powershell
pwsh scripts/init.ps1    # links deps/spiral to a sibling spiral checkout, cloning it when missing
pwsh build.ps1           # Spiral -> dir_tree_html.rs -> dist/dir-tree-html, then its --self-test
dist/dir-tree-html --dir <folder> --html <folder>/index.html
```
