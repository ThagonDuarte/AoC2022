import numpy as np
with open("src/bin/day01/input.txt") as file:
    current_calories = 0
    most_calories = 0
    for line in file:
        if line != "\n":
            current_calories =+ int(line)  
        if line == "\n":
            if current_calories > most_calories: 
                most_calories = current_calories
            current_calories = 0

    print(most_calories)

with open("src/bin/day01/input.txt") as file:
    current_calories = 0
    most_calories = 0
    elves = np.zeros(3000)
    i = 0
    new_line = False
    for line in file:
        print(line)
        if (line != "\n") & (new_line == True):
            new_line = False 
        if (line != "\n") & (new_line == False):
            current_calories = current_calories + int(line)
        if (line == "\n") & (new_line == False):
            new_line = True
        if (line == "\n") & (new_line == True):
            print(current_calories)
            elves[i] = current_calories
            current_calories = 0
            i = i + 1
            print("Enter")
            
    n = elves.shape[0]
    elves.sort()
    print(elves)
    print(max(elves))
    print(elves[n-1] + elves[n-2] + elves[n-3])