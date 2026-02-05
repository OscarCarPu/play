package quii

import (
	"fmt"
	"net/http"
	"time"
)

func Racer(aUrl, bUrl string) (winner string, error error) {
	return ConfigurableRacer(aUrl, bUrl, 10*time.Second)
}

func ConfigurableRacer(aUrl, bUrl string, timeout time.Duration) (winner string, error error) {
	select {
	case <-ping(aUrl):
		return aUrl, nil
	case <-ping(bUrl):
		return bUrl, nil
	case <-time.After(timeout):
		return "", fmt.Errorf("timed out waiting for %s and %s", aUrl, bUrl)
	}
}

func ping(url string) chan struct{} {
	ch := make(chan struct{})
	go func() {
		http.Get(url)
		close(ch)
	}()
	return ch
}
