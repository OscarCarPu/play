package clockface

import (
	"math"
	"time"
)

// A Point represents a two dimensional Cartesian coordinate.
type Point struct {
	X float64
	Y float64
}

func secondsInRadians(t time.Time) float64 {
	return (math.Pi / (30 / (float64(t.Second()))))
}

func minutesInRadis(t time.Time) float64 {
	return (secondsInRadians(t) / 60) + (math.Pi / (30 / (float64(t.Minute()))))
}

func secondHandPoint(t time.Time) Point {
	angle := secondsInRadians(t)
	x := math.Sin(angle)
	y := math.Cos(angle)

	return Point{x, y}
}

func minuteHandPoint(t time.Time) Point {
	angle := minutesInRadis(t)
	x := math.Sin(angle)
	y := math.Cos(angle)

	return Point{x, y}
}
