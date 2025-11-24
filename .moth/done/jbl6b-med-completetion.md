# Shell completion for moth commands

Tab completion for bash, zsh, and fish with dynamic completions for story IDs
and status names.

## End User Experience
```bash
moth sh<TAB>        → moth show
moth show <TAB>     → abc12  def34  ghi56  (list of story IDs)
moth show ab<TAB>   → moth show abc12
moth mv abc12 <TAB> → ready  doing  done   (list of statuses)
moth ls -s <TAB>    → ready  doing  done
moth priority abc12 <TAB> → top  bottom  above  below
moth priority abc12 above <TAB> → def34  ghi56  (other story IDs)
```

## Completion Types

### Static completions
- Command names: `init`, `new`, `ls`, `show`, `start`, `done`, `mv`, `edit`, `rm`, `report`, `priority`, `compact`
- Flags: `--priority`, `--no-edit`, `--status`, `--all`, `--since`, `--until`, `--compact`, `--no-compact`
- Priority subcommands: `top`, `bottom`, `above`, `below`
- Priority values: `crit`, `high`, `med`, `low`

### Dynamic completions
- Story IDs: query `.moth/*/` for all story files, extract IDs
- Status names: read from `.moth/config.yml`
- Story IDs excluding current: for `above`/`below` (don't suggest the story being moved)

## Installation

### Bash
```bash
# Add to ~/.bashrc
eval "$(moth completions bash)"

# Or generate to file
moth completions bash > /etc/bash_completion.d/moth
```

### Zsh
```bash
# Add to ~/.zshrc (before compinit)
eval "$(moth completions zsh)"

# Or generate to file
moth completions zsh > ~/.zfunc/_moth
```

### Fish
```bash
moth completions fish > ~/.config/fish/completions/moth.fish
```

## Command Specification
```
moth completions <shell>
```

Where `<shell>` is one of: `bash`, `zsh`, `fish`

Outputs completion script to stdout.

## Implementation Tasks

### 1. Add clap_complete dependency
- [ ] Add `clap_complete` to Cargo.toml
- [ ] Add `clap_complete_fig` if supporting Fig (optional)

### 2. Create completions command
- [ ] Create `src/cmd/completions.rs`
- [ ] Add `Completions { shell: String }` variant to Commands
- [ ] Generate static completions via `clap_complete::generate()`

### 3. Dynamic completion helpers
- [ ] `moth --list-ids` - output all story IDs, one per line
- [ ] `moth --list-statuses` - output all status names, one per line
- [ ] These are hidden flags, not user-facing commands
- [ ] Fast path: skip config validation, just scan filesystem

### 4. Bash completion script
```bash
_moth_completions() {
    local cur prev words cword
    _init_completion || return

    case "${prev}" in
        show|start|done|edit|rm)
            COMPREPLY=($(compgen -W "$(moth --list-ids 2>/dev/null)" -- "${cur}"))
            return
            ;;
        mv)
            if [[ ${cword} -eq 2 ]]; then
                COMPREPLY=($(compgen -W "$(moth --list-ids 2>/dev/null)" -- "${cur}"))
            elif [[ ${cword} -eq 3 ]]; then
                COMPREPLY=($(compgen -W "$(moth --list-statuses 2>/dev/null)" -- "${cur}"))
            fi
            return
            ;;
        priority)
            if [[ ${cword} -eq 2 ]]; then
                COMPREPLY=($(compgen -W "$(moth --list-ids 2>/dev/null)" -- "${cur}"))
            elif [[ ${cword} -eq 3 ]]; then
                COMPREPLY=($(compgen -W "top bottom above below" -- "${cur}"))
            elif [[ ${prev} == "above" || ${prev} == "below" ]]; then
                COMPREPLY=($(compgen -W "$(moth --list-ids 2>/dev/null)" -- "${cur}"))
            fi
            return
            ;;
        -s|--status|compact)
            COMPREPLY=($(compgen -W "$(moth --list-statuses 2>/dev/null)" -- "${cur}"))
            return
            ;;
        -p|--priority)
            COMPREPLY=($(compgen -W "crit high med low" -- "${cur}"))
            return
            ;;
    esac

    if [[ ${cur} == -* ]]; then
        # Flag completion handled by clap_complete
        ...
    else
        COMPREPLY=($(compgen -W "init new ls show start done mv edit rm report priority compact completions" -- "${cur}"))
    fi
}
complete -F _moth_completions moth
```

### 5. Zsh completion script
```zsh
#compdef moth

_moth() {
    local state

    _arguments -C \
        '1: :->command' \
        '*: :->args'

    case $state in
        command)
            _values 'commands' \
                'init[Initialize .moth/ directory]' \
                'new[Create a new issue]' \
                'ls[List issues]' \
                'show[Show issue details]' \
                'start[Move issue to doing]' \
                'done[Move issue to done]' \
                'mv[Move issue to status]' \
                'edit[Edit issue]' \
                'rm[Delete issue]' \
                'report[Export history as CSV]' \
                'priority[Set story priority]' \
                'compact[Renumber priorities]' \
                'completions[Generate shell completions]'
            ;;
        args)
            case $words[2] in
                show|start|done|edit|rm)
                    _values 'story' $(moth --list-ids 2>/dev/null)
                    ;;
                mv)
                    if (( CURRENT == 3 )); then
                        _values 'story' $(moth --list-ids 2>/dev/null)
                    elif (( CURRENT == 4 )); then
                        _values 'status' $(moth --list-statuses 2>/dev/null)
                    fi
                    ;;
                priority)
                    if (( CURRENT == 3 )); then
                        _values 'story' $(moth --list-ids 2>/dev/null)
                    elif (( CURRENT == 4 )); then
                        _values 'position' top bottom above below
                    elif [[ $words[4] == (above|below) ]] && (( CURRENT == 5 )); then
                        _values 'story' $(moth --list-ids 2>/dev/null)
                    fi
                    ;;
                compact)
                    _values 'status' $(moth --list-statuses 2>/dev/null)
                    ;;
                ls)
                    _arguments '-s[Filter by status]:status:($(moth --list-statuses 2>/dev/null))' \
                               '-a[Show all]'
                    ;;
                completions)
                    _values 'shell' bash zsh fish
                    ;;
            esac
            ;;
    esac
}

_moth
```

### 6. Fish completion script
```fish
# Commands
complete -c moth -f -n "__fish_use_subcommand" -a init -d "Initialize .moth/ directory"
complete -c moth -f -n "__fish_use_subcommand" -a new -d "Create a new issue"
complete -c moth -f -n "__fish_use_subcommand" -a ls -d "List issues"
complete -c moth -f -n "__fish_use_subcommand" -a show -d "Show issue details"
complete -c moth -f -n "__fish_use_subcommand" -a start -d "Move issue to doing"
complete -c moth -f -n "__fish_use_subcommand" -a done -d "Move issue to done"
complete -c moth -f -n "__fish_use_subcommand" -a mv -d "Move issue to status"
complete -c moth -f -n "__fish_use_subcommand" -a edit -d "Edit issue"
complete -c moth -f -n "__fish_use_subcommand" -a rm -d "Delete issue"
complete -c moth -f -n "__fish_use_subcommand" -a report -d "Export history as CSV"
complete -c moth -f -n "__fish_use_subcommand" -a priority -d "Set story priority"
complete -c moth -f -n "__fish_use_subcommand" -a compact -d "Renumber priorities"
complete -c moth -f -n "__fish_use_subcommand" -a completions -d "Generate shell completions"

# Dynamic story ID completion
complete -c moth -f -n "__fish_seen_subcommand_from show start done edit rm" -a "(moth --list-ids 2>/dev/null)"

# mv: id then status
complete -c moth -f -n "__fish_seen_subcommand_from mv; and __fish_is_token_n 3" -a "(moth --list-ids 2>/dev/null)"
complete -c moth -f -n "__fish_seen_subcommand_from mv; and __fish_is_token_n 4" -a "(moth --list-statuses 2>/dev/null)"

# priority: id then position then optional target id
complete -c moth -f -n "__fish_seen_subcommand_from priority; and __fish_is_token_n 3" -a "(moth --list-ids 2>/dev/null)"
complete -c moth -f -n "__fish_seen_subcommand_from priority; and __fish_is_token_n 4" -a "top bottom above below"
complete -c moth -f -n "__fish_seen_subcommand_from priority; and __fish_is_token_n 5" -a "(moth --list-ids 2>/dev/null)"

# compact: optional status
complete -c moth -f -n "__fish_seen_subcommand_from compact" -a "(moth --list-statuses 2>/dev/null)"

# completions: shell names
complete -c moth -f -n "__fish_seen_subcommand_from completions" -a "bash zsh fish"
```

### 7. Performance optimization
- [ ] `--list-ids` and `--list-statuses` must be fast (<50ms)
- [ ] Skip full config validation
- [ ] Direct filesystem scan, no git operations
- [ ] Cache results? Probably not needed for small repos

## Edge Cases

- [ ] No `.moth/` directory: output nothing, exit 0 (fail silently for completions)
- [ ] Partial ID matching: let shell handle prefix matching
- [ ] Very long ID lists: shell handles truncation
- [ ] Special characters in slugs: properly quote/escape
- [ ] Running outside repo: no completions for dynamic values

## Testing

- [ ] Test each shell in Docker container
- [ ] Test completion at each argument position
- [ ] Test with 0, 1, 10, 100 stories
- [ ] Test in nested directory (should find .moth/ in parent)

## Acceptance Criteria

- [ ] `moth <TAB>` shows all commands (all shells)
- [ ] `moth show <TAB>` shows story IDs (all shells)
- [ ] `moth mv abc12 <TAB>` shows statuses (all shells)
- [ ] `moth priority abc12 above <TAB>` shows other story IDs
- [ ] Completions work when running moth from subdirectory
- [ ] `moth completions bash|zsh|fish` outputs valid script
- [ ] No errors or slowdown when `.moth/` doesn't exist