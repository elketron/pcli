# PCLI 

a cli project manager

## commands 

currently there are only two commands namely project and templates
these let you add projects and templates. Templates are defined in two different categories "project templates" and "file templates". Right now file templates are very simple and will just be the generic file in the future I want to add stuff like vars in there for a more dinamic template.

## todo

- [X] add vars for file templates
- [ ] add todos
- [ ] add functionality for todos in project (every todoitem is a git branch and will merge on completion)
- [ ] add todo groups (same as normal todos in git terms.)
- [ ] add functionality for project commands and configuration in the .proj dir
- [ ] fix bugs
- [ ] think of more stuff to add
- [ ] add rules for how / where files should end up (in the project config file). with filetype

## using templates

using templates is done via the pcli template use command.
templates can contain variables in the format of "{{ var }}". 

vars you can use are 

- filename
- language
- date
- projectname

something that i might do is add a special config where you can add your own variable. But that would be for a way later version.
