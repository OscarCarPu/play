package main

import (
	"reflect"
	"slices"
	"testing"
)

func TestSum(t *testing.T) {
	t.Run("collection of 5 numbers", func(t *testing.T) {
		numbers := []int{1,2,3,4,5}
		sum := Sum(numbers)
		want := 15
		if sum != want {
			t.Errorf("got %d, want %d, given %v", sum, want, numbers)
		}
	})
	t.Run("collection of any size", func(t *testing.T) {
		numbers := []int{1,2,3}
		sum := Sum(numbers)
		want := 6
		if sum != want {
			t.Errorf("got %d, want %d, given %v", sum, want, numbers)
		}
	})
}

func TestSumAll(t *testing.T) {
	got := SumAll([]int{1,2}, []int{0,9})
	want := []int{3,9}

	if !slices.Equal(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
}

func TestSumAllTails(t *testing.T) {
	got := SumALlTails([]int{1,2}, []int{0,9})
	want := []int{2,9}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
}
