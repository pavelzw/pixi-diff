Output .github/assets/demo/demo-dark.mp4
Output .github/assets/demo/demo-dark.gif

Set Theme "rose-pine-moon"
Set Shell "bash"
Set FontSize 28
Set Width 1920
Set Height 750

# need to have pixi-diff-to-markdown and mdt (md-tui) installed
# pixi global install md-tui
# pixi global install pixi-diff-to-markdown
Hide
Type "export PATH=$(pwd)/target/debug:$PATH" Enter
Type "rm -rf $HOME/pixi-diff-example" Enter
Type "git clone https://github.com/pavelzw/pixi-pycharm $HOME/pixi-diff-example" Enter
Sleep 3s
Type "pushd $HOME/pixi-diff-example" Enter
Type "git checkout 5c681d61686f7b29a4358995f511c6943e5297f5" Enter
Type "clear" Enter
Sleep 1s
Show

Type@150ms "git show HEAD~20:pixi.lock | pixi-diff --before - --after pixi.lock | pixi-diff-to-markdown | mdt" Sleep 1s Enter
Sleep 5s

Down@150ms 20
Sleep 8s
