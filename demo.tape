  Output public/vhs/demo-desktop.webm
  Set Shell "bash"
  Set FontSize 14
  Set Width 1200
  Set Height 600
  Set Padding 16
  Set Theme {
    "background":    "#0a0a0a",
    "foreground":    "#ededed",
    "cursor":        "#5fb878",
    "black":         "#0a0a0a",
    "red":           "#b8860b",
    "green":         "#5fb878",
    "yellow":        "#d4a017",
    "blue":          "#ededed",
    "magenta":       "#ededed",
    "cyan":          "#ededed",
    "white":         "#ededed",
    "brightBlack":   "#8a8a8a",
    "brightGreen":   "#5fb878"
  }

  Type "mz status"
  Sleep 500ms
  Enter
  Sleep 2s
  Type "mz plan"
  Sleep 500ms
  Enter
  Sleep 3s
  Type "mz auto"
  Sleep 500ms
  Enter
  Sleep 5s
