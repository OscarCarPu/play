package iteration

import "strings"

func Repeat(character string, repeatCount int) string {
	var repeated strings.Builder
	for range make([]int, repeatCount) {
		repeated.WriteString(character)
	}
	return repeated.String()
}
