# /bin/bash generate_day.sh
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

if [ $day -lt 10 ]; then
    day="0$day"
fi

if [ -d "day$day" ]; then
    echo "Day $day already exists"
    exit 1
elif [ $day -gt 25 ]; then
    echo "Day $day is too high"
    exit 1
elif [ $day -lt 1 ]; then
    echo "Day $day is too low"
    exit 1
else
    echo "Creating day$day"
fi

prevDay=$((day-1))
dayNum=$((day))
formattedPrevDay=0

if [ $prevDay -lt 10 ]; then
    formattedPrevDay="0$prevDay"
fi

mkdir -p "day$day"

touch "day$day/input.txt"
touch "day$day/example.txt"

touch "day$day/solution.rs"
echo "const INPUT: &str = include_str!(\"input.txt\");" >> "day$day/solution.rs"
echo "pub fn part1() {" >> "day$day/solution.rs"
echo "}" >> "day$day/solution.rs"
echo "pub fn part2() {" >> "day$day/solution.rs"
echo "}" >> "day$day/solution.rs"

touch "day$day/mod.rs"
echo "pub mod solution;" >> "day$day/mod.rs"

echo "pub mod day$day;" >> "lib.rs"

echo "" >> "main.rs"
echo "" >> "main.rs"
echo "fn day$day() {" >> "main.rs"
echo "    day$day::solution::part1();" >> "main.rs"
echo "    day$day::solution::part2();" >> "main.rs"
echo "}" >> "main.rs"

# add the day to the match statement (black magic)
sed -i "/$prevDay => day$formattedPrevDay(),/a \ \ \ \ \ \ \ \ $dayNum => day$day()," "main.rs"

echo "Day $day created successfully"
cargo fmt