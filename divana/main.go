package main

import (
	"log"

	"github.com/cprkv/divana/divana/config"
	"github.com/cprkv/divana/divana/sound"
	"github.com/cprkv/divana/divana/ui"
)

var (
	cfg *config.Data
)

func main() {
	defer ui.HandleMainRecover("Divana")

	cfg = config.Load()

	sound.ContextInit()
	defer sound.ContextDestroy()

	playback := sound.EnumeratePlaybackDevices()
	capture := sound.EnumerateCaptureDevices()

	log.Println("playback devices:")
	for _, dev := range playback {
		log.Printf("  %s", dev.ToString())
	}

	log.Println("capture devices:")
	for _, dev := range capture {
		log.Printf("  %s", dev.ToString())
	}

	ui.Example()
}
