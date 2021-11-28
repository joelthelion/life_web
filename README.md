Life simulation written in Rust, inspired by this very old screensaver: https://sourceforge.net/projects/primlife/

Demo: https://joelthelion.github.io/life_web/demo/
  
Biots are allowed to evolve through mutation and natural selection.
Biots have a simple genome giving them unique characteristics:
  - Green allows them to collect energy from the sun
  - Red allows them to eat other biots
  - Dark blue allows them to defend
  - Light blue allows them to move around

A special trait, intelligence (denoted by a square), allows them to move towards the nearest edible biot instead of randomly.
All non-green traits cost energy.
