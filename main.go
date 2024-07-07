package main

import (
	"fmt"
	"math/rand"
	"time"

	hook "github.com/robotn/gohook"
)

func main() {
	/* ︵‿︵‿︵‿୨♡୧‿︵‿︵‿︵ */
	delayLowBound := 30
	delayHighBound := 300
	/* ⋅.˳˳˳.⋅ॱ˙˙ॱ⋅.˳˳˳˳.⋅ॱ˙˙ॱᐧ.˳˳˳.⋅ */

	// > random
	r := rand.New(rand.NewSource(time.Now().UnixNano()))
	randint := func() int { return r.Intn(delayHighBound-delayLowBound+1) + delayLowBound }
	// \\

	// > event listener
	hook.Register(hook.KeyDown, []string{"q", "ctrl", "shift"}, func(e hook.Event) {
		fmt.Println("ctrl-shift-q")
		hook.End()
	})
	// \\

	for {
		fmt.Printf("clicking %d\n", randint())
		time.Sleep(time.Millisecond * time.Duration(randint()))
	}
}
