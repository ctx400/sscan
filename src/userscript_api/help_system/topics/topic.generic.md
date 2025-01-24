# sscan - A scriptable file/process/network scanner #

## Basic Commands ##

| Command           | Description                               |
| ----------------- | ----------------------------------------- |
| help()            | View this general help message.           |
| help:topics()     | List all available help topics.           |
| help 'topic'      | View detailed help on 'topic'.            |

## Interactive Mode ##

In interactive mode, sscan provides a REPL which accepts multiline Lua
input terminated by a semicolon.

## More Help ##

- To get detailed help on a topic, use `help 'topic'`
- To list all topics, use `help:topics()`
