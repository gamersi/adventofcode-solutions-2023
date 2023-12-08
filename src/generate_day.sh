#!/bin/bash
# Description: Generate a new day folder with the template files

if [ ${PWD##*/} != "src" ]; then
    if [ -d "src" ]; then
        echo "Attempting to change directory to src"
        cd src
    else
        echo "Please run this script from the src folder"
        exit 1
    fi
fi

read -p "Enter the day number: " day

formattedDay=$(printf "%02d" $day)
day=$(echo $day | sed 's/^0*//')  # Remove leading zeros

if [ -z "$day" ]; then
    echo "Invalid day number"
    exit 1
fi

if [ -d "day$day" ]; then
    echo "Day $formattedDay already exists"
    exit 1
elif [ $day -gt 25 ]; then
    echo "Day $formattedDay is too high"
    exit 1
elif [ $day -lt 1 ]; then
    echo "Day $formattedDay is too low"
    exit 1
else
    echo "Creating day$formattedDay"
fi

prevDay=$((day-1))
dayNum=$((day))
formattedPrevDay=$(printf "%02d" $prevDay)  # Add leading zeros if needed

mkdir -p "day$formattedDay"

touch "day$formattedDay/input.txt"
touch "day$formattedDay/example.txt"

touch "day$formattedDay/solution.rs"
echo "const INPUT: &str = include_str!(\"input.txt\");" >> "day$formattedDay/solution.rs"
echo "pub fn part1() {" >> "day$formattedDay/solution.rs"
echo "}" >> "day$formattedDay/solution.rs"
echo "pub fn part2() {" >> "day$formattedDay/solution.rs"
echo "}" >> "day$formattedDay/solution.rs"

touch "day$formattedDay/mod.rs"
echo "pub mod solution;" >> "day$formattedDay/mod.rs"

echo "pub mod day$formattedDay;" >> "lib.rs"

echo "" >> "main.rs"
echo "" >> "main.rs"
echo "fn day$formattedDay() {" >> "main.rs"
echo "    day$formattedDay::solution::part1();" >> "main.rs"
echo "    day$formattedDay::solution::part2();" >> "main.rs"
echo "}" >> "main.rs"

# add the day to the match statement (black magic)
sed -i "/$prevDay => day$formattedPrevDay(),/a \ \ \ \ \ \ \ \ $dayNum => day$formattedDay()," "main.rs"

echo "Day $formattedDay created successfully"
cargo fmt