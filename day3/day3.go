package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

const NUMBER_OF_BITS = 12

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

	logs := make([]string, 0, 4096)
	for scanner.Scan() {
		line := scanner.Text()
		logs = append(logs, line)
	}

	fmt.Printf("Part 1 answer: %v\n", part1(logs))

	fmt.Printf("Part 2 answer: %v\n", part2(logs))

}

func part1(ss []string) int {
	result_ones := 0
	result_zeroes := 0
	for i := NUMBER_OF_BITS - 1; i >= 0; i-- {
		ones := 0
		nb_lines := 0
		for _, line := range ss {
			nb_lines++
			if string(line[i]) == "1" {
				ones++
			}
		}
		if ones > nb_lines/2 {
			result_ones += int(math.Pow(2.0, float64(NUMBER_OF_BITS-1-i)))
		} else {
			result_zeroes += int(math.Pow(2.0, float64(NUMBER_OF_BITS-1-i)))
		}
	}
	return result_ones * result_zeroes
}

func part2(ss []string) int {
	return get_rating(ss, true) * get_rating(ss, false)
}

func filter(ss []string, test func(string) bool) (ret []string) {
	for _, s := range ss {
		if test(s) {
			ret = append(ret, s)
		}
	}
	return
}

func get_rating(ss []string, most_common bool) (ret int) {
	candidates := make([]string, len(ss))
	copy(candidates, ss)
	for i := 0; i < NUMBER_OF_BITS; i++ {
		ones := 0
		nb_lines := 0
		var most_common_bit byte
		for _, line := range candidates {
			nb_lines++
			if line[i] == '1' {
				ones++
			}
		}
		var criteria func(s string) bool
		if float64(ones) >= float64(nb_lines)/2.0 {
			most_common_bit = '1'
		} else {
			most_common_bit = '0'
		}
		criteria = func(s string) bool {
			if most_common {
				return s[i] == most_common_bit
			} else {
				return s[i] != most_common_bit
			}
		}
		candidates = filter(candidates, criteria)
		if len(candidates) == 1 {
			result, err := strconv.ParseInt(candidates[0], 2, 32)
			if err != nil {
				return 0
			}
			ret = int(result)

		}
	}
	return ret
}
