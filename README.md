# cop

cop copies a text selected from the list to the clipboard.

## How to use

### Show text list

`cop ls` shows current list. (alias is `l`)

```
% cop ls
Index | Text
--------------------------------------------------
    0 | my-email-address@gmail.com
    1 | HelloWorld
    2 | abcdefghijklmnopqrstuvwxyz0123456789
```

> It works as `ls` even if no subcommand is specified.

### Copy text to Clipboard from the list

`cop get <index>` copies text to Clipboard from list. (alias is `g`)

```
% cop get 1
HelloWorld
```

### Add new text

`cop add` adds a new text to the list. (alias is `a`)

```
% cop add abc
Add 0:abc
```

`0` is a index assigned to the entered text. This index is used to get text.

### Remove text

`cop rm` removes the text at the specified index from the list. (alias is `r`)

```
% cop rm 0
Remove abc
```

## How to install

```
brew tap mitsutoshi/
brew install mitsutoshi/
```
