# dir-tree-html

Renders a directory tree as a single HTML page. Written in [Spiral](https://github.com/i574n/spiral), compiled to Rust.

```powershell
pwsh scripts/init.ps1    # links deps/spiral to a sibling spiral checkout, cloning it when missing
pwsh build.ps1           # Spiral -> dir_tree_html.rs -> dist/dir-tree-html, then its --self-test
dist/dir-tree-html --dir <folder> --html <folder>/index.html
```
