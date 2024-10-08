tmux:
  tmux new -ds wt-synth -n "README"
  tmux send-keys -t wt-synth:README 'nv ./README.md "+set wrap"' ENTER
  @just new-window "Edit" ""
  @just new-window "Run" ""
  @just new-window "git" "git status"
  tmux a -t wt-synth

new-window NAME CMD:
  tmux new-w -t wt-synth -n "{{NAME}}"
  tmux send-keys -t wt-synth:"{{NAME}}" "{{CMD}}" ENTER

