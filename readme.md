# nowdo
nowdo is an todo list that helps you stay on track.

I have tried a few different todo lists, and they all had one problem in common. If you don't look at them they are not very useful.

That is why I created nowdo, a todo list that reminds you what you should
be working on

It will read off the first item in your todo list every 10 minutes and 
create a notification for you reminding you.

## time
You can specify a different time by useing the --time flag,
``` 
./nowdo -t 30
```
The above will cause nowdo to alert you every 30 minutes

## editing todo.md
You can edit todo.md however you normally would, its just a simple markdown
file afterall, but you can also use `./nowdo edit` to use your $EDITOR your editor to edit your todo list


~/todo.md should be in the following format

```
#this is a task's title
This is a description of the task
it can have as many lines as you want.

#task2
This wont be shown by nowdo until you open
~/todo.md and remove the first task

#buy milk
im almost out
```
