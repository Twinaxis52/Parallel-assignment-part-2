# Parallel-assignment-part-1-and-2
To run the program u need t
First to be able to run this files u will need to rename them main.rs(VERY IMPORTANT) I just change them to part 1 and part 2 to be able to include in one repository also u will need to have install rust using this website https://code.visualstudio.com/docs/languages/rust this will set u up perfectly with a main.rs. You will want to obviously build it with cargo build and can try to debug it first so it can set that up. After that you will run this command to build it rustc main.rs if it isnt under a folder if it is under a folder and you follow the website i posted it would be rustc src\main.rs should work since it will be under the src folder and this command to run it cargo run or .\main.exe

For part 1 of the assignment i have made the minotaur choose the guest/threads randomly and have made in the code a cupcake tracker that will let the guest request a cupcake if it isnt there and if they eat the cupcake the coutner will go up and then when all the guest have eaten a cupcake a print message saying all guest have eaten the cupcake will pop up. This way the guest have not ignored the generous hospitality of the minotaur and they won't talk to each other since theres no receiver or sender between the threads.

For part 2 of the assignment I have chosen the second strategy because the minotaur doesn't want many guest around the vase to break it so I believe a long line could still pose a threat and the first strategy could make a lot of people enter the room which is worse. Therefore, i have went with the second strategy which is showing other guest the room si busy or available.

All of my threads run random so the minotaur would be choosing the guest random and i have a flag that is represented by busy and a count to show how many guest saw the vase.
