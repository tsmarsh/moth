.moth/hooks directory allows specifying shell scripts that will be executed before or after moth actions.

the structure (command name, before/after, script name):
./moth/hooks/
    new/          
        before/   
        after/    
            setup-git-branch.sh
    done/         
        before/   
        after/
            commit-final-spec-changes.sh
