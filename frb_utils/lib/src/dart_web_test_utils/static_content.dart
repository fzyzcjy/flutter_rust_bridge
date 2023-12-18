const kTestEntrypointHtmlContent = r'''
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Flutter Rust Bridge Pure Dart</title>
  </head>
  <body class="console">
    <div id="output"></div>
    <style>
      .red {
        color: red;
      }
      .green {
        color: green;
      }
      .yellow {
        color: yellow;
      }
      .console {
        background-color: #111;
      }
      a {
        color: darkorange;
      }
      pre {
        font-family: SF Mono, "JetBrainsMono Nerd Font", "Fira Sans", "Hack",
          "Monaco", "Courier New", Consolas, Courier, monospace;
        overflow: auto;
        color: #aaa;
      }
      .error {
        background-color: #633;
        overflow: auto;
        padding: 14px;
        border-top: 3px solid red;
        border-bottom: 3px solid red;
      }
    </style>
  </body>
  <script>
    const log = console.log;
    const output = document.getElementById("output");
    const linkRegex = /(http:\/\/.*\.(js|wasm))/g;
    const colorize = (value) => {
      const pre = document.createElement("pre");
      pre.innerHTML = new Option(String(value)).innerHTML
        .replaceAll("\033[31m", "<span class='red'>")
        .replaceAll("\033[32m", "<span class='green'>")
        .replaceAll("\033[33m", "<span class='yellow'>")
        .replaceAll("\033[0m", "</span>")
        .replaceAll("\033[1m", "<strong>")
        .replaceAll(linkRegex, '<a href="\$1">\$1</a>');
      return pre;
    };
    const decorateError = (value) => {
      const pre = document.createElement("pre");
      pre.innerHTML = new Option(String(value)).innerHTML.replaceAll(
        linkRegex,
        '<a href="\$1">\$1</a>'
      );
      pre.classList.add("error");
      return pre;
    };
    const href = window.location.href.replace("http", "ws");
    const channel = new WebSocket(href);
    console.log = (...args) => {
      for (const a of args) {
        output.appendChild(colorize(a));
        channel.send(JSON.stringify(a));
      }
      log(...args);
    };
    const error = console.error;
    console.error = (...args) => {
      for (const a of args) {
        output.append(decorateError(a));
        channel.send(JSON.stringify(a));
      }
      error(...args);
    };
  </script>
  <script src="main.dart.js" async></script>
</html>
''';
