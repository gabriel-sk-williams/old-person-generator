## A simple command-line Old Person Generator using weighted RNG in Rust.

![screenshot.jpg](/../media/jpg/screenshot.jpg?raw=true "Old Person Generator screenshot")

• Names are randomly selected from a vec of pre-collected names from generations past.

• The remaining stats are rolled using weighted distributions.

• Age is rolled along a decay function, weighted towards 65 or above, where:


#### &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; y = amplitude * e<sup>kx</sup>

• Ailments are then rolled according to age; younger old persons tend to have few ailments.

• The remaining Stats (Strength, Sight, Mobility, Wisdom) are rolled along a -growth curve- moderated by age, where:

#### &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; y = amplitude * ln(e)

• The older the person, the higher the stats tend to roll. Each Stat is then down-regulated by the presence of certain Ailments. The "Frailty" ailment imposes a downward modifier on the Strength Stat, for instance.
