package space

type Planet string

// earthYearSeconds seconds of one earth year
var earthYearSeconds float64 = 31557600

// Age calculate how old someone would be on `planet`
func Age(seconds float64, planet Planet) float64 {
	var ratio float64
	switch planet {
	case "Earth":
		ratio = 1
	case "Mercury":
		ratio = 0.2408467
	case "Venus":
		ratio = 0.61519726
	case "Mars":
		ratio = 1.8808158
	case "Jupiter":
		ratio = 11.862615
	case "Saturn":
		ratio = 29.447498
	case "Uranus":
		ratio = 84.016846
	case "Neptune":
		ratio = 164.79132
	}
	return seconds / (ratio * earthYearSeconds)
}
