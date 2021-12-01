package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func part1(depths []int) int {
	increases := 0
	for i, v := range depths {
		if i >= 1 && (v > depths[i-1]) {
			increases++
		}
	}
	return increases
}

func sum(values []int) int {
	result := 0
	for _, v := range values {
		result += v
	}
	return result
}

func part2(depths []int) int {
	increases := 0
	for i := range depths {
		if i < len(depths)-3 {
			window1 := depths[i : i+3]
			window2 := depths[i+1 : i+4]
			if sum(window2) > sum(window1) {
				increases++
			}
		}
	}
	return increases
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	depths := make([]int, 0, 4096)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line != "" {
			depth, err := strconv.Atoi(line)
			if err != nil {
				log.Fatal(err)
			}
			depths = append(depths, depth)
		}
	}

	fmt.Printf("Part 1 answer: %v\n", part1(depths))
	fmt.Printf("Part 2 answer: %v\n", part2(depths))
}
