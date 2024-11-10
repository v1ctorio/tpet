# TermPet
`termpet`
TermPet is a simple terminal game where you have a virtual pet cow that you need to play with and feed to keep it happy and healthy.

The idea is that you add it to your shell init and it greets you (and tells you if it wants to play or eat) every time you open a new terminal.
The project is currently only text but the idea is to make it like terminal drawings for your pet like cowsay or that, current TODO is:

## TODO
- [x] Save the pets data in files
- [x] Add a command to create the pet
- [x] Add a command to play with the pet and feed it
- [x] Add a way to see the pet's stats
- [ ] Add minigames like rock paper scissors, asking questions (input a valid tar command for example), guess the number... to feed/play with the pet
- [x] Make the pet stats lower when real time passes (calculates the time between the last time the pet was interacted and the current time)
- [ ] Add a way to change the pet's name
- [ ] Add a way to change the pet's appearance
- [ ] Set graphical pet appearance
- [x] Make the pet say greetings and ask for food/play in a random way
- [ ] Add VHS gifs to this readme
- [ ] Package/make binaries
- [ ] Load customs phrases from files/teach pets phrases
- [ ] Check windows compatibility

Currently, the most difficult (for me as a rust beginner) features are implemented but theres a long todo there, you can see the full command list using `termpet --help`

Enjoy playing with your terminal pet!