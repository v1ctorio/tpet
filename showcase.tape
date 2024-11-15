
Require echo

Set Shell "fish"
Set FontSize 32
Set Width 1200
Set Height 600


Type "alias tpet='./target/debug/termpet'" Sleep 200ms  Enter
Type "export TPET_FILE_PATH=$(mktemp -d)/gregory.pet" Sleep 200ms  Enter
Type "cargo build > /dev/null" Hide Sleep 500ms  Enter
Type 'clear' Enter
Sleep 200ms
Show
Sleep 200ms
Type "tpet --help" Sleep 1000ms  Enter
Type "tpet --init" Sleep 600ms  Enter
Type "Gregory" Sleep 600ms  Enter
Sleep 5s

Output assets/showcase.gif
Output assets/showcase.mp4