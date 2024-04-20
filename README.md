Todo is a super fast and simple tasks organizer written in rust

Usage: todo-app [COMMAND] [ARGUMENTS]

Example: todo-app list

    Available commands:
        - add [TASK/s]
            adds new task/s
            Example: todo add \"buy carrots\"
        - list
            lists all tasks
            Example: todo list
        - done [INDEX]
            marks task as done
            Example: todo done 2 3 (marks second and third tasks as completed)
        - rm [INDEX]
            removes a task
            Example: todo rm 4
        - reset
            deletes all tasks
        - restore 
            restore recent backup after reset
