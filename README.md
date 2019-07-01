<h1 align="center">Welcome to kraftfahrzeug 👋</h1>

> A serialization agnostic repl for websocket transports

## Design notes

 1. Panes
     - manage panes for inspecting messages and sending a message
     - move focus of pane
     - change contents of statusbar according to pane type
     - view pane name in the lower right corner instead of application name
     - move pane and resize
 2. Design inspect pane
     - TODO
 3. Design send pane
     - TODO
 4. Show messages in abstract object notation (like js objects in google chrome debugger)
     - add highlighting for object notation
        - strings
        - numbers
        - field names
        - parenthesis
        - abbreviations
     - add level based abbreviation for object notation

### Abbreviation for objects

```
{
  data: {
    from: "Sam",
    to: "Max",
    content: "Hello, world!"
  },
  type: "Message"
}
```

to

```
{ data: { from: "Sam", to: "Max", ... }, type: "Message" }
```

> Abbrevations are only present on level `n`, abbreviating all messages deeper than `n` and may abbreviate other message at level `n`. An item on a lower level than `n` (e.g. the root) will never be abbreviated.

## Author

👤 **Fin Christensen**

* Github: [@fin-ger](https://github.com/fin-ger)

## Show your support

Give a ⭐️ if this project helped you!

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
